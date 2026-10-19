package com.chino.siglus;

import android.graphics.Bitmap;
import android.graphics.BitmapFactory;
import android.graphics.drawable.BitmapDrawable;
import android.graphics.drawable.GradientDrawable;
import android.util.LruCache;
import android.view.LayoutInflater;
import android.view.View;
import android.view.ViewGroup;
import android.widget.ImageView;
import android.widget.TextView;

import androidx.annotation.NonNull;
import androidx.annotation.Nullable;
import androidx.recyclerview.widget.RecyclerView;

import java.io.File;
import java.util.ArrayList;
import java.util.List;

public final class GameAdapter extends RecyclerView.Adapter<GameAdapter.Holder> {

    public interface Listener {
        void onGameClicked(GameEntry e);
        void onGameLongPressed(GameEntry e);
    }

    private final Listener listener;
    private final List<GameEntry> items = new ArrayList<>();
    private final LruCache<String, Bitmap> covers = new LruCache<String, Bitmap>(24 * 1024 * 1024) {
        @Override
        protected int sizeOf(String key, Bitmap value) {
            return value.getByteCount();
        }
    };

    public GameAdapter(Listener listener) {
        this.listener = listener;
    }

    public void setItems(List<GameEntry> newItems) {
        items.clear();
        if (newItems != null) {
            items.addAll(newItems);
        }
        covers.evictAll();
        notifyDataSetChanged();
    }

    public static int engineColor(String engine) {
        switch (engine) {
            case "siglus": return 0xFF5973F2;
            case "reallive": return 0xFFE57340;
            case "avg32": return 0xFF40A673;
            case "uk2": return 0xFFB359CC;
            default: return 0xFF808080;
        }
    }

    @NonNull
    @Override
    public Holder onCreateViewHolder(@NonNull ViewGroup parent, int viewType) {
        View v = LayoutInflater.from(parent.getContext()).inflate(R.layout.item_game, parent, false);
        return new Holder(v);
    }

    @Override
    public void onBindViewHolder(@NonNull Holder holder, int position) {
        GameEntry e = items.get(position);
        holder.title.setText(e.title);
        holder.placeholderTitle.setText(e.title);

        GradientDrawable badgeBg = new GradientDrawable();
        badgeBg.setCornerRadius(100f);
        badgeBg.setColor(engineColor(e.engine));
        holder.badge.setBackground(badgeBg);
        holder.badge.setText(e.engineName);

        String nlsLabel = e.nlsLabel();
        if (nlsLabel != null) {
            holder.subtitle.setText(nlsLabel);
            holder.subtitle.setVisibility(View.VISIBLE);
        } else {
            holder.subtitle.setVisibility(View.GONE);
        }

        Bitmap cover = loadCover(e);
        if (cover != null) {
            BitmapDrawable d = new BitmapDrawable(holder.itemView.getResources(), cover);
            // Icons are pixel art on a card: keep them crisp.
            d.setFilterBitmap(!"icon".equals(e.coverKind));
            holder.cover.setImageDrawable(d);
            holder.cover.setVisibility(View.VISIBLE);
            holder.placeholderTitle.setVisibility(View.GONE);
        } else {
            holder.cover.setImageDrawable(null);
            holder.cover.setVisibility(View.GONE);
            holder.placeholderTitle.setVisibility(View.VISIBLE);
        }
        holder.itemView.setOnClickListener(v -> listener.onGameClicked(e));
        holder.itemView.setOnLongClickListener(v -> {
            listener.onGameLongPressed(e);
            return true;
        });
    }

    @Nullable
    private Bitmap loadCover(GameEntry e) {
        if (e.coverPath == null || e.coverPath.isEmpty()) return null;
        Bitmap cached = covers.get(e.coverPath);
        if (cached != null) return cached;
        if (!new File(e.coverPath).isFile()) return null;
        try {
            Bitmap b = BitmapFactory.decodeFile(e.coverPath);
            if (b != null) covers.put(e.coverPath, b);
            return b;
        } catch (Throwable t) {
            return null;
        }
    }

    @Override
    public int getItemCount() {
        return items.size();
    }

    static final class Holder extends RecyclerView.ViewHolder {
        final ImageView cover;
        final TextView placeholderTitle;
        final TextView badge;
        final TextView title;
        final TextView subtitle;

        Holder(@NonNull View itemView) {
            super(itemView);
            cover = itemView.findViewById(R.id.img_cover);
            itemView.findViewById(R.id.cover_frame).setClipToOutline(true);
            placeholderTitle = itemView.findViewById(R.id.txt_placeholder_title);
            badge = itemView.findViewById(R.id.txt_engine);
            title = itemView.findViewById(R.id.txt_title);
            subtitle = itemView.findViewById(R.id.txt_subtitle);
        }
    }
}
