package com.chino.siglus;

import android.graphics.Bitmap;
import android.os.Bundle;
import android.os.Handler;
import android.os.Looper;
import android.view.Choreographer;
import android.view.KeyEvent;
import android.view.MotionEvent;
import android.view.View;
import android.view.WindowManager;
import android.widget.TextView;
import android.widget.Toast;

import androidx.activity.OnBackPressedCallback;
import androidx.annotation.Nullable;
import androidx.appcompat.app.AlertDialog;
import androidx.appcompat.app.AppCompatActivity;
import androidx.core.view.WindowCompat;
import androidx.core.view.WindowInsetsCompat;
import androidx.core.view.WindowInsetsControllerCompat;

import java.nio.ByteBuffer;
import java.nio.ByteOrder;

/**
 * Hosts RealLive, AVG32 and UK2 games: the native runtime renders into a
 * framebuffer which is copied into a Bitmap every display frame.
 *
 * Touch: tap = left click, two-finger tap = right click (menu / cancel),
 * two-finger vertical swipe = mouse wheel. Back = right click.
 */
public final class FramebufferGameActivity extends AppCompatActivity implements Choreographer.FrameCallback {
    public static final String EXTRA_ROOT = "root";
    public static final String EXTRA_ENGINE = "engine";
    public static final String EXTRA_NLS = "nls";
    public static final String EXTRA_TITLE = "title";

    private static final long LEFT_PRESS_DELAY_MS = 90;
    private static final float WHEEL_STEP_PX = 48f;

    private final Handler handler = new Handler(Looper.getMainLooper());

    private FramebufferView view;
    private View loading;
    private TextView skipButton;

    private long handle;
    private boolean running;
    private long lastFrameNanos;
    private boolean skipping;

    @Nullable private Bitmap bitmap;
    @Nullable private ByteBuffer buffer;
    private int frameW;
    private int frameH;

    // Touch gesture state.
    private boolean multiTouch;
    private boolean leftPressed;
    private boolean leftPressPending;
    private float wheelAnchorY;
    private float wheelAccum;
    private boolean wheelUsed;
    private final Runnable sendLeftPress = () -> {
        if (leftPressPending && handle != 0) {
            leftPressPending = false;
            leftPressed = true;
            NativeGames.fbPointerButton(handle, NativeGames.BUTTON_LEFT, true);
        }
    };

    @Override
    protected void onCreate(@Nullable Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        getWindow().addFlags(WindowManager.LayoutParams.FLAG_KEEP_SCREEN_ON);
        setContentView(R.layout.activity_framebuffer);
        enterImmersive();

        view = findViewById(R.id.fb_view);
        loading = findViewById(R.id.fb_loading);
        skipButton = findViewById(R.id.btn_skip);
        TextView loadingText = findViewById(R.id.fb_loading_text);

        String title = getIntent().getStringExtra(EXTRA_TITLE);
        loadingText.setText(title != null ? title : "");

        findViewById(R.id.btn_exit).setOnClickListener(v -> confirmExit());
        findViewById(R.id.btn_menu).setOnClickListener(v -> rightClick());
        skipButton.setOnClickListener(v -> toggleSkip());
        view.setOnTouchListener((v, ev) -> onGameTouch(ev));

        getOnBackPressedDispatcher().addCallback(this, new OnBackPressedCallback(true) {
            @Override
            public void handleOnBackPressed() {
                if (handle != 0) {
                    rightClick();
                } else {
                    finish();
                }
            }
        });

        NativeSiglus.initAndroidContext(this);
        // Let the loading overlay draw before the (blocking) open.
        view.post(this::openGame);
    }

    private void openGame() {
        String root = getIntent().getStringExtra(EXTRA_ROOT);
        String engine = getIntent().getStringExtra(EXTRA_ENGINE);
        String nls = getIntent().getStringExtra(EXTRA_NLS);
        String[] err = new String[1];
        long h = 0;
        try {
            h = NativeGames.fbOpen(root, engine, nls, err);
        } catch (Throwable t) {
            err[0] = t.getMessage();
        }
        if (h == 0) {
            String msg = err[0] != null ? err[0] : "Unknown error";
            new AlertDialog.Builder(this)
                    .setTitle("Failed to start")
                    .setMessage(msg)
                    .setPositiveButton("OK", (d, w) -> finish())
                    .setOnCancelListener(d -> finish())
                    .show();
            return;
        }
        handle = h;
        loading.setVisibility(View.GONE);
        startLoop();
    }

    private void startLoop() {
        if (running || handle == 0) return;
        running = true;
        lastFrameNanos = 0;
        Choreographer.getInstance().postFrameCallback(this);
    }

    private void stopLoop() {
        running = false;
        Choreographer.getInstance().removeFrameCallback(this);
    }

    @Override
    public void doFrame(long frameTimeNanos) {
        if (!running || handle == 0) return;
        int dt = lastFrameNanos == 0 ? 16 : (int) ((frameTimeNanos - lastFrameNanos) / 1_000_000L);
        lastFrameNanos = frameTimeNanos;
        dt = Math.max(0, Math.min(100, dt));

        int status = NativeGames.fbStep(handle, dt);
        if (status != 0) {
            closeGame();
            finish();
            return;
        }
        presentFrame();
        Choreographer.getInstance().postFrameCallback(this);
    }

    private void presentFrame() {
        int[] size = NativeGames.fbFrameSize(handle);
        if (size == null || size[0] <= 0 || size[1] <= 0) return;
        if (bitmap == null || size[0] != frameW || size[1] != frameH) {
            frameW = size[0];
            frameH = size[1];
            bitmap = Bitmap.createBitmap(frameW, frameH, Bitmap.Config.ARGB_8888);
            buffer = ByteBuffer.allocateDirect(frameW * frameH * 4).order(ByteOrder.nativeOrder());
        }
        buffer.rewind();
        if (NativeGames.fbCopyFrame(handle, buffer)) {
            buffer.rewind();
            // ARGB_8888 stores bytes as R, G, B, A: the same order as the RGBA frame.
            bitmap.copyPixelsFromBuffer(buffer);
            view.setFrame(bitmap);
        }
    }

    // ------------------------------------------------------------------
    // Input
    // ------------------------------------------------------------------

    private boolean onGameTouch(MotionEvent ev) {
        if (handle == 0) return true;
        switch (ev.getActionMasked()) {
            case MotionEvent.ACTION_DOWN: {
                multiTouch = false;
                wheelUsed = false;
                movePointer(ev.getX(), ev.getY());
                leftPressPending = true;
                handler.postDelayed(sendLeftPress, LEFT_PRESS_DELAY_MS);
                break;
            }
            case MotionEvent.ACTION_POINTER_DOWN: {
                if (!multiTouch) {
                    multiTouch = true;
                    handler.removeCallbacks(sendLeftPress);
                    leftPressPending = false;
                    wheelAnchorY = averageY(ev);
                    wheelAccum = 0;
                }
                break;
            }
            case MotionEvent.ACTION_MOVE: {
                if (multiTouch) {
                    float y = averageY(ev);
                    wheelAccum += y - wheelAnchorY;
                    wheelAnchorY = y;
                    while (Math.abs(wheelAccum) >= WHEEL_STEP_PX) {
                        boolean up = wheelAccum > 0;
                        NativeGames.fbWheel(handle, up);
                        wheelAccum += up ? -WHEEL_STEP_PX : WHEEL_STEP_PX;
                        wheelUsed = true;
                    }
                } else {
                    movePointer(ev.getX(), ev.getY());
                }
                break;
            }
            case MotionEvent.ACTION_UP: {
                if (multiTouch) {
                    if (leftPressed) {
                        leftPressed = false;
                        NativeGames.fbPointerButton(handle, NativeGames.BUTTON_LEFT, false);
                    }
                    if (!wheelUsed) {
                        rightClick();
                    }
                } else {
                    movePointer(ev.getX(), ev.getY());
                    if (leftPressPending) {
                        handler.removeCallbacks(sendLeftPress);
                        leftPressPending = false;
                        NativeGames.fbPointerButton(handle, NativeGames.BUTTON_LEFT, true);
                    }
                    leftPressed = false;
                    NativeGames.fbPointerButton(handle, NativeGames.BUTTON_LEFT, false);
                }
                multiTouch = false;
                break;
            }
            case MotionEvent.ACTION_CANCEL: {
                handler.removeCallbacks(sendLeftPress);
                leftPressPending = false;
                if (leftPressed) {
                    leftPressed = false;
                    NativeGames.fbPointerButton(handle, NativeGames.BUTTON_LEFT, false);
                }
                multiTouch = false;
                break;
            }
            default:
                break;
        }
        return true;
    }

    private static float averageY(MotionEvent ev) {
        float sum = 0;
        for (int i = 0; i < ev.getPointerCount(); i++) sum += ev.getY(i);
        return sum / Math.max(1, ev.getPointerCount());
    }

    private void movePointer(float x, float y) {
        int[] p = view.toFrame(x, y);
        NativeGames.fbPointerMove(handle, p[0], p[1]);
    }

    private void rightClick() {
        if (handle == 0) return;
        NativeGames.fbPointerButton(handle, NativeGames.BUTTON_RIGHT, true);
        NativeGames.fbPointerButton(handle, NativeGames.BUTTON_RIGHT, false);
    }

    private void toggleSkip() {
        if (handle == 0) return;
        skipping = !skipping;
        NativeGames.fbKey(handle, NativeGames.KEY_CTRL, skipping);
        skipButton.setAlpha(skipping ? 1f : 0.6f);
        Toast.makeText(this, skipping ? "Skip on" : "Skip off", Toast.LENGTH_SHORT).show();
    }

    private static int mapKey(int keyCode) {
        switch (keyCode) {
            case KeyEvent.KEYCODE_ENTER:
            case KeyEvent.KEYCODE_NUMPAD_ENTER:
            case KeyEvent.KEYCODE_DPAD_CENTER:
            case KeyEvent.KEYCODE_BUTTON_A:
                return NativeGames.KEY_ENTER;
            case KeyEvent.KEYCODE_ESCAPE:
            case KeyEvent.KEYCODE_BUTTON_B:
                return NativeGames.KEY_ESCAPE;
            case KeyEvent.KEYCODE_SPACE: return NativeGames.KEY_SPACE;
            case KeyEvent.KEYCODE_DPAD_UP: return NativeGames.KEY_UP;
            case KeyEvent.KEYCODE_DPAD_DOWN: return NativeGames.KEY_DOWN;
            case KeyEvent.KEYCODE_DPAD_LEFT: return NativeGames.KEY_LEFT;
            case KeyEvent.KEYCODE_DPAD_RIGHT: return NativeGames.KEY_RIGHT;
            case KeyEvent.KEYCODE_PAGE_UP: return NativeGames.KEY_PAGE_UP;
            case KeyEvent.KEYCODE_PAGE_DOWN: return NativeGames.KEY_PAGE_DOWN;
            case KeyEvent.KEYCODE_MOVE_HOME: return NativeGames.KEY_HOME;
            case KeyEvent.KEYCODE_MOVE_END: return NativeGames.KEY_END;
            case KeyEvent.KEYCODE_DEL: return NativeGames.KEY_BACKSPACE;
            case KeyEvent.KEYCODE_TAB: return NativeGames.KEY_TAB;
            case KeyEvent.KEYCODE_CTRL_LEFT:
            case KeyEvent.KEYCODE_CTRL_RIGHT:
                return NativeGames.KEY_CTRL;
            case KeyEvent.KEYCODE_SHIFT_LEFT:
            case KeyEvent.KEYCODE_SHIFT_RIGHT:
                return NativeGames.KEY_SHIFT;
            default:
                if (keyCode >= KeyEvent.KEYCODE_F1 && keyCode <= KeyEvent.KEYCODE_F12) {
                    return NativeGames.KEY_F_BASE + (keyCode - KeyEvent.KEYCODE_F1 + 1);
                }
                return 0;
        }
    }

    @Override
    public boolean onKeyDown(int keyCode, KeyEvent event) {
        int code = mapKey(keyCode);
        if (handle != 0 && code != 0) {
            if (event.getRepeatCount() == 0 || code <= NativeGames.KEY_PAGE_DOWN) {
                NativeGames.fbKey(handle, code, true);
            }
            return true;
        }
        if (handle != 0 && event.getUnicodeChar() > 0 && !event.isCtrlPressed()) {
            NativeGames.fbText(handle, String.valueOf((char) event.getUnicodeChar()));
            return true;
        }
        return super.onKeyDown(keyCode, event);
    }

    @Override
    public boolean onKeyUp(int keyCode, KeyEvent event) {
        int code = mapKey(keyCode);
        if (handle != 0 && code != 0) {
            NativeGames.fbKey(handle, code, false);
            return true;
        }
        return super.onKeyUp(keyCode, event);
    }

    // ------------------------------------------------------------------
    // Lifecycle
    // ------------------------------------------------------------------

    private void confirmExit() {
        new AlertDialog.Builder(this)
                .setTitle("Quit game?")
                .setMessage("Unsaved progress will be lost.")
                .setPositiveButton("Quit", (d, w) -> {
                    closeGame();
                    finish();
                })
                .setNegativeButton("Cancel", null)
                .show();
    }

    private void closeGame() {
        stopLoop();
        handler.removeCallbacks(sendLeftPress);
        if (handle != 0) {
            long h = handle;
            handle = 0;
            NativeGames.fbClose(h);
        }
    }

    private void enterImmersive() {
        WindowCompat.setDecorFitsSystemWindows(getWindow(), false);
        WindowInsetsControllerCompat c = WindowCompat.getInsetsController(getWindow(), getWindow().getDecorView());
        c.hide(WindowInsetsCompat.Type.systemBars());
        c.setSystemBarsBehavior(WindowInsetsControllerCompat.BEHAVIOR_SHOW_TRANSIENT_BARS_BY_SWIPE);
    }

    @Override
    public void onWindowFocusChanged(boolean hasFocus) {
        super.onWindowFocusChanged(hasFocus);
        if (hasFocus) enterImmersive();
    }

    @Override
    protected void onResume() {
        super.onResume();
        startLoop();
    }

    @Override
    protected void onPause() {
        stopLoop();
        super.onPause();
    }

    @Override
    protected void onDestroy() {
        closeGame();
        super.onDestroy();
    }
}
