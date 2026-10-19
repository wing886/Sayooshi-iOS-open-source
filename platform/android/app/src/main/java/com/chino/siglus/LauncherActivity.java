package com.chino.siglus;

import android.content.Intent;
import android.net.Uri;
import android.os.Build;
import android.os.Bundle;
import android.os.Environment;
import android.provider.Settings;
import android.view.View;
import android.widget.Button;
import android.widget.TextView;
import android.widget.Toast;

import androidx.activity.result.ActivityResultLauncher;
import androidx.activity.result.contract.ActivityResultContracts;
import androidx.annotation.Nullable;
import androidx.appcompat.app.AlertDialog;
import androidx.appcompat.app.AppCompatActivity;
import androidx.recyclerview.widget.GridLayoutManager;
import androidx.recyclerview.widget.RecyclerView;

import java.util.ArrayList;
import java.util.Collections;
import java.util.List;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;

/**
 * Launcher UI:
 * - Shows the library as a grid of cover tiles with the detected engine.
 * - Import: pick a folder that is one game or contains many games (batch);
 *   every game below it is detected, unsupported ones are reported afterwards.
 * - Run: SiglusEngine games start the Siglus player; RealLive, AVG32 and UK2
 *   games start the framebuffer player with the selected text encoding.
 */
public final class LauncherActivity extends AppCompatActivity implements GameAdapter.Listener {

    private GameLibrary library;
    private GameAdapter adapter;
    private View emptyView;
    private View busyView;
    private TextView busyText;
    private GridLayoutManager layoutManager;

    private final ExecutorService worker = Executors.newSingleThreadExecutor();

    private final ActivityResultLauncher<Uri> openTreeLauncher =
            registerForActivityResult(new ActivityResultContracts.OpenDocumentTree(), this::onImportTreeSelected);

    @Override
    protected void onCreate(@Nullable Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);

        setContentView(R.layout.activity_launcher);

        library = new GameLibrary(this);

        RecyclerView rv = findViewById(R.id.game_grid);
        layoutManager = new GridLayoutManager(this, spanCount());
        rv.setLayoutManager(layoutManager);
        adapter = new GameAdapter(this);
        rv.setAdapter(adapter);

        emptyView = findViewById(R.id.empty_view);
        busyView = findViewById(R.id.busy_view);
        busyText = findViewById(R.id.busy_text);

        Button importBtn = findViewById(R.id.btn_import);
        importBtn.setOnClickListener(v -> startImportFlow());
        Button refreshBtn = findViewById(R.id.btn_refresh);
        refreshBtn.setOnClickListener(v -> refreshAll());
        emptyView.setOnClickListener(v -> startImportFlow());

        refresh();
    }

    @Override
    protected void onResume() {
        super.onResume();
        if (layoutManager != null) layoutManager.setSpanCount(spanCount());
        refresh();
    }

    @Override
    protected void onDestroy() {
        worker.shutdown();
        super.onDestroy();
    }

    private int spanCount() {
        float dp = getResources().getDisplayMetrics().widthPixels / getResources().getDisplayMetrics().density;
        return Math.max(2, (int) (dp / 190f));
    }

    private void refresh() {
        List<GameEntry> entries = library.load();
        adapter.setItems(entries);
        emptyView.setVisibility(entries.isEmpty() ? View.VISIBLE : View.GONE);
    }

    private void setBusy(@Nullable String text) {
        if (text == null) {
            busyView.setVisibility(View.GONE);
        } else {
            busyText.setText(text);
            busyView.setVisibility(View.VISIBLE);
        }
    }

    // ------------------------------------------------------------------
    // Import
    // ------------------------------------------------------------------

    private void startImportFlow() {
        if (!hasAllFilesAccess()) {
            showAllFilesAccessPrompt();
            return;
        }
        openTreeLauncher.launch(null);
    }

    private boolean hasAllFilesAccess() {
        // MANAGE_EXTERNAL_STORAGE applies on Android 11+.
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.R) {
            return Environment.isExternalStorageManager();
        }
        return true;
    }

    private void showAllFilesAccessPrompt() {
        new AlertDialog.Builder(this)
                .setTitle("Grant Storage Access")
                .setMessage("Games are played in place without copying, which requires 'All files access'. Please grant it, then import again.")
                .setPositiveButton("Open Settings", (d, w) -> {
                    try {
                        Intent i = new Intent(Settings.ACTION_MANAGE_APP_ALL_FILES_ACCESS_PERMISSION);
                        i.setData(Uri.parse("package:" + getPackageName()));
                        startActivity(i);
                    } catch (Throwable t) {
                        startActivity(new Intent(Settings.ACTION_MANAGE_ALL_FILES_ACCESS_PERMISSION));
                    }
                })
                .setNegativeButton("Cancel", null)
                .show();
    }

    private void onImportTreeSelected(@Nullable Uri treeUri) {
        if (treeUri == null) {
            return;
        }
        try {
            getContentResolver().takePersistableUriPermission(treeUri, Intent.FLAG_GRANT_READ_URI_PERMISSION);
        } catch (Throwable ignored) {
            // Direct-path import still works without a persisted grant.
        }

        setBusy(getString(R.string.scanning));
        worker.execute(() -> {
            GameLibrary.ImportReport report = library.importTrees(Collections.singletonList(treeUri));
            runOnUiThread(() -> {
                setBusy(null);
                refresh();
                showImportReport(report);
            });
        });
    }

    private void showImportReport(GameLibrary.ImportReport report) {
        StringBuilder msg = new StringBuilder();
        String title;
        if (!report.added.isEmpty()) {
            title = report.added.size() == 1 ? "Imported 1 game" : "Imported " + report.added.size() + " games";
            for (GameEntry e : report.added) {
                msg.append("• ").append(e.title).append("  (").append(e.engineName).append(")\n");
            }
        } else if (!report.updated.isEmpty()) {
            title = "Already in library";
            for (GameEntry e : report.updated) {
                msg.append("• ").append(e.title).append('\n');
            }
        } else if (!report.unsupported.isEmpty()) {
            title = "Not supported";
        } else {
            title = "No games found";
            msg.append("No SiglusEngine, RealLive, AVG32 or UK2 game was found in the selected folder.\n");
        }

        if (!report.unsupported.isEmpty()) {
            if (msg.length() > 0) msg.append('\n');
            msg.append(report.unsupported.size() == 1
                    ? "1 game can't be played:\n"
                    : report.unsupported.size() + " games can't be played:\n");
            for (GameLibrary.Unsupported u : report.unsupported) {
                msg.append("• ").append(u.title).append(" — ").append(u.reason).append('\n');
            }
        }
        if (!report.failedFolders.isEmpty() && (!report.added.isEmpty() || !report.unsupported.isEmpty())) {
            msg.append("\nNothing found in: ").append(String.join(", ", report.failedFolders)).append('\n');
        } else if (!report.failedFolders.isEmpty() && report.updated.isEmpty()) {
            msg.append("\nOnly folders on local storage can be imported without copying.\n");
        }

        new AlertDialog.Builder(this)
                .setTitle(title)
                .setMessage(msg.toString().trim())
                .setPositiveButton("OK", null)
                .setNeutralButton("Import More", (d, w) -> startImportFlow())
                .show();
    }

    private void refreshAll() {
        setBusy("Refreshing…");
        worker.execute(() -> {
            List<GameEntry> missing = library.refreshAll();
            runOnUiThread(() -> {
                setBusy(null);
                refresh();
                if (!missing.isEmpty()) {
                    List<String> names = new ArrayList<>();
                    for (GameEntry e : missing) names.add(e.title);
                    new AlertDialog.Builder(this)
                            .setTitle("Missing games")
                            .setMessage("These folders can no longer be found:\n• " + String.join("\n• ", names))
                            .setPositiveButton("Remove", (d, w) -> {
                                for (GameEntry e : missing) library.remove(e.id);
                                refresh();
                            })
                            .setNegativeButton("Keep", null)
                            .show();
                }
            });
        });
    }

    // ------------------------------------------------------------------
    // Launch
    // ------------------------------------------------------------------

    @Override
    public void onGameClicked(GameEntry e) {
        try {
            library.markPlayed(e.id);
            if (e.isSiglus()) {
                // The Siglus player reads its game root from launch.json.
                LaunchConfig.write(this, e.rootPath);
                startActivity(new Intent(this, SiglusGameActivity.class));
            } else {
                Intent it = new Intent(this, FramebufferGameActivity.class);
                it.putExtra(FramebufferGameActivity.EXTRA_ROOT, e.rootPath);
                it.putExtra(FramebufferGameActivity.EXTRA_ENGINE, e.engine);
                it.putExtra(FramebufferGameActivity.EXTRA_NLS, e.nls);
                it.putExtra(FramebufferGameActivity.EXTRA_TITLE, e.title);
                startActivity(it);
            }
        } catch (Throwable t) {
            Toast.makeText(this, "Failed to start: " + t.getMessage(), Toast.LENGTH_LONG).show();
        }
    }

    @Override
    public void onGameLongPressed(GameEntry e) {
        if (e == null) return;

        List<String> actions = new ArrayList<>();
        actions.add("Play");
        if (!e.nlsOptions.isEmpty()) {
            actions.add("Text Encoding (" + e.nlsLabel() + ")");
        }
        actions.add("Remove from Library");

        new AlertDialog.Builder(this)
                .setTitle(e.title)
                .setMessage(e.engineName + "\n" + e.rootPath)
                .setItems(actions.toArray(new String[0]), (d, which) -> {
                    String action = actions.get(which);
                    if (action.equals("Play")) {
                        onGameClicked(e);
                    } else if (action.startsWith("Text Encoding")) {
                        chooseNls(e);
                    } else {
                        confirmRemove(e);
                    }
                })
                .setNegativeButton("Cancel", null)
                .show();
    }

    private void chooseNls(GameEntry e) {
        String[] labels = new String[e.nlsOptions.size()];
        int checked = 0;
        for (int i = 0; i < labels.length; i++) {
            GameEntry.NlsOption o = e.nlsOptions.get(i);
            labels[i] = o.label;
            if (o.id.equals(e.nls)) checked = i;
        }
        new AlertDialog.Builder(this)
                .setTitle("Text Encoding")
                .setSingleChoiceItems(labels, checked, (d, which) -> {
                    d.dismiss();
                    String nls = e.nlsOptions.get(which).id;
                    worker.execute(() -> {
                        library.setNls(e.id, nls);
                        runOnUiThread(this::refresh);
                    });
                })
                .setNegativeButton("Cancel", null)
                .show();
    }

    private void confirmRemove(GameEntry e) {
        new AlertDialog.Builder(this)
                .setTitle("Remove " + e.title + "?")
                .setMessage("The game files are not deleted.")
                .setPositiveButton("Remove", (d, w) -> {
                    library.remove(e.id);
                    refresh();
                })
                .setNegativeButton("Cancel", null)
                .show();
    }
}
