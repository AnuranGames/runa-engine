use std::path::Path;
use winit::window::Icon;

/// Loads a window icon from an image file.
///
/// # Requirements
/// - Format: PNG (recommended)
/// - Size: multiple of 16 (16x16, 32x32, 64x64, 128x128, 256x256)
/// - Channels: RGBA (with alpha)
///
/// # Example
/// ```rust
/// let icon = load_window_icon("assets/icon.png")?;
/// window.set_window_icon(Some(icon));
/// ```
pub fn load_window_icon<P: AsRef<Path>>(path: P) -> Result<Icon, String> {
    let path = path.as_ref();

    // Load the image through the image crate
    let img = image::open(path)
        .map_err(|e| format!("Failed to load icon '{}': {}", path.display(), e))?;

    // Convert to RGBA8
    let rgba = img.to_rgba8();
    let (width, height) = rgba.dimensions();

    // Validate the size (winit expects square icons)
    if width != height {
        return Err(format!("Icon must be square, got {}x{}", width, height));
    }

    // Validate commonly supported icon sizes
    const VALID_SIZES: &[u32] = &[16, 32, 48, 64, 128, 256, 512];
    if !VALID_SIZES.contains(&width) {
        return Err(format!(
            "Icon size {}x{} is not standard. Recommended: 16, 32, 64, 128, 256, 512",
            width, height
        ));
    }

    // Create the Icon for winit
    Icon::from_rgba(rgba.to_vec(), width, height)
        .map_err(|e| format!("Failed to create icon: {}", e))
}

/// Loads multiple icon sizes (recommended for cross-platform support).
///
/// # Example
/// ```rust
/// let icons = load_window_icons(&[
///     "assets/icon_16.png",
///     "assets/icon_32.png",
///     "assets/icon_64.png",
///     "assets/icon_256.png",
/// ])?;
/// window.set_window_icon(icons.first().cloned()); // winit uses the first compatible icon
/// ```

#[allow(dead_code)]
pub fn load_window_icons<P: AsRef<Path>>(paths: &[P]) -> Result<Vec<Icon>, String> {
    let mut icons = Vec::new();

    for path in paths {
        match load_window_icon(path) {
            Ok(icon) => icons.push(icon),
            Err(e) => eprintln!("⚠️  Skipping icon '{}': {}", path.as_ref().display(), e),
        }
    }

    if icons.is_empty() {
        return Err("No valid icons loaded".to_string());
    }

    Ok(icons)
}
