package com.chino.siglus;

import androidx.annotation.Nullable;

import java.util.ArrayList;
import java.util.List;

/** One game in the launcher library. */
public final class GameEntry {
    public static final class NlsOption {
        public final String id;
        public final String label;

        public NlsOption(String id, String label) {
            this.id = id;
            this.label = label;
        }
    }

    public final String id;
    public String title;
    public final String rootPath;
    public final long addedAtEpochMs;
    public long lastPlayedEpochMs;
    @Nullable public String coverPath;
    /** "image" for game artwork, "icon" for an exe/ico icon card. */
    public String coverKind = "image";
    /** siglus / reallive / avg32 / uk2. */
    public String engine = "siglus";
    public String engineName = "SiglusEngine";
    /** Selected text encoding; null for engines without a choice (Siglus). */
    @Nullable public String nls;
    public final List<NlsOption> nlsOptions = new ArrayList<>();

    public GameEntry(String id, String title, String rootPath, long addedAtEpochMs, @Nullable String coverPath) {
        this.id = id;
        this.title = title;
        this.rootPath = rootPath;
        this.addedAtEpochMs = addedAtEpochMs;
        this.coverPath = coverPath;
    }

    public boolean isSiglus() {
        return "siglus".equals(engine);
    }

    @Nullable
    public String nlsLabel() {
        if (nls == null) return null;
        for (NlsOption o : nlsOptions) {
            if (o.id.equals(nls)) return o.label;
        }
        return nls;
    }
}
