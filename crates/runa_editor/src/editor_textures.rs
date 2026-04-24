use std::fs;
use std::path::{Path, PathBuf};

use egui::{ColorImage, Id, TextureHandle};
use resvg::{tiny_skia, usvg};

const EDITOR_ICON_RASTER_SIZE: u32 = 64;
const COMPONENT_ICON_RASTER_SIZE: u32 = 96;

pub fn load_editor_icon(ctx: &egui::Context, texture_name: &str, icon_name: &str) -> TextureHandle {
    let icon_path = resolve_editor_icon_path(icon_name).unwrap_or_else(|| {
        panic!(
            "failed to find editor icon `{icon_name}` as SVG or PNG in {}",
            editor_icon_directory().display()
        )
    });

    load_cached_texture(ctx, texture_name, &icon_path, Some(EDITOR_ICON_RASTER_SIZE))
        .unwrap_or_else(|error| panic!("failed to load editor icon `{icon_name}`: {error}"))
}

pub fn load_component_icon(
    ctx: &egui::Context,
    texture_name: &str,
    component_icon_name: &str,
) -> TextureHandle {
    let icon_path = resolve_component_icon_path(component_icon_name)
        .or_else(|| resolve_component_icon_path("c-Object"))
        .unwrap_or_else(|| {
            panic!(
                "failed to find component icon `{component_icon_name}` or fallback `c-Object` as SVG or PNG in {}",
                component_icon_directory().display()
            )
        });

    load_cached_texture(
        ctx,
        texture_name,
        &icon_path,
        Some(COMPONENT_ICON_RASTER_SIZE),
    )
    .or_else(|_| {
        let fallback = resolve_component_icon_path("c-Object")
            .ok_or_else(|| "fallback component icon `c-Object` is missing".to_string())?;
        load_cached_texture(
            ctx,
            "component_icon_fallback_c_object",
            &fallback,
            Some(COMPONENT_ICON_RASTER_SIZE),
        )
    })
    .unwrap_or_else(|error| {
        panic!("failed to load component icon `{component_icon_name}`: {error}")
    })
}

pub fn load_texture_from_path(
    ctx: &egui::Context,
    texture_name: &str,
    path: &Path,
    raster_size: Option<u32>,
) -> Result<TextureHandle, String> {
    let bytes =
        fs::read(path).map_err(|error| format!("failed to read {}: {error}", path.display()))?;

    let extension = path
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_ascii_lowercase())
        .unwrap_or_default();

    match extension.as_str() {
        "svg" => load_svg_texture(
            ctx,
            texture_name,
            &bytes,
            raster_size.unwrap_or(EDITOR_ICON_RASTER_SIZE),
        ),
        _ => load_raster_texture(ctx, texture_name, &bytes),
    }
}

fn resolve_editor_icon_path(icon_name: &str) -> Option<PathBuf> {
    let icon_dir = editor_icon_directory();
    let svg = icon_dir.join(format!("{icon_name}.svg"));
    if svg.exists() {
        return Some(svg);
    }

    let png = icon_dir.join(format!("{icon_name}.png"));
    if png.exists() {
        return Some(png);
    }

    None
}

fn resolve_component_icon_path(icon_name: &str) -> Option<PathBuf> {
    let icon_dir = component_icon_directory();
    let png = icon_dir.join(format!("{icon_name}.png"));
    if png.exists() {
        return Some(png);
    }

    let svg = icon_dir.join(format!("{icon_name}.svg"));
    if svg.exists() {
        return Some(svg);
    }

    None
}

fn editor_icon_directory() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("assets")
        .join("icons")
}

fn component_icon_directory() -> PathBuf {
    editor_icon_directory().join("components")
}

fn load_cached_texture(
    ctx: &egui::Context,
    texture_name: &str,
    path: &Path,
    raster_size: Option<u32>,
) -> Result<TextureHandle, String> {
    let cache_id = Id::new((
        "editor_texture_cache",
        texture_name,
        path.to_string_lossy().to_string(),
    ));
    if let Some(texture) = ctx.data_mut(|data| data.get_temp::<TextureHandle>(cache_id)) {
        return Ok(texture);
    }

    let texture = load_texture_from_path(ctx, texture_name, path, raster_size)?;
    ctx.data_mut(|data| data.insert_temp(cache_id, texture.clone()));
    Ok(texture)
}

fn load_raster_texture(
    ctx: &egui::Context,
    texture_name: &str,
    bytes: &[u8],
) -> Result<TextureHandle, String> {
    let image = image::load_from_memory(bytes)
        .map_err(|error| format!("failed to decode raster image: {error}"))?
        .to_rgba8();
    let size = [image.width() as usize, image.height() as usize];
    let pixels = image.into_raw();
    let color_image = ColorImage::from_rgba_unmultiplied(size, &pixels);
    Ok(ctx.load_texture(texture_name, color_image, egui::TextureOptions::LINEAR))
}

fn load_svg_texture(
    ctx: &egui::Context,
    texture_name: &str,
    bytes: &[u8],
    raster_size: u32,
) -> Result<TextureHandle, String> {
    let options = usvg::Options::default();
    let tree = usvg::Tree::from_data(bytes, &options)
        .map_err(|error| format!("failed to parse SVG: {error}"))?;

    let svg_size = tree.size();
    let largest_axis = svg_size.width().max(svg_size.height());
    if largest_axis <= 0.0 {
        return Err("SVG has invalid size".to_string());
    }

    // Rasterize into a predictable square budget so icons stay crisp in egui.
    let scale = raster_size as f32 / largest_axis;
    let width = (svg_size.width() * scale).round().max(1.0) as u32;
    let height = (svg_size.height() * scale).round().max(1.0) as u32;
    let mut pixmap = tiny_skia::Pixmap::new(width, height)
        .ok_or_else(|| "failed to allocate SVG pixmap".to_string())?;
    let transform = tiny_skia::Transform::from_scale(scale, scale);

    resvg::render(&tree, transform, &mut pixmap.as_mut());

    let color_image =
        ColorImage::from_rgba_unmultiplied([width as usize, height as usize], pixmap.data());
    Ok(ctx.load_texture(texture_name, color_image, egui::TextureOptions::LINEAR))
}
