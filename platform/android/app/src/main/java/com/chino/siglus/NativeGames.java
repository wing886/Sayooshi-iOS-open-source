package com.chino.siglus;

import java.nio.ByteBuffer;

/**
 * JNI bridge to the multi-engine launcher API in libsiglus
 * (game_launcher: probe / scan / framebuffer runtime for RealLive, AVG32 and UK2).
 */
public final class NativeGames {
    static {
        System.loadLibrary("siglus");
        System.loadLibrary("siglus_jni");
    }

    private NativeGames() {}

    // Key codes understood by fbKey (see siglus.h).
    public static final int KEY_ENTER = 1;
    public static final int KEY_ESCAPE = 2;
    public static final int KEY_SPACE = 3;
    public static final int KEY_UP = 4;
    public static final int KEY_DOWN = 5;
    public static final int KEY_LEFT = 6;
    public static final int KEY_RIGHT = 7;
    public static final int KEY_PAGE_UP = 8;
    public static final int KEY_PAGE_DOWN = 9;
    public static final int KEY_HOME = 10;
    public static final int KEY_END = 11;
    public static final int KEY_BACKSPACE = 12;
    public static final int KEY_TAB = 13;
    public static final int KEY_CTRL = 14;
    public static final int KEY_SHIFT = 15;
    public static final int KEY_F_BASE = 0x100;

    public static final int BUTTON_LEFT = 0;
    public static final int BUTTON_RIGHT = 1;

    /** Scans a folder (and sub folders up to depth) for games; returns a JSON array of probe results. */
    public static native String scanJson(String path, int depth, String coverCacheDir);

    /** Probes a single game root; returns one JSON object (or null). */
    public static native String probeJson(String root, String nls, String coverCacheDir);

    /** Registers an extra font file for game text. Returns 0 on success. */
    public static native int addFontFile(String path);

    /** Opens a non-Siglus game. Returns 0 on failure with the message in errorOut[0]. */
    public static native long fbOpen(String root, String engine, String nls, String[] errorOut);

    /** Advances the game by dtMs. Returns 0 while running and 1 once the game has ended. */
    public static native int fbStep(long handle, int dtMs);

    /** Returns {width, height} of the current frame, or null. */
    public static native int[] fbFrameSize(long handle);

    /** Copies the RGBA frame into a direct buffer of at least width*height*4 bytes. */
    public static native boolean fbCopyFrame(long handle, ByteBuffer directBuffer);

    public static native void fbPointerMove(long handle, int x, int y);
    public static native void fbPointerButton(long handle, int button, boolean pressed);
    public static native void fbWheel(long handle, boolean up);
    public static native void fbKey(long handle, int code, boolean pressed);
    public static native void fbText(long handle, String text);
    public static native void fbClose(long handle);
}
