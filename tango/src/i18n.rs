pub const FALLBACK_LANG: unic_langid::LanguageIdentifier = unic_langid::langid!("zh-CN");
fluent_templates::static_loader! {
    pub static LOCALES = {
        locales: "./locales",
        fallback_language: "zh-CN",
    };
}
