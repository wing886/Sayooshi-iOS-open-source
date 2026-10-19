package com.chino.siglus;

import android.content.Context;
import android.net.Uri;
import android.os.Environment;
import android.provider.DocumentsContract;

import androidx.annotation.Nullable;

import org.json.JSONArray;
import org.json.JSONObject;

import java.io.ByteArrayOutputStream;
import java.io.File;
import java.io.FileInputStream;
import java.io.FileOutputStream;
import java.io.InputStream;
import java.io.OutputStream;
import java.nio.charset.StandardCharsets;
import java.util.ArrayList;
import java.util.Collections;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.UUID;

/**
 * The launcher library: games found by scanning imported folders.
 *
 * Games are never copied. A folder picked in the system picker is mapped to its
 * filesystem path (no-copy mode) and scanned by the native launcher, which
 * detects every SiglusEngine, RealLive, AVG32 and UK2 game below it, their
 * titles, cover artwork (or the exe icon) and text encodings.
 */
public final class GameLibrary {

    /** How deep below a picked folder games are searched for. */
    public static final int SCAN_DEPTH = 3;

    private final Context ctx;
    private final File libraryFile;
    private final File coverDir;

    public GameLibrary(Context ctx) {
        this.ctx = ctx.getApplicationContext();
        File base = new File(this.ctx.getFilesDir(), "SiglusLauncher");
        //noinspection ResultOfMethodCallIgnored
        base.mkdirs();
        this.libraryFile = new File(base, "library.json");
        this.coverDir = new File(base, "covers");
        //noinspection ResultOfMethodCallIgnored
        coverDir.mkdirs();
    }

    // ---------------------------------------------------------------------
    // Persistence
    // ---------------------------------------------------------------------

    public synchronized List<GameEntry> load() {
        List<GameEntry> out = new ArrayList<>();
        try {
            if (!libraryFile.isFile()) {
                return out;
            }
            JSONArray arr = new JSONArray(readText(libraryFile));
            for (int i = 0; i < arr.length(); i++) {
                JSONObject o = arr.getJSONObject(i);
                GameEntry e = new GameEntry(
                        o.optString("id", UUID.randomUUID().toString()),
                        o.optString("title", "Untitled"),
                        o.optString("rootPath"),
                        o.optLong("addedAt", 0L),
                        o.has("coverPath") ? o.optString("coverPath") : null
                );
                e.lastPlayedEpochMs = o.optLong("lastPlayed", 0L);
                e.coverKind = o.optString("coverKind", "image");
                e.engine = o.optString("engine", "siglus");
                e.engineName = o.optString("engineName", engineDisplayName(e.engine));
                e.nls = o.has("nls") && !o.isNull("nls") ? o.optString("nls") : null;
                readNlsOptions(o.optJSONArray("nlsOptions"), e.nlsOptions);
                out.add(e);
            }
        } catch (Throwable ignored) {
            // Corrupt library: start over.
        }
        sort(out);
        return out;
    }

    private synchronized void save(List<GameEntry> entries) throws Exception {
        JSONArray arr = new JSONArray();
        for (GameEntry e : entries) {
            JSONObject o = new JSONObject();
            o.put("id", e.id);
            o.put("title", e.title);
            o.put("rootPath", e.rootPath);
            o.put("addedAt", e.addedAtEpochMs);
            o.put("lastPlayed", e.lastPlayedEpochMs);
            if (e.coverPath != null && !e.coverPath.isEmpty()) {
                o.put("coverPath", e.coverPath);
            }
            o.put("coverKind", e.coverKind);
            o.put("engine", e.engine);
            o.put("engineName", e.engineName);
            if (e.nls != null) {
                o.put("nls", e.nls);
            }
            JSONArray opts = new JSONArray();
            for (GameEntry.NlsOption opt : e.nlsOptions) {
                JSONObject j = new JSONObject();
                j.put("id", opt.id);
                j.put("label", opt.label);
                opts.put(j);
            }
            o.put("nlsOptions", opts);
            arr.put(o);
        }
        byte[] data = arr.toString(2).getBytes(StandardCharsets.UTF_8);
        try (OutputStream out = new FileOutputStream(libraryFile, false)) {
            out.write(data);
        }
    }

    private static void sort(List<GameEntry> list) {
        Collections.sort(list, (a, b) -> {
            int c = Long.compare(b.lastPlayedEpochMs, a.lastPlayedEpochMs);
            if (c != 0) return c;
            c = Long.compare(b.addedAtEpochMs, a.addedAtEpochMs);
            if (c != 0) return c;
            return a.title.compareToIgnoreCase(b.title);
        });
    }

    // ---------------------------------------------------------------------
    // Import (single game or a folder containing many games)
    // ---------------------------------------------------------------------

    /** A game found by the scanner that cannot be played. */
    public static final class Unsupported {
        public final String title;
        public final String path;
        public final String engineName;
        public final String reason;

        Unsupported(String title, String path, String engineName, String reason) {
            this.title = title;
            this.path = path;
            this.engineName = engineName;
            this.reason = reason;
        }
    }

    /** The outcome of one import, shown to the user afterwards. */
    public static final class ImportReport {
        public final List<GameEntry> added = new ArrayList<>();
        public final List<GameEntry> updated = new ArrayList<>();
        public final List<Unsupported> unsupported = new ArrayList<>();
        public final List<String> failedFolders = new ArrayList<>();

        public boolean isEmpty() {
            return added.isEmpty() && updated.isEmpty() && unsupported.isEmpty() && failedFolders.isEmpty();
        }
    }

    /** Maps picked SAF trees to paths and imports every game found below them. Blocking. */
    public ImportReport importTrees(List<Uri> treeUris) {
        List<String> paths = new ArrayList<>();
        ImportReport report = new ImportReport();
        for (Uri uri : treeUris) {
            String path = tryResolveDirectRootPath(uri);
            if (path == null || !new File(path).isDirectory()) {
                report.failedFolders.add(uri.getLastPathSegment() != null ? uri.getLastPathSegment() : uri.toString());
                continue;
            }
            paths.add(path);
        }
        importPaths(paths, report);
        return report;
    }

    /** Scans the given folders (each may be a game or contain several) and merges them. Blocking. */
    public synchronized void importPaths(List<String> folders, ImportReport report) {
        List<GameEntry> all = load();
        Map<String, GameEntry> byRoot = new HashMap<>();
        for (GameEntry e : all) {
            byRoot.put(normalize(e.rootPath), e);
        }

        for (String folder : folders) {
            String json = NativeGames.scanJson(folder, SCAN_DEPTH, coverDir.getAbsolutePath());
            JSONArray arr;
            try {
                arr = json != null ? new JSONArray(json) : new JSONArray();
            } catch (Throwable t) {
                arr = new JSONArray();
            }
            if (arr.length() == 0) {
                report.failedFolders.add(new File(folder).getName());
                continue;
            }
            for (int i = 0; i < arr.length(); i++) {
                JSONObject o = arr.optJSONObject(i);
                if (o == null) continue;
                String root = o.optString("root", folder);
                if (!o.optBoolean("supported", false)) {
                    report.unsupported.add(new Unsupported(
                            o.optString("title", new File(root).getName()),
                            root,
                            o.optString("engine_name", "Unknown"),
                            o.optString("unsupported_reason", "Unsupported engine")
                    ));
                    continue;
                }
                GameEntry existing = byRoot.get(normalize(root));
                if (existing != null) {
                    applyProbe(existing, o, false);
                    report.updated.add(existing);
                } else {
                    GameEntry e = new GameEntry(
                            UUID.randomUUID().toString(),
                            o.optString("title", new File(root).getName()),
                            root,
                            System.currentTimeMillis(),
                            null
                    );
                    applyProbe(e, o, true);
                    all.add(e);
                    byRoot.put(normalize(root), e);
                    report.added.add(e);
                }
            }
        }

        try {
            sort(all);
            save(all);
        } catch (Throwable ignored) {
        }
    }

    /** Re-probes every game (cover, title, missing folders). Blocking. Returns games whose folder is gone. */
    public synchronized List<GameEntry> refreshAll() {
        List<GameEntry> all = load();
        List<GameEntry> missing = new ArrayList<>();
        for (GameEntry e : all) {
            if (!new File(e.rootPath).isDirectory()) {
                missing.add(e);
                continue;
            }
            JSONObject o = probe(e.rootPath, e.nls);
            if (o != null && o.optBoolean("supported", false)) {
                applyProbe(e, o, false);
            }
        }
        try {
            save(all);
        } catch (Throwable ignored) {
        }
        return missing;
    }

    /** Selects a text encoding and refreshes the title decoded with it. Blocking. */
    public synchronized void setNls(String gameId, String nls) {
        List<GameEntry> all = load();
        for (GameEntry e : all) {
            if (!e.id.equals(gameId)) continue;
            e.nls = nls;
            JSONObject o = probe(e.rootPath, nls);
            if (o != null) {
                String title = o.optString("title", "");
                if (!title.isEmpty()) e.title = title;
            }
        }
        try {
            save(all);
        } catch (Throwable ignored) {
        }
    }

    public synchronized void markPlayed(String gameId) {
        List<GameEntry> all = load();
        for (GameEntry e : all) {
            if (e.id.equals(gameId)) e.lastPlayedEpochMs = System.currentTimeMillis();
        }
        try {
            save(all);
        } catch (Throwable ignored) {
        }
    }

    public synchronized void remove(String gameId) {
        List<GameEntry> all = load();
        List<GameEntry> keep = new ArrayList<>();
        for (GameEntry e : all) {
            if (e.id.equals(gameId)) {
                if (e.coverPath != null && e.coverPath.startsWith(coverDir.getAbsolutePath())) {
                    //noinspection ResultOfMethodCallIgnored
                    new File(e.coverPath).delete();
                }
            } else {
                keep.add(e);
            }
        }
        try {
            save(keep);
        } catch (Throwable ignored) {
        }
    }

    @Nullable
    private JSONObject probe(String root, @Nullable String nls) {
        try {
            String json = NativeGames.probeJson(root, nls, coverDir.getAbsolutePath());
            return json != null ? new JSONObject(json) : null;
        } catch (Throwable t) {
            return null;
        }
    }

    private static void applyProbe(GameEntry e, JSONObject o, boolean fresh) {
        e.engine = o.optString("engine", e.engine);
        e.engineName = o.optString("engine_name", engineDisplayName(e.engine));
        String cover = o.isNull("cover") ? null : o.optString("cover", null);
        if (cover != null && !cover.isEmpty()) {
            e.coverPath = cover;
            e.coverKind = o.optString("cover_kind", "image");
        }
        e.nlsOptions.clear();
        readNlsOptions(o.optJSONArray("nls_options"), e.nlsOptions);
        if (e.nlsOptions.isEmpty()) {
            e.nls = null;
        } else if (e.nls == null || fresh) {
            e.nls = o.isNull("nls") ? e.nlsOptions.get(0).id : o.optString("nls", e.nlsOptions.get(0).id);
        }
        // Keep a title decoded with a user-selected encoding.
        if (fresh || e.nls == null || e.nls.equals(o.optString("nls", ""))) {
            String title = o.optString("title", "");
            if (!title.isEmpty()) e.title = title;
        }
    }

    private static void readNlsOptions(@Nullable JSONArray arr, List<GameEntry.NlsOption> out) {
        if (arr == null) return;
        for (int i = 0; i < arr.length(); i++) {
            JSONObject j = arr.optJSONObject(i);
            if (j == null) continue;
            out.add(new GameEntry.NlsOption(j.optString("id"), j.optString("label", j.optString("id"))));
        }
    }

    public static String engineDisplayName(String engine) {
        switch (engine) {
            case "reallive": return "RealLive";
            case "avg32": return "AVG32";
            case "uk2": return "UK2";
            case "siglus": return "SiglusEngine";
            default: return engine;
        }
    }

    private static String normalize(String path) {
        try {
            return new File(path).getCanonicalPath();
        } catch (Throwable t) {
            return new File(path).getAbsolutePath();
        }
    }

    private static String readText(File f) throws Exception {
        try (InputStream in = new FileInputStream(f)) {
            ByteArrayOutputStream baos = new ByteArrayOutputStream();
            byte[] buf = new byte[8192];
            int n;
            while ((n = in.read(buf)) >= 0) {
                if (n > 0) baos.write(buf, 0, n);
            }
            return new String(baos.toByteArray(), StandardCharsets.UTF_8);
        }
    }

    /** Maps an external-storage SAF tree to its filesystem path (no-copy mode). */
    @Nullable
    public static String tryResolveDirectRootPath(Uri treeUri) {
        if (treeUri == null) return null;
        if (!"content".equalsIgnoreCase(treeUri.getScheme())) return null;

        try {
            String authority = treeUri.getAuthority();
            if (!"com.android.externalstorage.documents".equals(authority)) {
                return null;
            }

            String docId = DocumentsContract.getTreeDocumentId(treeUri);
            if (docId == null || docId.isEmpty()) return null;

            String[] parts = docId.split(":", 2);
            String volume = parts[0];
            String relative = parts.length > 1 ? parts[1] : "";

            File base;
            if ("primary".equalsIgnoreCase(volume)) {
                base = Environment.getExternalStorageDirectory();
            } else {
                // Common pattern for removable storage volume names.
                base = new File("/storage", volume);
            }

            if (relative.isEmpty()) {
                return base.getAbsolutePath();
            }
            return new File(base, relative).getAbsolutePath();
        } catch (Throwable ignored) {
            return null;
        }
    }
}
