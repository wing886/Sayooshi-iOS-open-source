//! Game icons for native windows, including the Wayland toplevel icon protocol.

use std::path::{Path, PathBuf};
use winit::icon::RgbaIcon;
use winit::window::WindowAttributes;

/// Apply the game icon before creating its window. Winit sends the pixels
/// directly to Wayland compositors that support xdg_toplevel_icon_v1.
pub fn configure_window(attributes: WindowAttributes, project_dir: &Path) -> WindowAttributes {
    let rgba = load_icon(project_dir);
    let icon = RgbaIcon::new(rgba.as_raw().clone(), rgba.width(), rgba.height())
        .ok()
        .map(Into::into);
    let attributes = attributes.with_window_icon(icon);
    #[cfg(target_os = "linux")]
    {
        use winit::platform::wayland::WindowAttributesWayland;
        let project_dir = project_dir
            .canonicalize()
            .unwrap_or_else(|_| project_dir.to_owned());
        let app_id = application_id(&project_dir);
        attributes.with_platform_attributes(Box::new(
            WindowAttributesWayland::default().with_name(&app_id, "siglus_engine"),
        ))
    }
    #[cfg(not(target_os = "linux"))]
    {
        attributes
    }
}

fn load_icon(project_dir: &Path) -> image::RgbaImage {
    // Prefer explicit icon names, then game-specific ICO files (e.g. train.ico).
    // Resolve names case-insensitively, just like the other game resources.
    let mut candidates: Vec<PathBuf> = ["icon.png", "icon.ico"]
        .iter()
        .filter_map(|name| {
            crate::resource::resolve_game_file(&project_dir.join(name))
                .ok()
                .flatten()
        })
        .collect();
    let mut ico_files: Vec<_> = std::fs::read_dir(project_dir)
        .into_iter()
        .flatten()
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| {
            path.extension()
                .is_some_and(|ext| ext.eq_ignore_ascii_case("ico"))
        })
        .collect();
    ico_files.sort();
    candidates.extend(ico_files);
    for path in candidates {
        match image::open(&path) {
            Ok(image) => return image.into_rgba8(),
            Err(error) => log::warn!("Could not load window icon {}: {error}", path.display()),
        }
    }
    image::load_from_memory_with_format(
        include_bytes!("../../../icon/Icon.png"),
        image::ImageFormat::Png,
    )
    .expect("bundled Siglus icon")
    .into_rgba8()
}

#[cfg(target_os = "linux")]
fn application_id(project_dir: &Path) -> String {
    use std::os::unix::ffi::OsStrExt;
    // Stable FNV-1a, unlike DefaultHasher whose algorithm may change between
    // releases. Different games must not share taskbar grouping or icon caches.
    let hash = project_dir
        .as_os_str()
        .as_bytes()
        .iter()
        .fold(0xcbf29ce484222325u64, |hash, byte| {
            (hash ^ u64::from(*byte)).wrapping_mul(0x100000001b3)
        });
    format!("org.siglus_rs.game.g{hash:016x}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn game_ico_is_loaded_and_invalid_icons_fall_back() {
        let root = std::env::temp_dir().join(format!("siglus-icon-test-{}", std::process::id()));
        std::fs::create_dir_all(&root).unwrap();
        std::fs::write(root.join("icon.png"), b"invalid").unwrap();
        let expected = image::RgbaImage::from_pixel(32, 32, image::Rgba([20, 40, 60, 255]));
        expected.save(root.join("train.ICO")).unwrap();
        assert_eq!(load_icon(&root), expected);
        std::fs::remove_file(root.join("train.ICO")).unwrap();
        assert!(load_icon(&root).width() > 0);
        std::fs::remove_dir_all(root).unwrap();
    }
}
