use eframe::{egui::{self, FontDefinitions, FontData, TextStyle}, epaint::{FontFamily, FontId}};

use super::{TangelloConfig, body2, heading2, heading3};

pub fn configure_fonts(config: &TangelloConfig,ctx: &egui::Context) {
    let mut fonts = FontDefinitions::default();
    let mut style = (*ctx.style()).clone();
    // Imports the MesloLGS font from its ttf file in order to enable support for other characters
    fonts.font_data.insert(
        "MesloLGS".to_owned(),
        FontData::from_static(include_bytes!("../../assets/MesloLGS_NF_Regular.ttf")),
    );
    fonts
        .families
        .get_mut(&FontFamily::Proportional)
        .unwrap()
        .push("MesloLGS".to_owned());

    // Sets font sizes for the different Text Styles.
    style.text_styles = [
        (TextStyle::Heading, FontId::new(35.0 * config.scale, FontFamily::Proportional)),
        (TextStyle::Body, FontId::new(20.0 * config.scale, FontFamily::Proportional)),
        (body2(), FontId::new(25.0 * config.scale, FontFamily::Proportional)),
        (heading2(), FontId::new(27.0 * config.scale, FontFamily::Proportional)),
        (heading3(), FontId::new(50.0 * config.scale, FontFamily::Proportional)),
        (TextStyle::Monospace,FontId::new(14.0 * config.scale, FontFamily::Proportional)),
        (TextStyle::Button,FontId::new(30.0 * config.scale, FontFamily::Proportional)),
        (TextStyle::Small,FontId::new(10.0 * config.scale, FontFamily::Proportional)),
    ].into();
    ctx.set_style(style);
    ctx.set_fonts(fonts);

}
