import init, {
  start_siglus_from_directory,
  launcherScan,
  launcherProbe,
  launcherAddFont,
  WebGame,
} from "./pkg/game_launcher.js";

// ---------------------------------------------------------------------------
// DOM
// ---------------------------------------------------------------------------

const $ = (id) => document.getElementById(id);

const folderInput = $("game-dir");
const fontInput = $("font-file");
const searchInput = $("search");
const statusLine = $("status-line");
const errorBox = $("error-box");
const grid = $("game-grid");
const unsupportedBox = $("unsupported");
const emptyCard = $("empty-card");
const dropZone = $("drop-zone");
const launchOverlay = $("launch-overlay");
const launchText = $("launch-text");
const libraryScreen = $("library-screen");
const playerScreen = $("player-screen");
const playerTitle = $("player-title");
const exitButton = $("exit-button");
const menuButton = $("menu-button");
const skipButton = $("skip-button");
const canvas = $("siglus-canvas");
const reportDialog = $("report-dialog");
const reportTitle = $("report-title");
const reportBody = $("report-body");

const ENGINE_NAMES = {
  siglus: "SiglusEngine",
  reallive: "RealLive",
  avg32: "AVG32",
  uk2: "UK2",
};

const SCAN_DEPTH = 4;

let wasmReady = null;
let games = [];
let unsupported = [];
let running = null;

// ---------------------------------------------------------------------------
// Small helpers
// ---------------------------------------------------------------------------

function setStatus(text) {
  statusLine.textContent = text;
}

function setError(error) {
  const text = error && error.stack ? error.stack : String(error);
  console.error(error);
  errorBox.textContent = text;
  errorBox.style.display = "block";
}

function clearError() {
  errorBox.textContent = "";
  errorBox.style.display = "none";
}

function showLaunching(text) {
  launchText.textContent = text;
  launchOverlay.style.display = "grid";
}

function hideLaunching() {
  launchOverlay.style.display = "none";
}

const nextFrame = () => new Promise((resolve) => setTimeout(resolve, 0));

function normalizePath(path) {
  return String(path || "")
    .replaceAll("\\", "/")
    .split("/")
    .filter((part) => part.length > 0 && part !== ".")
    .join("/");
}

function foldPath(path) {
  let out = "";
  for (let i = 0; i < path.length; i += 1) {
    const code = path.charCodeAt(i);
    out += code >= 0x41 && code <= 0x5a ? String.fromCharCode(code + 0x20) : path[i];
  }
  return out;
}

function parentOf(path) {
  const i = path.lastIndexOf("/");
  return i < 0 ? "" : path.slice(0, i);
}

function baseOf(path) {
  const i = path.lastIndexOf("/");
  return i < 0 ? path : path.slice(i + 1);
}

function hashString(s) {
  let h = 2166136261;
  for (let i = 0; i < s.length; i += 1) {
    h ^= s.charCodeAt(i);
    h = Math.imul(h, 16777619);
  }
  return (h >>> 0).toString(16);
}

function prefGet(key, fallback) {
  try {
    const v = localStorage.getItem(`gl.${key}`);
    return v === null ? fallback : v;
  } catch (_) {
    return fallback;
  }
}

function prefSet(key, value) {
  try {
    if (value === null || value === undefined) localStorage.removeItem(`gl.${key}`);
    else localStorage.setItem(`gl.${key}`, String(value));
  } catch (_) {
    // Storage may be unavailable (private mode); preferences are best-effort.
  }
}

// ---------------------------------------------------------------------------
// Persistent storage (IndexedDB): save files written by games, and fonts.
// ---------------------------------------------------------------------------

const db = (() => {
  let opened = null;
  function open() {
    if (opened) return opened;
    opened = new Promise((resolve) => {
      try {
        const req = indexedDB.open("game_launcher", 1);
        req.onupgradeneeded = () => {
          req.result.createObjectStore("files");
          req.result.createObjectStore("fonts");
        };
        req.onsuccess = () => resolve(req.result);
        req.onerror = () => resolve(null);
      } catch (_) {
        resolve(null);
      }
    });
    return opened;
  }
  async function tx(store, mode, fn) {
    const d = await open();
    if (!d) return null;
    return new Promise((resolve) => {
      try {
        const t = d.transaction(store, mode);
        const result = fn(t.objectStore(store));
        t.oncomplete = () => resolve(result && "result" in result ? result.result : null);
        t.onerror = () => resolve(null);
      } catch (_) {
        resolve(null);
      }
    });
  }
  async function entries(store) {
    const d = await open();
    if (!d) return [];
    return new Promise((resolve) => {
      const out = [];
      try {
        const req = d.transaction(store, "readonly").objectStore(store).openCursor();
        req.onsuccess = () => {
          const cursor = req.result;
          if (cursor) {
            out.push([cursor.key, cursor.value]);
            cursor.continue();
          } else {
            resolve(out);
          }
        };
        req.onerror = () => resolve(out);
      } catch (_) {
        resolve(out);
      }
    });
  }
  return {
    put: (store, key, value) => tx(store, "readwrite", (s) => s.put(value, key)),
    del: (store, key) => tx(store, "readwrite", (s) => s.delete(key)),
    entries,
  };
})();

// ---------------------------------------------------------------------------
// Virtual file system backing the game_fs crate
//
// Every picked folder is registered under its own name ("Folder/sub/file"),
// so several folders (each a game, or containing many games) can be imported
// together. Files written by games go to an overlay persisted in IndexedDB.
// A SiglusEngine game expects paths relative to its root, so while one runs a
// prefix maps those paths into the index.
// ---------------------------------------------------------------------------

const files = new Map(); // path -> File
const overlay = new Map(); // path -> Uint8Array (written by games)
const folded = new Map(); // folded path -> path
const dirChildren = new Map(); // dir -> Set(child name)
let viewPrefix = "";

function addChild(path) {
  const parts = path.split("/");
  let parent = "";
  for (const part of parts) {
    if (!dirChildren.has(parent)) dirChildren.set(parent, new Set());
    dirChildren.get(parent).add(part);
    parent = parent ? `${parent}/${part}` : part;
  }
}

function registerFile(path, file) {
  path = normalizePath(path);
  if (!path) return;
  const key = foldPath(path);
  const existing = folded.get(key);
  if (existing && existing !== path) {
    files.delete(existing);
  }
  files.set(path, file);
  folded.set(key, path);
  addChild(path);
}

function resolvePath(path) {
  let p = normalizePath(path);
  if (viewPrefix) p = p ? `${viewPrefix}/${p}` : viewPrefix;
  if (overlay.has(p) || files.has(p)) return p;
  return folded.get(foldPath(p)) || p;
}

function readFileSync(file) {
  const url = URL.createObjectURL(file);
  try {
    const xhr = new XMLHttpRequest();
    xhr.open("GET", url, false);
    xhr.overrideMimeType("text/plain; charset=x-user-defined");
    xhr.send(null);
    if (xhr.status !== 200 && xhr.status !== 0) {
      throw new Error(`file read failed: HTTP ${xhr.status}`);
    }
    const text = xhr.responseText || "";
    const out = new Uint8Array(text.length);
    for (let i = 0; i < text.length; i += 1) out[i] = text.charCodeAt(i) & 0xff;
    return out;
  } finally {
    URL.revokeObjectURL(url);
  }
}

globalThis.siglusFileExists = function siglusFileExists(path) {
  const p = resolvePath(path);
  return overlay.has(p) || files.has(p);
};

globalThis.siglusReadFile = function siglusReadFile(path) {
  const p = resolvePath(path);
  const written = overlay.get(p);
  if (written) return written;
  const file = files.get(p);
  if (!file) throw new Error(`file not found: ${path}`);
  return readFileSync(file);
};

globalThis.siglusListDir = function siglusListDir(path) {
  let p = normalizePath(path);
  if (viewPrefix) p = p ? `${viewPrefix}/${p}` : viewPrefix;
  let children = dirChildren.get(p);
  if (!children) {
    const key = foldPath(p);
    for (const [dir, set] of dirChildren) {
      if (foldPath(dir) === key) {
        children = set;
        break;
      }
    }
  }
  return children ? Array.from(children) : [];
};

globalThis.siglusKnownFileCount = function siglusKnownFileCount() {
  return files.size + overlay.size;
};

globalThis.gameFsWrite = function gameFsWrite(path, bytes) {
  const p = resolvePath(path);
  const copy = new Uint8Array(bytes); // the view points into wasm memory
  overlay.set(p, copy);
  folded.set(foldPath(p), p);
  addChild(p);
  db.put("files", p, copy);
};

globalThis.gameFsRemove = function gameFsRemove(path) {
  const p = resolvePath(path);
  const had = overlay.delete(p) || files.delete(p);
  if (!had) throw new Error(`file not found: ${path}`);
  folded.delete(foldPath(p));
  const siblings = dirChildren.get(parentOf(p));
  if (siblings) siblings.delete(baseOf(p));
  db.del("files", p);
};

async function restoreSavedFiles() {
  for (const [path, bytes] of await db.entries("files")) {
    overlay.set(path, bytes);
    folded.set(foldPath(path), path);
    addChild(path);
  }
}

// ---------------------------------------------------------------------------
// Fonts (the browser exposes no system fonts to wasm)
// ---------------------------------------------------------------------------

async function restoreFonts() {
  let count = 0;
  for (const [, bytes] of await db.entries("fonts")) {
    launcherAddFont(bytes);
    count += 1;
  }
  // Fonts shipped next to the page: fonts/fonts.json lists file names.
  try {
    const res = await fetch("fonts/fonts.json");
    if (res.ok) {
      for (const name of await res.json()) {
        const font = await fetch(`fonts/${name}`);
        if (font.ok) {
          launcherAddFont(new Uint8Array(await font.arrayBuffer()));
          count += 1;
        }
      }
    }
  } catch (_) {
    // not provided
  }
  return count;
}

async function addFontFiles(list) {
  await ensureWasm();
  let added = 0;
  for (const file of list) {
    const bytes = new Uint8Array(await file.arrayBuffer());
    launcherAddFont(bytes);
    await db.put("fonts", file.name, bytes);
    added += 1;
  }
  setStatus(`${added} font(s) added. Fonts are used for game text in RealLive and AVG32 games.`);
}

// ---------------------------------------------------------------------------
// Wasm
// ---------------------------------------------------------------------------

function ensureWasm() {
  if (!wasmReady) {
    wasmReady = (async () => {
      await init();
      await restoreSavedFiles();
      await restoreFonts();
    })();
  }
  return wasmReady;
}

// ---------------------------------------------------------------------------
// Import
// ---------------------------------------------------------------------------

/** Registers files ({path, file}) and returns their distinct top-level folders. */
function registerPicked(entries) {
  const tops = new Set();
  for (const { path, file } of entries) {
    const p = normalizePath(path);
    if (!p) continue;
    registerFile(p, file);
    const top = p.includes("/") ? p.slice(0, p.indexOf("/")) : "";
    tops.add(top);
  }
  return Array.from(tops);
}

async function readDroppedEntries(dataTransfer) {
  const out = [];
  const roots = [];
  for (const item of dataTransfer.items) {
    const entry = item.webkitGetAsEntry ? item.webkitGetAsEntry() : null;
    if (entry) roots.push(entry);
  }
  async function walk(entry, path) {
    if (entry.isFile) {
      const file = await new Promise((resolve, reject) => entry.file(resolve, reject));
      out.push({ path, file });
    } else if (entry.isDirectory) {
      const reader = entry.createReader();
      for (;;) {
        const batch = await new Promise((resolve, reject) => reader.readEntries(resolve, reject));
        if (batch.length === 0) break;
        for (const child of batch) await walk(child, `${path}/${child.name}`);
      }
    }
  }
  for (const root of roots) await walk(root, root.name);
  return out;
}

function gameFromProbe(info) {
  const id = hashString(info.root);
  const nls = prefGet(`nls.${id}`, null) || info.nls || null;
  return {
    id,
    root: info.root,
    engine: info.engine,
    engineName: info.engine_name || ENGINE_NAMES[info.engine] || info.engine,
    title: info.title || baseOf(info.root) || "Game",
    cover: info.cover || null,
    coverKind: info.cover_kind || "image",
    nls,
    nlsOptions: info.nls_options || [],
    lastPlayed: Number(prefGet(`played.${id}`, "0")) || 0,
  };
}

async function importEntries(entries) {
  if (entries.length === 0) return;
  clearError();
  showLaunching(`Scanning ${entries.length} file(s)…`);
  try {
    await ensureWasm();
    const tops = registerPicked(entries);
    await nextFrame();

    const added = [];
    const again = [];
    const newUnsupported = [];
    for (const top of tops) {
      showLaunching(`Detecting games in ${top || "the selection"}…`);
      await nextFrame();
      let results = [];
      try {
        results = JSON.parse(launcherScan(top, SCAN_DEPTH));
      } catch (error) {
        console.error(error);
      }
      for (const info of results) {
        if (!info.supported) {
          const u = {
            root: info.root,
            title: info.title || baseOf(info.root),
            engineName: info.engine_name || "Unknown",
            reason: info.unsupported_reason || "Unsupported engine",
          };
          unsupported = unsupported.filter((x) => x.root !== u.root);
          unsupported.push(u);
          newUnsupported.push(u);
          continue;
        }
        const game = gameFromProbe(info);
        if (game.nls && info.nls && game.nls !== info.nls) {
          try {
            const refined = JSON.parse(launcherProbe(game.root, game.nls));
            if (refined.title) game.title = refined.title;
          } catch (_) {
            // keep the default title
          }
        }
        const index = games.findIndex((g) => g.id === game.id);
        if (index >= 0) {
          games[index] = game;
          again.push(game);
        } else {
          games.push(game);
          added.push(game);
        }
      }
    }
    sortGames();
    render();
    showReport(added, again, newUnsupported);
  } catch (error) {
    setError(error);
  } finally {
    hideLaunching();
  }
}

function showReport(added, again, bad) {
  const lines = [];
  let title;
  if (added.length > 0) {
    title = added.length === 1 ? "Imported 1 game" : `Imported ${added.length} games`;
    for (const g of added) lines.push(`• ${g.title} (${g.engineName})`);
  } else if (again.length > 0) {
    title = "Already in library";
    for (const g of again) lines.push(`• ${g.title}`);
  } else if (bad.length > 0) {
    title = "Not supported";
  } else {
    title = "No games found";
    lines.push("No SiglusEngine, RealLive, AVG32 or UK2 game was found in the selected folders.");
  }
  if (bad.length > 0) {
    if (lines.length > 0) lines.push("");
    lines.push(bad.length === 1 ? "1 game can't be played here:" : `${bad.length} games can't be played here:`);
    for (const u of bad) lines.push(`• ${u.title} — ${u.reason}`);
  }
  setStatus(`${games.length} game(s) in library.`);
  reportTitle.textContent = title;
  reportBody.textContent = lines.join("\n");
  if (reportDialog.open) reportDialog.close();
  if (typeof reportDialog.showModal === "function") reportDialog.showModal();
  else alert(`${title}\n\n${lines.join("\n")}`);
}

// ---------------------------------------------------------------------------
// Library view
// ---------------------------------------------------------------------------

function sortGames() {
  games.sort((a, b) => {
    if (a.lastPlayed !== b.lastPlayed) return b.lastPlayed - a.lastPlayed;
    return a.title.localeCompare(b.title);
  });
}

function el(tag, className, text) {
  const node = document.createElement(tag);
  if (className) node.className = className;
  if (text !== undefined) node.textContent = text;
  return node;
}

function render() {
  const query = searchInput.value.trim().toLowerCase();
  grid.innerHTML = "";
  const visible = games.filter(
    (g) => !query || g.title.toLowerCase().includes(query) || g.root.toLowerCase().includes(query),
  );
  emptyCard.style.display = games.length === 0 && unsupported.length === 0 ? "block" : "none";

  for (const game of visible) {
    const tile = el("article", "game-tile");

    const poster = el("button", "poster");
    poster.title = `Play ${game.title}`;
    if (game.cover) {
      const img = el("img", game.coverKind === "icon" ? "cover pixelated" : "cover");
      img.src = game.cover;
      img.alt = "";
      poster.append(img);
    } else {
      poster.append(el("div", "poster-title", game.title));
    }
    const badge = el("span", `badge engine-${game.engine}`, game.engineName);
    poster.append(badge, el("span", "play-hint", "▶"));
    poster.addEventListener("click", () => launchGame(game));

    const info = el("div", "tile-info");
    info.append(el("div", "game-title", game.title), el("div", "game-path", game.root));

    const actions = el("div", "tile-actions");
    if (game.nlsOptions.length > 0) {
      const select = el("select", "nls-select");
      select.title = "Text encoding";
      for (const option of game.nlsOptions) {
        const o = el("option", "", option.label);
        o.value = option.id;
        o.selected = option.id === game.nls;
        select.append(o);
      }
      select.addEventListener("change", () => setNls(game, select.value));
      actions.append(select);
    } else {
      const play = el("button", "secondary play-button", "▶ Play");
      play.addEventListener("click", () => launchGame(game));
      actions.append(play, el("div", "grow"));
    }
    const remove = el("button", "icon-button", "✕");
    remove.title = "Remove from library";
    remove.addEventListener("click", () => {
      games = games.filter((g) => g.id !== game.id);
      render();
    });
    actions.append(remove);

    tile.append(poster, info, actions);
    grid.append(tile);
  }

  unsupportedBox.innerHTML = "";
  if (unsupported.length > 0) {
    unsupportedBox.append(el("h2", "", "Not supported"));
    for (const u of unsupported) {
      const row = el("div", "unsupported-row");
      row.append(el("strong", "", u.title), el("span", "", ` · ${u.engineName} — ${u.reason}`));
      unsupportedBox.append(row);
    }
  }
}

async function setNls(game, nls) {
  game.nls = nls;
  prefSet(`nls.${game.id}`, nls);
  try {
    const info = JSON.parse(launcherProbe(game.root, nls));
    if (info.title) game.title = info.title;
  } catch (_) {
    // keep the old title
  }
  render();
}

// ---------------------------------------------------------------------------
// Players
// ---------------------------------------------------------------------------

async function launchGame(game) {
  if (running) return;
  clearError();
  try {
    showLaunching(`Starting ${game.title}…`);
    await ensureWasm();
    game.lastPlayed = Date.now();
    prefSet(`played.${game.id}`, game.lastPlayed);

    libraryScreen.style.display = "none";
    playerScreen.style.display = "block";
    playerTitle.textContent = game.title;
    canvas.focus();

    if (game.engine === "siglus") {
      await launchSiglus(game);
    } else {
      launchFramebuffer(game);
    }
    hideLaunching();
  } catch (error) {
    running = null;
    viewPrefix = "";
    playerScreen.style.display = "none";
    libraryScreen.style.display = "flex";
    hideLaunching();
    setError(error);
  }
}

async function launchSiglus(game) {
  // The Siglus host reads paths relative to the game root.
  viewPrefix = game.root;
  const prefix = game.root ? `${game.root}/` : "";
  const list = [];
  for (const [path, file] of files) {
    if (path.startsWith(prefix)) {
      list.push({ path: path.slice(prefix.length), size: file.size, lastModified: file.lastModified || 0 });
    }
  }
  if (!globalThis.siglusFileExists("Scene.pck")) {
    throw new Error("Scene.pck was not found in the selected SiglusEngine game");
  }
  running = { kind: "siglus" };
  menuButton.style.display = "none";
  skipButton.style.display = "none";
  await start_siglus_from_directory("siglus-canvas", JSON.stringify(list));
}

const KEY_CODES = {
  Enter: 1,
  NumpadEnter: 1,
  Escape: 2,
  Space: 3,
  ArrowUp: 4,
  ArrowDown: 5,
  ArrowLeft: 6,
  ArrowRight: 7,
  PageUp: 8,
  PageDown: 9,
  Home: 10,
  End: 11,
  Backspace: 12,
  Tab: 13,
  ControlLeft: 14,
  ControlRight: 14,
  ShiftLeft: 15,
  ShiftRight: 15,
};

function keyCodeFor(event) {
  if (event.code in KEY_CODES) return KEY_CODES[event.code];
  const f = /^F(\d{1,2})$/.exec(event.code);
  if (f && Number(f[1]) >= 1 && Number(f[1]) <= 12) return 0x100 + Number(f[1]);
  if (event.key && [...event.key].length === 1) return 0x10000 + event.key.codePointAt(0);
  return 0;
}

function launchFramebuffer(game) {
  const web = new WebGame(game.root, game.engine, game.nls || undefined);
  const ctx = canvas.getContext("2d");
  const back = document.createElement("canvas");
  const backCtx = back.getContext("2d");
  let image = null;
  let last = performance.now();
  let rect = { x: 0, y: 0, w: 1, h: 1 };
  let skipping = false;
  const cleanups = [];

  const state = {
    kind: "framebuffer",
    web,
    stop: () => {
      state.stopped = true;
      for (const fn of cleanups) fn();
      try {
        web.close();
      } catch (_) {
        // already closed
      }
      web.free();
    },
    stopped: false,
  };
  running = state;
  menuButton.style.display = "";
  skipButton.style.display = "";
  skipButton.classList.remove("active");

  function layout() {
    const dpr = window.devicePixelRatio || 1;
    const cw = Math.max(1, Math.floor(canvas.clientWidth * dpr));
    const ch = Math.max(1, Math.floor(canvas.clientHeight * dpr));
    if (canvas.width !== cw || canvas.height !== ch) {
      canvas.width = cw;
      canvas.height = ch;
    }
    const fw = back.width || 1;
    const fh = back.height || 1;
    let scale = Math.min(cw / fw, ch / fh);
    const integer = Math.floor(scale);
    if (integer >= 1 && integer / scale > 0.92) scale = integer;
    const w = Math.round(fw * scale);
    const h = Math.round(fh * scale);
    rect = { x: Math.floor((cw - w) / 2), y: Math.floor((ch - h) / 2), w, h };
  }

  function toFrame(event) {
    const bounds = canvas.getBoundingClientRect();
    const dpr = canvas.width / Math.max(1, bounds.width);
    const px = (event.clientX - bounds.left) * dpr;
    const py = (event.clientY - bounds.top) * dpr;
    const x = Math.floor(((px - rect.x) * back.width) / Math.max(1, rect.w));
    const y = Math.floor(((py - rect.y) * back.height) / Math.max(1, rect.h));
    return [Math.max(0, Math.min(back.width - 1, x)), Math.max(0, Math.min(back.height - 1, y))];
  }

  function frame(now) {
    if (state.stopped) return;
    const dt = Math.max(0, Math.min(100, Math.round(now - last)));
    last = now;
    let alive = true;
    try {
      alive = web.step(dt);
    } catch (error) {
      setError(error);
      alive = false;
    }
    if (!alive) {
      exitPlayer();
      return;
    }
    const w = web.width();
    const h = web.height();
    if (w > 0 && h > 0) {
      if (!image || image.width !== w || image.height !== h) {
        back.width = w;
        back.height = h;
        image = backCtx.createImageData(w, h);
      }
      image.data.set(web.frame());
      backCtx.putImageData(image, 0, 0);
      layout();
      ctx.imageSmoothingEnabled = false;
      ctx.fillStyle = "#000";
      ctx.fillRect(0, 0, canvas.width, canvas.height);
      ctx.drawImage(back, rect.x, rect.y, rect.w, rect.h);
      canvas.style.cursor = web.cursorVisible() ? "default" : "none";
    }
    requestAnimationFrame(frame);
  }

  function on(target, type, fn, options) {
    target.addEventListener(type, fn, options);
    cleanups.push(() => target.removeEventListener(type, fn, options));
  }

  on(canvas, "pointermove", (e) => {
    const [x, y] = toFrame(e);
    web.pointerMove(x, y);
  });
  on(canvas, "pointerdown", (e) => {
    canvas.focus();
    const [x, y] = toFrame(e);
    web.pointerMove(x, y);
    if (e.button === 0 || e.button === 2) web.pointerButton(e.button === 2 ? 1 : 0, true);
    e.preventDefault();
  });
  on(canvas, "pointerup", (e) => {
    const [x, y] = toFrame(e);
    web.pointerMove(x, y);
    if (e.button === 0 || e.button === 2) web.pointerButton(e.button === 2 ? 1 : 0, false);
    e.preventDefault();
  });
  on(canvas, "contextmenu", (e) => e.preventDefault());
  on(canvas, "wheel", (e) => {
    web.wheel(e.deltaY < 0);
    e.preventDefault();
  }, { passive: false });
  on(window, "keydown", (e) => {
    if (e.metaKey) return;
    const code = keyCodeFor(e);
    if (code) web.key(code, true);
    if (!e.ctrlKey && !e.altKey && e.key && [...e.key].length === 1) web.text(e.key);
    if (code) e.preventDefault();
  });
  on(window, "keyup", (e) => {
    const code = keyCodeFor(e);
    if (code) {
      web.key(code, false);
      e.preventDefault();
    }
  });
  on(window, "blur", () => {
    web.key(14, skipping);
    web.key(15, false);
  });
  on(menuButton, "click", () => {
    web.pointerButton(1, true);
    web.pointerButton(1, false);
    canvas.focus();
  });
  on(skipButton, "click", () => {
    skipping = !skipping;
    web.key(14, skipping);
    skipButton.classList.toggle("active", skipping);
    canvas.focus();
  });

  requestAnimationFrame((now) => {
    last = now;
    frame(now);
  });
}

function exitPlayer() {
  if (running && running.kind === "siglus") {
    // The Siglus host owns the event loop; a reload is the clean way out.
    window.location.reload();
    return;
  }
  if (running && running.stop) running.stop();
  running = null;
  viewPrefix = "";
  playerScreen.style.display = "none";
  libraryScreen.style.display = "flex";
  sortGames();
  render();
}

// ---------------------------------------------------------------------------
// Events
// ---------------------------------------------------------------------------

folderInput.addEventListener("change", async () => {
  const entries = Array.from(folderInput.files || []).map((file) => ({
    path: file.webkitRelativePath || file.name,
    file,
  }));
  folderInput.value = "";
  await importEntries(entries);
});

fontInput.addEventListener("change", async () => {
  try {
    await addFontFiles(Array.from(fontInput.files || []));
  } catch (error) {
    setError(error);
  }
  fontInput.value = "";
});

searchInput.addEventListener("input", render);

exitButton.addEventListener("click", () => {
  if (running && running.kind === "framebuffer" && !confirm("Quit the game? Unsaved progress will be lost.")) return;
  exitPlayer();
});

$("report-ok").addEventListener("click", () => reportDialog.close());
$("report-more").addEventListener("click", () => reportDialog.close());

let dragDepth = 0;
document.addEventListener("dragenter", (e) => {
  if (running) return;
  dragDepth += 1;
  dropZone.classList.add("visible");
  e.preventDefault();
});
document.addEventListener("dragleave", () => {
  dragDepth = Math.max(0, dragDepth - 1);
  if (dragDepth === 0) dropZone.classList.remove("visible");
});
document.addEventListener("dragover", (e) => e.preventDefault());
document.addEventListener("drop", async (e) => {
  e.preventDefault();
  dragDepth = 0;
  dropZone.classList.remove("visible");
  if (running || !e.dataTransfer) return;
  try {
    const entries = await readDroppedEntries(e.dataTransfer);
    await importEntries(entries);
  } catch (error) {
    setError(error);
  }
});

render();
setStatus("Choose game folders — one game, several games, or a folder that contains many games.");
ensureWasm().catch(setError);
