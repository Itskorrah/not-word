//! Approved original-B identity, rendered from the same canonical assets as the application and
//! documentation. The symbol and bold wordmark form one lockup; interface accents remain separate.

use egui::{Context, TextureHandle, Ui, vec2};

use crate::theme::Tokens;

fn texture(ctx: &Context, dark: bool) -> Option<TextureHandle> {
    let id = egui::Id::new(("not_word_original_b_lockup", dark));
    if let Some(cached) = ctx.data(|data| data.get_temp::<Option<TextureHandle>>(id)) {
        return cached;
    }
    let bytes: &[u8] =
        if dark { include_bytes!("../../../docs/brand/not-word-logo-white.png") } else { include_bytes!("../../../docs/brand/not-word-logo.png") };
    let texture = match image::load_from_memory(bytes) {
        Ok(image) => {
            let pixels = image.to_rgba8();
            let size = [pixels.width() as usize, pixels.height() as usize];
            let image = egui::ColorImage::from_rgba_unmultiplied(size, pixels.as_raw());
            Some(ctx.load_texture(if dark { "Not Word logo reversed" } else { "Not Word logo" }, image, egui::TextureOptions::LINEAR))
        }
        Err(error) => {
            log::error!("embedded Not Word logo: {error}");
            None
        }
    };
    // Failed decoding is cached too: an invalid embedded asset must not cause repeated work.
    ctx.data_mut(|data| data.insert_temp(id, texture.clone()));
    texture
}

/// Show the complete approved symbol-and-wordmark, without tinting or adding another mark.
/// Transparent light/dark exports preserve the surrounding interface and the user's accents.
pub(crate) fn lockup(ui: &mut Ui, width: f32) {
    let dark = Tokens::get(ui.ctx()).dark;
    if let Some(texture) = texture(ui.ctx(), dark) {
        let source_size = texture.size_vec2();
        let width = width.min(ui.available_width()).max(1.0);
        let size = vec2(width, width * source_size.y / source_size.x.max(1.0));
        ui.add(egui::Image::from_texture(&texture).fit_to_exact_size(size).alt_text("Not Word"));
    } else {
        // Keep the product identifiable if a distribution contains an invalid image.
        ui.label(egui::RichText::new("Not Word").strong());
    }
}
