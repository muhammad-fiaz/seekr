//! Demonstrates internationalization support.

use seekr::i18n::{Language, Translations};

fn main() {
    println!("Supported languages:");
    for lang in Language::all() {
        println!(
            "  {} ({}) - {}",
            lang.code(),
            lang.native_name(),
            lang.code()
        );
    }

    println!("\nTranslations for 'Files:':");
    for lang in Language::all() {
        let t = Translations::for_language(lang);
        println!("  {}: {}", lang.code(), t.files);
    }

    let detected = seekr::i18n::detect_locale();
    println!(
        "\nDetected locale: {} ({})",
        detected.code(),
        detected.native_name()
    );
}
