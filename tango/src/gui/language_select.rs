use crate::gui;

pub fn show(ui: &mut egui::Ui, font_families: &gui::FontFamilies, language: &mut unic_langid::LanguageIdentifier) {
    let languages = &[
        (
            unic_langid::langid!("zh-CN"),
            egui::RichText::new("简体中文").family(font_families.hans.egui.clone()),
        ),
        (
            unic_langid::langid!("zh-TW"),
            egui::RichText::new("繁體中文").family(font_families.hant.egui.clone()),
        ),
    ];

    egui::ComboBox::from_id_source("settings-window-general-language")
        .width(200.0)
        .selected_text(
            languages
                .iter()
                .find(|(lang, _)| language.matches(lang, false, false))
                .map(|(_, label)| label.clone())
                .unwrap_or_else(|| egui::RichText::new("")),
        )
        .show_ui(ui, |ui| {
            for (lang, label) in languages.iter() {
                ui.selectable_value(language, lang.clone(), label.clone());
            }
        });
}
