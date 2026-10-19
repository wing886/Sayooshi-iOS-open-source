#include <jni.h>
#include <android/native_window_jni.h>
#include <android/log.h>
#include <dlfcn.h>

#include <cstdint>
#include <cstring>
#include <mutex>
#include <string>
#include <unordered_map>

#define LOG_TAG "siglus_jni"
#define LOGE(...) __android_log_print(ANDROID_LOG_ERROR, LOG_TAG, __VA_ARGS__)
#define LOGW(...) __android_log_print(ANDROID_LOG_WARN, LOG_TAG, __VA_ARGS__)
#define LOGI(...) __android_log_print(ANDROID_LOG_INFO, LOG_TAG, __VA_ARGS__)

using messagebox_callback_t = void (*)(void* user_data,
                                       uint64_t request_id,
                                       int32_t kind,
                                       const char* title_utf8,
                                       const char* message_utf8);

using create_fn_t = void* (*)(void* native_window_ptr,
                              uint32_t w_px,
                              uint32_t h_px,
                              double scale,
                              const char* game_dir_utf8);
using set_messagebox_callback_fn_t = void (*)(void* handle, messagebox_callback_t callback, void* user_data);
using submit_messagebox_result_fn_t = void (*)(void* handle, uint64_t request_id, int64_t value);
using step_fn_t = int32_t (*)(void* handle, uint32_t dt_ms);
using resize_fn_t = void (*)(void* handle, uint32_t w_px, uint32_t h_px);
using set_surface_fn_t = int32_t (*)(void* handle, void* native_window_ptr, uint32_t w_px, uint32_t h_px);
using touch_fn_t = void (*)(void* handle, int32_t phase, double x_px, double y_px);
using key_fn_t = void (*)(void* handle, int32_t key_code);
using destroy_fn_t = void (*)(void* handle);
using init_context_fn_t = void (*)(void* java_vm_ptr, void* app_context_global_ref);
using string_free_fn_t = void (*)(char* ptr);
using string_from_dir_fn_t = char* (*)(const char* game_dir_utf8);

struct Api {
    create_fn_t create = nullptr;
    set_messagebox_callback_fn_t set_messagebox_callback = nullptr;
    submit_messagebox_result_fn_t submit_messagebox_result = nullptr;
    step_fn_t step = nullptr;
    resize_fn_t resize = nullptr;
    set_surface_fn_t set_surface = nullptr;
    touch_fn_t touch = nullptr;
    key_fn_t key_down = nullptr;
    key_fn_t key_up = nullptr;
    destroy_fn_t destroy = nullptr;
    init_context_fn_t init_context = nullptr;
    string_free_fn_t string_free = nullptr;
    string_from_dir_fn_t game_name_from_dir = nullptr;
    string_from_dir_fn_t game_cover_path_from_dir = nullptr;
    string_from_dir_fn_t game_cover_mime_from_dir = nullptr;
};

static Api g_api;
static std::once_flag g_api_once;
static void* g_lib_handle = nullptr;

static std::once_flag g_ctx_once;
static JavaVM* g_java_vm = nullptr;
static jobject g_app_ctx = nullptr;

static std::mutex g_win_mu;
static std::unordered_map<jlong, ANativeWindow*> g_windows;

static jclass g_activity_class = nullptr;
static jmethodID g_activity_on_messagebox = nullptr;

static void* load_symbol(const char* sym) {
    void* p = dlsym(g_lib_handle, sym);
    if (!p) {
        LOGE("dlsym failed: %s (%s)", sym, dlerror());
    }
    return p;
}

static void load_api_or_log() {
    std::call_once(g_api_once, []() {
        g_lib_handle = dlopen("libsiglus.so", RTLD_NOW);
        if (!g_lib_handle) {
            LOGE("dlopen libsiglus.so failed: %s", dlerror());
            return;
        }

        g_api.create = reinterpret_cast<create_fn_t>(load_symbol("siglus_android_create"));
        g_api.set_messagebox_callback = reinterpret_cast<set_messagebox_callback_fn_t>(load_symbol("siglus_android_set_native_messagebox_callback"));
        g_api.submit_messagebox_result = reinterpret_cast<submit_messagebox_result_fn_t>(load_symbol("siglus_android_submit_messagebox_result"));
        g_api.step = reinterpret_cast<step_fn_t>(load_symbol("siglus_android_step"));
        g_api.resize = reinterpret_cast<resize_fn_t>(load_symbol("siglus_android_resize"));
        g_api.set_surface = reinterpret_cast<set_surface_fn_t>(load_symbol("siglus_android_set_surface"));
        g_api.touch = reinterpret_cast<touch_fn_t>(load_symbol("siglus_android_touch"));
        g_api.key_down = reinterpret_cast<key_fn_t>(load_symbol("siglus_android_key_down"));
        g_api.key_up = reinterpret_cast<key_fn_t>(load_symbol("siglus_android_key_up"));
        g_api.destroy = reinterpret_cast<destroy_fn_t>(load_symbol("siglus_android_destroy"));
        g_api.init_context = reinterpret_cast<init_context_fn_t>(load_symbol("siglus_android_init_context"));
        g_api.string_free = reinterpret_cast<string_free_fn_t>(load_symbol("siglus_string_free"));
        g_api.game_name_from_dir = reinterpret_cast<string_from_dir_fn_t>(load_symbol("siglus_game_name_from_dir"));
        g_api.game_cover_path_from_dir = reinterpret_cast<string_from_dir_fn_t>(load_symbol("siglus_game_cover_path_from_dir"));
        g_api.game_cover_mime_from_dir = reinterpret_cast<string_from_dir_fn_t>(load_symbol("siglus_game_cover_mime_from_dir"));

        if (g_api.create && g_api.step && g_api.resize && g_api.set_surface && g_api.touch && g_api.destroy) {
            LOGI("siglus_android_* symbols resolved");
        } else {
            LOGE("missing one or more required siglus_android_* symbols");
        }
        if (!g_api.init_context) {
            LOGW("siglus_android_init_context is missing; Android audio backends may fail");
        }
        if (!g_api.set_messagebox_callback || !g_api.submit_messagebox_result) {
            LOGW("native messagebox callback symbols are missing");
        }
    });
}

static JNIEnv* attach_current_thread(bool* did_attach) {
    if (did_attach) {
        *did_attach = false;
    }
    if (!g_java_vm) {
        return nullptr;
    }
    JNIEnv* env = nullptr;
    jint status = g_java_vm->GetEnv(reinterpret_cast<void**>(&env), JNI_VERSION_1_6);
    if (status == JNI_OK) {
        return env;
    }
    if (status == JNI_EDETACHED) {
        if (g_java_vm->AttachCurrentThread(&env, nullptr) == JNI_OK) {
            if (did_attach) {
                *did_attach = true;
            }
            return env;
        }
    }
    return nullptr;
}

static void detach_current_thread_if_needed(bool did_attach) {
    if (did_attach && g_java_vm) {
        g_java_vm->DetachCurrentThread();
    }
}

static void release_window_locked(jlong handle_key) {
    auto it = g_windows.find(handle_key);
    if (it != g_windows.end()) {
        if (it->second) {
            ANativeWindow_release(it->second);
        }
        g_windows.erase(it);
    }
}

static const char* get_utf8_or_null(JNIEnv* env, jstring s) {
    if (!s) return nullptr;
    return env->GetStringUTFChars(s, nullptr);
}

static void release_utf8(JNIEnv* env, jstring s, const char* p) {
    if (s && p) {
        env->ReleaseStringUTFChars(s, p);
    }
}

static jstring take_siglus_string(JNIEnv* env, char* p) {
    if (!p) {
        return nullptr;
    }
    jstring out = env->NewStringUTF(p);
    if (g_api.string_free) {
        g_api.string_free(p);
    }
    return out;
}

static jstring string_from_dir(JNIEnv* env, jstring game_dir_utf8, string_from_dir_fn_t fn) {
    if (!fn) {
        return nullptr;
    }
    const char* game_dir = get_utf8_or_null(env, game_dir_utf8);
    char* p = fn(game_dir);
    release_utf8(env, game_dir_utf8, game_dir);
    return take_siglus_string(env, p);
}

static int64_t fallback_messagebox_value(int32_t kind) {
    switch (kind) {
        case 0: return 0; // OK
        case 1: return 1; // OK/CANCEL -> CANCEL
        case 2: return 1; // YES/NO -> NO
        case 3: return 2; // YES/NO/CANCEL -> CANCEL
        default: return 0;
    }
}

static void siglus_messagebox_callback(void* user_data,
                                       uint64_t request_id,
                                       int32_t kind,
                                       const char* title_utf8,
                                       const char* message_utf8) {
    auto handle_key = reinterpret_cast<jlong>(user_data);
    bool did_attach = false;
    JNIEnv* env = attach_current_thread(&did_attach);
    if (!env || !g_activity_class || !g_activity_on_messagebox) {
        if (g_api.submit_messagebox_result && handle_key != 0) {
            g_api.submit_messagebox_result(reinterpret_cast<void*>(handle_key), request_id, fallback_messagebox_value(kind));
        }
        detach_current_thread_if_needed(did_attach);
        return;
    }

    jstring title = env->NewStringUTF(title_utf8 ? title_utf8 : "Siglus");
    jstring message = env->NewStringUTF(message_utf8 ? message_utf8 : "");
    env->CallStaticVoidMethod(g_activity_class,
                              g_activity_on_messagebox,
                              handle_key,
                              static_cast<jlong>(request_id),
                              static_cast<jint>(kind),
                              title,
                              message);
    if (env->ExceptionCheck()) {
        env->ExceptionClear();
        if (g_api.submit_messagebox_result && handle_key != 0) {
            g_api.submit_messagebox_result(reinterpret_cast<void*>(handle_key), request_id, fallback_messagebox_value(kind));
        }
    }
    if (title) env->DeleteLocalRef(title);
    if (message) env->DeleteLocalRef(message);
    detach_current_thread_if_needed(did_attach);
}

extern "C" JNIEXPORT jint JNICALL JNI_OnLoad(JavaVM* vm, void*) {
    g_java_vm = vm;
    JNIEnv* env = nullptr;
    if (vm->GetEnv(reinterpret_cast<void**>(&env), JNI_VERSION_1_6) != JNI_OK || !env) {
        return JNI_ERR;
    }
    jclass local = env->FindClass("com/chino/siglus/SiglusGameActivity");
    if (local) {
        g_activity_class = reinterpret_cast<jclass>(env->NewGlobalRef(local));
        env->DeleteLocalRef(local);
        if (g_activity_class) {
            g_activity_on_messagebox = env->GetStaticMethodID(
                    g_activity_class,
                    "onNativeMessagebox",
                    "(JJILjava/lang/String;Ljava/lang/String;)V");
        }
    }
    if (!g_activity_class || !g_activity_on_messagebox) {
        LOGW("failed to resolve SiglusGameActivity.onNativeMessagebox");
    }
    return JNI_VERSION_1_6;
}

extern "C" JNIEXPORT void JNICALL
Java_com_chino_siglus_NativeSiglus_nativeInitAndroidContext(JNIEnv* env, jclass, jobject app_context) {
    load_api_or_log();
    if (!g_api.init_context) {
        LOGE("nativeInitAndroidContext: siglus_android_init_context is null");
        return;
    }
    if (!app_context) {
        LOGE("nativeInitAndroidContext: app_context is null");
        return;
    }

    std::call_once(g_ctx_once, [env, app_context]() {
        JavaVM* vm = nullptr;
        if (env->GetJavaVM(&vm) != JNI_OK || !vm) {
            LOGE("nativeInitAndroidContext: GetJavaVM failed");
            return;
        }
        jobject gref = env->NewGlobalRef(app_context);
        if (!gref) {
            LOGE("nativeInitAndroidContext: NewGlobalRef failed");
            return;
        }
        g_java_vm = vm;
        g_app_ctx = gref;
        g_api.init_context(reinterpret_cast<void*>(vm), reinterpret_cast<void*>(gref));
        LOGI("nativeInitAndroidContext: ndk-context initialized");
    });
}

extern "C" JNIEXPORT jstring JNICALL
Java_com_chino_siglus_NativeSiglus_gameNameFromDir(JNIEnv* env, jclass, jstring game_dir_utf8) {
    load_api_or_log();
    return string_from_dir(env, game_dir_utf8, g_api.game_name_from_dir);
}

extern "C" JNIEXPORT jstring JNICALL
Java_com_chino_siglus_NativeSiglus_gameCoverPathFromDir(JNIEnv* env, jclass, jstring game_dir_utf8) {
    load_api_or_log();
    return string_from_dir(env, game_dir_utf8, g_api.game_cover_path_from_dir);
}

extern "C" JNIEXPORT jstring JNICALL
Java_com_chino_siglus_NativeSiglus_gameCoverMimeFromDir(JNIEnv* env, jclass, jstring game_dir_utf8) {
    load_api_or_log();
    return string_from_dir(env, game_dir_utf8, g_api.game_cover_mime_from_dir);
}

extern "C" JNIEXPORT jlong JNICALL
Java_com_chino_siglus_NativeSiglus_create(JNIEnv* env, jclass,
                                        jobject surface,
                                        jint width_px,
                                        jint height_px,
                                        jdouble scale,
                                        jstring game_dir_utf8) {
    load_api_or_log();
    if (!g_api.create) {
        return 0;
    }
    if (!surface) {
        LOGE("create: surface is null");
        return 0;
    }

    ANativeWindow* win = ANativeWindow_fromSurface(env, surface);
    if (!win) {
        LOGE("ANativeWindow_fromSurface returned null");
        return 0;
    }

    const char* game_dir = get_utf8_or_null(env, game_dir_utf8);

    void* handle = g_api.create(reinterpret_cast<void*>(win),
                                static_cast<uint32_t>(width_px),
                                static_cast<uint32_t>(height_px),
                                static_cast<double>(scale),
                                game_dir);

    release_utf8(env, game_dir_utf8, game_dir);

    if (!handle) {
        ANativeWindow_release(win);
        LOGE("siglus_android_create returned null");
        return 0;
    }

    jlong key = reinterpret_cast<jlong>(handle);
    {
        std::lock_guard<std::mutex> lk(g_win_mu);
        release_window_locked(key);
        g_windows.emplace(key, win);
    }

    if (g_api.set_messagebox_callback) {
        g_api.set_messagebox_callback(handle, siglus_messagebox_callback, reinterpret_cast<void*>(key));
    }

    return key;
}

extern "C" JNIEXPORT void JNICALL
Java_com_chino_siglus_NativeSiglus_setNativeMessageboxCallback(JNIEnv*, jclass, jlong handle) {
    load_api_or_log();
    if (g_api.set_messagebox_callback && handle != 0) {
        g_api.set_messagebox_callback(reinterpret_cast<void*>(handle), siglus_messagebox_callback, reinterpret_cast<void*>(handle));
    }
}

extern "C" JNIEXPORT void JNICALL
Java_com_chino_siglus_NativeSiglus_submitMessageboxResult(JNIEnv*, jclass,
                                                        jlong handle,
                                                        jlong request_id,
                                                        jlong value) {
    load_api_or_log();
    if (g_api.submit_messagebox_result && handle != 0) {
        g_api.submit_messagebox_result(reinterpret_cast<void*>(handle), static_cast<uint64_t>(request_id), static_cast<int64_t>(value));
    }
}

extern "C" JNIEXPORT jint JNICALL
Java_com_chino_siglus_NativeSiglus_step(JNIEnv*, jclass, jlong handle, jint dt_ms) {
    load_api_or_log();
    if (!g_api.step || handle == 0) {
        return 1;
    }
    return static_cast<jint>(g_api.step(reinterpret_cast<void*>(handle), static_cast<uint32_t>(dt_ms)));
}

extern "C" JNIEXPORT void JNICALL
Java_com_chino_siglus_NativeSiglus_resize(JNIEnv*, jclass, jlong handle, jint width_px, jint height_px) {
    load_api_or_log();
    if (!g_api.resize || handle == 0) {
        return;
    }
    g_api.resize(reinterpret_cast<void*>(handle), static_cast<uint32_t>(width_px), static_cast<uint32_t>(height_px));
}

extern "C" JNIEXPORT jboolean JNICALL
Java_com_chino_siglus_NativeSiglus_setSurface(JNIEnv* env, jclass,
                                            jlong handle,
                                            jobject surface,
                                            jint width_px,
                                            jint height_px) {
    load_api_or_log();
    if (!g_api.set_surface || handle == 0) {
        return JNI_FALSE;
    }
    if (!surface) {
        LOGW("setSurface: surface is null (ignored)");
        return JNI_FALSE;
    }

    ANativeWindow* win = ANativeWindow_fromSurface(env, surface);
    if (!win) {
        LOGE("setSurface: ANativeWindow_fromSurface returned null");
        return JNI_FALSE;
    }

    const int32_t attached = g_api.set_surface(
        reinterpret_cast<void*>(handle),
        reinterpret_cast<void*>(win),
        static_cast<uint32_t>(width_px),
        static_cast<uint32_t>(height_px));
    if (attached == 0) {
        ANativeWindow_release(win);
        LOGE("setSurface: siglus_android_set_surface rejected replacement surface");
        return JNI_FALSE;
    }

    {
        std::lock_guard<std::mutex> lk(g_win_mu);
        release_window_locked(handle);
        g_windows.emplace(handle, win);
    }
    return JNI_TRUE;
}

extern "C" JNIEXPORT void JNICALL
Java_com_chino_siglus_NativeSiglus_touch(JNIEnv*, jclass, jlong handle, jint phase, jdouble x_px, jdouble y_px) {
    load_api_or_log();
    if (!g_api.touch || handle == 0) {
        return;
    }
    g_api.touch(reinterpret_cast<void*>(handle), static_cast<int32_t>(phase),
                static_cast<double>(x_px), static_cast<double>(y_px));
}

extern "C" JNIEXPORT void JNICALL
Java_com_chino_siglus_NativeSiglus_keyDown(JNIEnv*, jclass, jlong handle, jint key_code) {
    load_api_or_log();
    if (!g_api.key_down || handle == 0) {
        return;
    }
    g_api.key_down(reinterpret_cast<void*>(handle), static_cast<int32_t>(key_code));
}

extern "C" JNIEXPORT void JNICALL
Java_com_chino_siglus_NativeSiglus_keyUp(JNIEnv*, jclass, jlong handle, jint key_code) {
    load_api_or_log();
    if (!g_api.key_up || handle == 0) {
        return;
    }
    g_api.key_up(reinterpret_cast<void*>(handle), static_cast<int32_t>(key_code));
}

extern "C" JNIEXPORT void JNICALL
Java_com_chino_siglus_NativeSiglus_destroy(JNIEnv*, jclass, jlong handle) {
    load_api_or_log();
    if (!g_api.destroy || handle == 0) {
        return;
    }
    if (g_api.set_messagebox_callback) {
        g_api.set_messagebox_callback(reinterpret_cast<void*>(handle), nullptr, nullptr);
    }
    g_api.destroy(reinterpret_cast<void*>(handle));
    {
        std::lock_guard<std::mutex> lk(g_win_mu);
        release_window_locked(handle);
    }
}

// ---------------------------------------------------------------------------
// Multi-engine launcher API (game_launcher): probing, and frame-by-frame
// hosting of RealLive, AVG32 and UK2 games.  Exposed to Java as NativeGames.
// ---------------------------------------------------------------------------

using game_string_free_fn_t = void (*)(char*);
using game_probe_json_fn_t = char* (*)(const char*, const char*, const char*);
using game_scan_json_fn_t = char* (*)(const char*, int32_t, const char*);
using game_add_font_file_fn_t = int32_t (*)(const char*);
using game_fb_open_fn_t = void* (*)(const char*, const char*, const char*, char**);
using game_fb_step_fn_t = int32_t (*)(void*, uint32_t);
using game_fb_frame_fn_t = const uint8_t* (*)(void*, uint32_t*, uint32_t*);
using game_fb_xy_fn_t = void (*)(void*, int32_t, int32_t);
using game_fb_int_fn_t = void (*)(void*, int32_t);
using game_fb_key_fn_t = void (*)(void*, uint32_t, int32_t);
using game_fb_text_fn_t = void (*)(void*, const char*);
using game_fb_close_fn_t = void (*)(void*);

struct GameApi {
    game_string_free_fn_t string_free = nullptr;
    game_probe_json_fn_t probe_json = nullptr;
    game_scan_json_fn_t scan_json = nullptr;
    game_add_font_file_fn_t add_font_file = nullptr;
    game_fb_open_fn_t fb_open = nullptr;
    game_fb_step_fn_t fb_step = nullptr;
    game_fb_frame_fn_t fb_frame = nullptr;
    game_fb_xy_fn_t fb_pointer_move = nullptr;
    game_fb_xy_fn_t fb_pointer_button = nullptr;
    game_fb_int_fn_t fb_wheel = nullptr;
    game_fb_key_fn_t fb_key = nullptr;
    game_fb_text_fn_t fb_text = nullptr;
    game_fb_close_fn_t fb_close = nullptr;
};

static GameApi g_game_api;
static std::once_flag g_game_api_once;

static void load_game_api_or_log() {
    load_api_or_log();
    std::call_once(g_game_api_once, []() {
        if (!g_lib_handle) {
            return;
        }
        g_game_api.string_free = reinterpret_cast<game_string_free_fn_t>(load_symbol("game_string_free"));
        g_game_api.probe_json = reinterpret_cast<game_probe_json_fn_t>(load_symbol("game_probe_json"));
        g_game_api.scan_json = reinterpret_cast<game_scan_json_fn_t>(load_symbol("game_scan_json"));
        g_game_api.add_font_file = reinterpret_cast<game_add_font_file_fn_t>(load_symbol("game_add_font_file"));
        g_game_api.fb_open = reinterpret_cast<game_fb_open_fn_t>(load_symbol("game_fb_open"));
        g_game_api.fb_step = reinterpret_cast<game_fb_step_fn_t>(load_symbol("game_fb_step"));
        g_game_api.fb_frame = reinterpret_cast<game_fb_frame_fn_t>(load_symbol("game_fb_frame"));
        g_game_api.fb_pointer_move = reinterpret_cast<game_fb_xy_fn_t>(load_symbol("game_fb_pointer_move"));
        g_game_api.fb_pointer_button = reinterpret_cast<game_fb_xy_fn_t>(load_symbol("game_fb_pointer_button"));
        g_game_api.fb_wheel = reinterpret_cast<game_fb_int_fn_t>(load_symbol("game_fb_wheel"));
        g_game_api.fb_key = reinterpret_cast<game_fb_key_fn_t>(load_symbol("game_fb_key"));
        g_game_api.fb_text = reinterpret_cast<game_fb_text_fn_t>(load_symbol("game_fb_text"));
        g_game_api.fb_close = reinterpret_cast<game_fb_close_fn_t>(load_symbol("game_fb_close"));
    });
}

static jstring take_game_string(JNIEnv* env, char* p) {
    if (!p) {
        return nullptr;
    }
    jstring out = env->NewStringUTF(p);
    if (g_game_api.string_free) {
        g_game_api.string_free(p);
    }
    return out;
}

extern "C" JNIEXPORT jstring JNICALL
Java_com_chino_siglus_NativeGames_scanJson(JNIEnv* env, jclass, jstring path, jint depth, jstring cache_dir) {
    load_game_api_or_log();
    if (!g_game_api.scan_json) {
        return nullptr;
    }
    const char* p = get_utf8_or_null(env, path);
    const char* c = get_utf8_or_null(env, cache_dir);
    char* out = g_game_api.scan_json(p, static_cast<int32_t>(depth), c);
    release_utf8(env, path, p);
    release_utf8(env, cache_dir, c);
    return take_game_string(env, out);
}

extern "C" JNIEXPORT jstring JNICALL
Java_com_chino_siglus_NativeGames_probeJson(JNIEnv* env, jclass, jstring root, jstring nls, jstring cache_dir) {
    load_game_api_or_log();
    if (!g_game_api.probe_json) {
        return nullptr;
    }
    const char* r = get_utf8_or_null(env, root);
    const char* n = get_utf8_or_null(env, nls);
    const char* c = get_utf8_or_null(env, cache_dir);
    char* out = g_game_api.probe_json(r, n, c);
    release_utf8(env, root, r);
    release_utf8(env, nls, n);
    release_utf8(env, cache_dir, c);
    return take_game_string(env, out);
}

extern "C" JNIEXPORT jint JNICALL
Java_com_chino_siglus_NativeGames_addFontFile(JNIEnv* env, jclass, jstring path) {
    load_game_api_or_log();
    if (!g_game_api.add_font_file) {
        return 1;
    }
    const char* p = get_utf8_or_null(env, path);
    jint rc = g_game_api.add_font_file(p);
    release_utf8(env, path, p);
    return rc;
}

/** Returns a handle, or 0 with the reason in errorOut[0]. */
extern "C" JNIEXPORT jlong JNICALL
Java_com_chino_siglus_NativeGames_fbOpen(JNIEnv* env, jclass, jstring root, jstring engine, jstring nls, jobjectArray error_out) {
    load_game_api_or_log();
    if (!g_game_api.fb_open) {
        return 0;
    }
    const char* r = get_utf8_or_null(env, root);
    const char* e = get_utf8_or_null(env, engine);
    const char* n = get_utf8_or_null(env, nls);
    char* error = nullptr;
    void* handle = g_game_api.fb_open(r, e, n, &error);
    release_utf8(env, root, r);
    release_utf8(env, engine, e);
    release_utf8(env, nls, n);
    if (error) {
        jstring message = take_game_string(env, error);
        if (error_out && env->GetArrayLength(error_out) > 0) {
            env->SetObjectArrayElement(error_out, 0, message);
        }
    }
    return reinterpret_cast<jlong>(handle);
}

extern "C" JNIEXPORT jint JNICALL
Java_com_chino_siglus_NativeGames_fbStep(JNIEnv*, jclass, jlong handle, jint dt_ms) {
    if (!g_game_api.fb_step || handle == 0) {
        return 1;
    }
    return g_game_api.fb_step(reinterpret_cast<void*>(handle), static_cast<uint32_t>(dt_ms));
}

/** Frame size as {width, height}, or null before the first frame. */
extern "C" JNIEXPORT jintArray JNICALL
Java_com_chino_siglus_NativeGames_fbFrameSize(JNIEnv* env, jclass, jlong handle) {
    if (!g_game_api.fb_frame || handle == 0) {
        return nullptr;
    }
    uint32_t w = 0, h = 0;
    if (!g_game_api.fb_frame(reinterpret_cast<void*>(handle), &w, &h) || w == 0 || h == 0) {
        return nullptr;
    }
    jintArray out = env->NewIntArray(2);
    jint values[2] = {static_cast<jint>(w), static_cast<jint>(h)};
    env->SetIntArrayRegion(out, 0, 2, values);
    return out;
}

/** Copies the RGBA frame into a direct buffer of at least w*h*4 bytes. */
extern "C" JNIEXPORT jboolean JNICALL
Java_com_chino_siglus_NativeGames_fbCopyFrame(JNIEnv* env, jclass, jlong handle, jobject buffer) {
    if (!g_game_api.fb_frame || handle == 0 || !buffer) {
        return JNI_FALSE;
    }
    uint32_t w = 0, h = 0;
    const uint8_t* pixels = g_game_api.fb_frame(reinterpret_cast<void*>(handle), &w, &h);
    void* dst = env->GetDirectBufferAddress(buffer);
    jlong capacity = env->GetDirectBufferCapacity(buffer);
    size_t bytes = static_cast<size_t>(w) * h * 4;
    if (!pixels || !dst || capacity < static_cast<jlong>(bytes)) {
        return JNI_FALSE;
    }
    memcpy(dst, pixels, bytes);
    // Frames are opaque; force alpha so a premultiplied Bitmap keeps colour.
    uint8_t* out = static_cast<uint8_t*>(dst);
    for (size_t i = 3; i < bytes; i += 4) {
        out[i] = 0xff;
    }
    return JNI_TRUE;
}

extern "C" JNIEXPORT void JNICALL
Java_com_chino_siglus_NativeGames_fbPointerMove(JNIEnv*, jclass, jlong handle, jint x, jint y) {
    if (g_game_api.fb_pointer_move && handle != 0) {
        g_game_api.fb_pointer_move(reinterpret_cast<void*>(handle), x, y);
    }
}

extern "C" JNIEXPORT void JNICALL
Java_com_chino_siglus_NativeGames_fbPointerButton(JNIEnv*, jclass, jlong handle, jint button, jboolean pressed) {
    if (g_game_api.fb_pointer_button && handle != 0) {
        g_game_api.fb_pointer_button(reinterpret_cast<void*>(handle), button, pressed ? 1 : 0);
    }
}

extern "C" JNIEXPORT void JNICALL
Java_com_chino_siglus_NativeGames_fbWheel(JNIEnv*, jclass, jlong handle, jboolean up) {
    if (g_game_api.fb_wheel && handle != 0) {
        g_game_api.fb_wheel(reinterpret_cast<void*>(handle), up ? 1 : 0);
    }
}

extern "C" JNIEXPORT void JNICALL
Java_com_chino_siglus_NativeGames_fbKey(JNIEnv*, jclass, jlong handle, jint code, jboolean pressed) {
    if (g_game_api.fb_key && handle != 0) {
        g_game_api.fb_key(reinterpret_cast<void*>(handle), static_cast<uint32_t>(code), pressed ? 1 : 0);
    }
}

extern "C" JNIEXPORT void JNICALL
Java_com_chino_siglus_NativeGames_fbText(JNIEnv* env, jclass, jlong handle, jstring text) {
    if (!g_game_api.fb_text || handle == 0) {
        return;
    }
    const char* t = get_utf8_or_null(env, text);
    g_game_api.fb_text(reinterpret_cast<void*>(handle), t);
    release_utf8(env, text, t);
}

extern "C" JNIEXPORT void JNICALL
Java_com_chino_siglus_NativeGames_fbClose(JNIEnv*, jclass, jlong handle) {
    if (g_game_api.fb_close && handle != 0) {
        g_game_api.fb_close(reinterpret_cast<void*>(handle));
    }
}
