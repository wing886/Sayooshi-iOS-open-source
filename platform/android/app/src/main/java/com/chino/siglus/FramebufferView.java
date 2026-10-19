package com.chino.siglus;

import android.content.Context;
import android.graphics.Bitmap;
import android.graphics.Canvas;
import android.graphics.Paint;
import android.graphics.Rect;
import android.util.AttributeSet;
import android.view.View;

import androidx.annotation.Nullable;

/** Draws a game framebuffer letterboxed and upscaled with crisp pixels. */
public final class FramebufferView extends View {
    private final Paint paint = new Paint();
    private final Rect src = new Rect();
    private final Rect dst = new Rect();
    @Nullable private Bitmap frame;

    public FramebufferView(Context context) {
        this(context, null);
    }

    public FramebufferView(Context context, @Nullable AttributeSet attrs) {
        super(context, attrs);
        paint.setFilterBitmap(false);
        paint.setAntiAlias(false);
        paint.setDither(false);
    }

    public void setFrame(@Nullable Bitmap bitmap) {
        frame = bitmap;
        invalidate();
    }

    /** The on-screen rectangle the frame occupies. */
    public Rect frameRect() {
        layoutFrame();
        return dst;
    }

    private void layoutFrame() {
        int vw = getWidth();
        int vh = getHeight();
        if (frame == null || vw <= 0 || vh <= 0) {
            dst.set(0, 0, vw, vh);
            return;
        }
        int fw = frame.getWidth();
        int fh = frame.getHeight();
        src.set(0, 0, fw, fh);
        float scale = Math.min(vw / (float) fw, vh / (float) fh);
        // Prefer an integer scale when it still fills most of the screen.
        float integer = (float) Math.floor(scale);
        if (integer >= 1f && integer / scale > 0.92f) {
            scale = integer;
        }
        int w = Math.round(fw * scale);
        int h = Math.round(fh * scale);
        int x = (vw - w) / 2;
        int y = (vh - h) / 2;
        dst.set(x, y, x + w, y + h);
    }

    /** Converts a view coordinate to a frame pixel, clamped to the frame. */
    public int[] toFrame(float x, float y) {
        if (frame == null) return new int[] {0, 0};
        layoutFrame();
        float fx = (x - dst.left) * frame.getWidth() / (float) Math.max(1, dst.width());
        float fy = (y - dst.top) * frame.getHeight() / (float) Math.max(1, dst.height());
        int ix = Math.max(0, Math.min(frame.getWidth() - 1, (int) fx));
        int iy = Math.max(0, Math.min(frame.getHeight() - 1, (int) fy));
        return new int[] {ix, iy};
    }

    @Override
    protected void onDraw(Canvas canvas) {
        canvas.drawColor(0xFF000000);
        if (frame == null) return;
        layoutFrame();
        canvas.drawBitmap(frame, src, dst, paint);
    }
}
