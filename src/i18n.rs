use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Supported languages.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Language {
    English,
    Spanish,
    French,
    German,
    Japanese,
    Chinese,
    Portuguese,
    Russian,
    Arabic,
    Hindi,
    Korean,
    Italian,
    Turkish,
    Polish,
    Dutch,
    Swedish,
    Thai,
    Vietnamese,
    Indonesian,
    Czech,
    Greek,
    Hebrew,
    Finnish,
    Norwegian,
    Danish,
    Hungarian,
    Romanian,
    Ukrainian,
    Bengali,
    Tamil,
}

impl Language {
    /// Returns the language code (ISO 639-1).
    pub fn code(&self) -> &str {
        match self {
            Language::English => "en",
            Language::Spanish => "es",
            Language::French => "fr",
            Language::German => "de",
            Language::Japanese => "ja",
            Language::Chinese => "zh",
            Language::Portuguese => "pt",
            Language::Russian => "ru",
            Language::Arabic => "ar",
            Language::Hindi => "hi",
            Language::Korean => "ko",
            Language::Italian => "it",
            Language::Turkish => "tr",
            Language::Polish => "pl",
            Language::Dutch => "nl",
            Language::Swedish => "sv",
            Language::Thai => "th",
            Language::Vietnamese => "vi",
            Language::Indonesian => "id",
            Language::Czech => "cs",
            Language::Greek => "el",
            Language::Hebrew => "he",
            Language::Finnish => "fi",
            Language::Norwegian => "no",
            Language::Danish => "da",
            Language::Hungarian => "hu",
            Language::Romanian => "ro",
            Language::Ukrainian => "uk",
            Language::Bengali => "bn",
            Language::Tamil => "ta",
        }
    }

    /// Returns the language name in its native script.
    pub fn native_name(&self) -> &str {
        match self {
            Language::English => "English",
            Language::Spanish => "Español",
            Language::French => "Français",
            Language::German => "Deutsch",
            Language::Japanese => "日本語",
            Language::Chinese => "中文",
            Language::Portuguese => "Português",
            Language::Russian => "Русский",
            Language::Arabic => "العربية",
            Language::Hindi => "हिन्दी",
            Language::Korean => "한국어",
            Language::Italian => "Italiano",
            Language::Turkish => "Türkçe",
            Language::Polish => "Polski",
            Language::Dutch => "Nederlands",
            Language::Swedish => "Svenska",
            Language::Thai => "ไทย",
            Language::Vietnamese => "Tiếng Việt",
            Language::Indonesian => "Bahasa Indonesia",
            Language::Czech => "Čeština",
            Language::Greek => "Ελληνικά",
            Language::Hebrew => "עברית",
            Language::Finnish => "Suomi",
            Language::Norwegian => "Norsk",
            Language::Danish => "Dansk",
            Language::Hungarian => "Magyar",
            Language::Romanian => "Română",
            Language::Ukrainian => "Українська",
            Language::Bengali => "বাংলা",
            Language::Tamil => "தமிழ்",
        }
    }

    /// Returns the English name of the language.
    pub fn english_name(&self) -> &str {
        match self {
            Language::English => "English",
            Language::Spanish => "Spanish",
            Language::French => "French",
            Language::German => "German",
            Language::Japanese => "Japanese",
            Language::Chinese => "Chinese",
            Language::Portuguese => "Portuguese",
            Language::Russian => "Russian",
            Language::Arabic => "Arabic",
            Language::Hindi => "Hindi",
            Language::Korean => "Korean",
            Language::Italian => "Italian",
            Language::Turkish => "Turkish",
            Language::Polish => "Polish",
            Language::Dutch => "Dutch",
            Language::Swedish => "Swedish",
            Language::Thai => "Thai",
            Language::Vietnamese => "Vietnamese",
            Language::Indonesian => "Indonesian",
            Language::Czech => "Czech",
            Language::Greek => "Greek",
            Language::Hebrew => "Hebrew",
            Language::Finnish => "Finnish",
            Language::Norwegian => "Norwegian",
            Language::Danish => "Danish",
            Language::Hungarian => "Hungarian",
            Language::Romanian => "Romanian",
            Language::Ukrainian => "Ukrainian",
            Language::Bengali => "Bengali",
            Language::Tamil => "Tamil",
        }
    }

    /// Returns all supported languages.
    pub fn all() -> Vec<Language> {
        vec![
            Language::English,
            Language::Spanish,
            Language::French,
            Language::German,
            Language::Japanese,
            Language::Chinese,
            Language::Portuguese,
            Language::Russian,
            Language::Arabic,
            Language::Hindi,
            Language::Korean,
            Language::Italian,
            Language::Turkish,
            Language::Polish,
            Language::Dutch,
            Language::Swedish,
            Language::Thai,
            Language::Vietnamese,
            Language::Indonesian,
            Language::Czech,
            Language::Greek,
            Language::Hebrew,
            Language::Finnish,
            Language::Norwegian,
            Language::Danish,
            Language::Hungarian,
            Language::Romanian,
            Language::Ukrainian,
            Language::Bengali,
            Language::Tamil,
        ]
    }

    /// Parses a language code into a Language.
    pub fn from_code(code: &str) -> Option<Language> {
        match code.to_lowercase().as_str() {
            "en" => Some(Language::English),
            "es" => Some(Language::Spanish),
            "fr" => Some(Language::French),
            "de" => Some(Language::German),
            "ja" => Some(Language::Japanese),
            "zh" => Some(Language::Chinese),
            "pt" => Some(Language::Portuguese),
            "ru" => Some(Language::Russian),
            "ar" => Some(Language::Arabic),
            "hi" => Some(Language::Hindi),
            "ko" => Some(Language::Korean),
            "it" => Some(Language::Italian),
            "tr" => Some(Language::Turkish),
            "pl" => Some(Language::Polish),
            "nl" => Some(Language::Dutch),
            "sv" => Some(Language::Swedish),
            "th" => Some(Language::Thai),
            "vi" => Some(Language::Vietnamese),
            "id" => Some(Language::Indonesian),
            "cs" => Some(Language::Czech),
            "el" => Some(Language::Greek),
            "he" => Some(Language::Hebrew),
            "fi" => Some(Language::Finnish),
            "no" => Some(Language::Norwegian),
            "da" => Some(Language::Danish),
            "hu" => Some(Language::Hungarian),
            "ro" => Some(Language::Romanian),
            "uk" => Some(Language::Ukrainian),
            "bn" => Some(Language::Bengali),
            "ta" => Some(Language::Tamil),
            _ => None,
        }
    }

    /// Parses a language name (English or native) into a Language.
    pub fn from_name(name: &str) -> Option<Language> {
        let lower = name.to_lowercase();
        match lower.as_str() {
            "english" | "en" => Some(Language::English),
            "spanish" | "español" | "es" => Some(Language::Spanish),
            "french" | "français" | "fr" => Some(Language::French),
            "german" | "deutsch" | "de" => Some(Language::German),
            "japanese" | "日本語" | "ja" => Some(Language::Japanese),
            "chinese" | "中文" | "zh" => Some(Language::Chinese),
            "portuguese" | "português" | "pt" => Some(Language::Portuguese),
            "russian" | "русский" | "ru" => Some(Language::Russian),
            "arabic" | "العربية" | "ar" => Some(Language::Arabic),
            "hindi" | "हिन्दी" | "hi" => Some(Language::Hindi),
            "korean" | "한국어" | "ko" => Some(Language::Korean),
            "italian" | "italiano" | "it" => Some(Language::Italian),
            "turkish" | "türkçe" | "tr" => Some(Language::Turkish),
            "polish" | "polski" | "pl" => Some(Language::Polish),
            "dutch" | "nederlands" | "nl" => Some(Language::Dutch),
            "swedish" | "svenska" | "sv" => Some(Language::Swedish),
            "thai" | "ไทย" | "th" => Some(Language::Thai),
            "vietnamese" | "tiếng việt" | "vi" => Some(Language::Vietnamese),
            "indonesian" | "bahasa indonesia" | "id" => Some(Language::Indonesian),
            "czech" | "čeština" | "cs" => Some(Language::Czech),
            "greek" | "ελληνικά" | "el" => Some(Language::Greek),
            "hebrew" | "עברית" | "he" => Some(Language::Hebrew),
            "finnish" | "suomi" | "fi" => Some(Language::Finnish),
            "norwegian" | "norsk" | "no" => Some(Language::Norwegian),
            "danish" | "dansk" | "da" => Some(Language::Danish),
            "hungarian" | "magyar" | "hu" => Some(Language::Hungarian),
            "romanian" | "română" | "ro" => Some(Language::Romanian),
            "ukrainian" | "українська" | "uk" => Some(Language::Ukrainian),
            "bengali" | "বাংলা" | "bn" => Some(Language::Bengali),
            "tamil" | "தமிழ்" | "ta" => Some(Language::Tamil),
            _ => None,
        }
    }
}

/// Translation strings for a language.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Translations {
    pub indexed_files: String,
    pub indexed_dirs: String,
    pub search_results: String,
    pub no_results: String,
    pub search_time: String,
    pub index_stats: String,
    pub files: String,
    pub directories: String,
    pub total_size: String,
    pub hidden_files: String,
    pub unique_extensions: String,
    pub last_indexed: String,
    pub root: String,
    pub error: String,
    pub not_found: String,
    pub permission_denied: String,
    pub indexing: String,
    pub watching: String,
    pub press_ctrl_c: String,
    pub created: String,
    pub modified: String,
    pub deleted: String,
    pub renamed: String,
    pub configuration: String,
    pub search_root: String,
    pub cache_enabled: String,
    pub cache_ttl: String,
    pub default_limit: String,
    pub color: String,
    pub database: String,
    pub running_diagnostics: String,
    pub all_checks_passed: String,
    pub removed_stale: String,
    pub no_stale: String,
    pub version: String,
    pub search: String,
    pub index: String,
    pub watch: String,
    pub stats: String,
    pub doctor: String,
    pub config: String,
    pub reindex: String,
    pub benchmark: String,
    pub open_file: String,
    pub open_dir: String,
    pub reveal: String,
    pub history: String,
    pub saved: String,
    pub analytics: String,
    pub grep: String,
    pub ml_search: String,
    pub semantic: String,
}

impl Translations {
    /// Returns translations for the given language.
    pub fn for_language(lang: Language) -> Self {
        match lang {
            Language::English => Self::english(),
            Language::Spanish => Self::spanish(),
            Language::French => Self::french(),
            Language::German => Self::german(),
            Language::Japanese => Self::japanese(),
            Language::Chinese => Self::chinese(),
            Language::Portuguese => Self::portuguese(),
            Language::Russian => Self::russian(),
            Language::Arabic => Self::arabic(),
            Language::Hindi => Self::hindi(),
            Language::Korean => Self::korean(),
            Language::Italian => Self::italian(),
            Language::Turkish => Self::turkish(),
            Language::Polish => Self::polish(),
            Language::Dutch => Self::dutch(),
            Language::Swedish => Self::swedish(),
            Language::Thai => Self::thai(),
            Language::Vietnamese => Self::vietnamese(),
            Language::Indonesian => Self::indonesian(),
            Language::Czech => Self::czech(),
            Language::Greek => Self::greek(),
            Language::Hebrew => Self::hebrew(),
            Language::Finnish => Self::finnish(),
            Language::Norwegian => Self::norwegian(),
            Language::Danish => Self::danish(),
            Language::Hungarian => Self::hungarian(),
            Language::Romanian => Self::romanian(),
            Language::Ukrainian => Self::ukrainian(),
            Language::Bengali => Self::bengali(),
            Language::Tamil => Self::tamil(),
        }
    }

    /// Creates translations from a custom HashMap.
    pub fn from_custom(translations: HashMap<String, String>) -> Self {
        let default = Self::english();
        Self {
            indexed_files: translations
                .get("indexed_files")
                .cloned()
                .unwrap_or(default.indexed_files),
            indexed_dirs: translations
                .get("indexed_dirs")
                .cloned()
                .unwrap_or(default.indexed_dirs),
            search_results: translations
                .get("search_results")
                .cloned()
                .unwrap_or(default.search_results),
            no_results: translations
                .get("no_results")
                .cloned()
                .unwrap_or(default.no_results),
            search_time: translations
                .get("search_time")
                .cloned()
                .unwrap_or(default.search_time),
            index_stats: translations
                .get("index_stats")
                .cloned()
                .unwrap_or(default.index_stats),
            files: translations.get("files").cloned().unwrap_or(default.files),
            directories: translations
                .get("directories")
                .cloned()
                .unwrap_or(default.directories),
            total_size: translations
                .get("total_size")
                .cloned()
                .unwrap_or(default.total_size),
            hidden_files: translations
                .get("hidden_files")
                .cloned()
                .unwrap_or(default.hidden_files),
            unique_extensions: translations
                .get("unique_extensions")
                .cloned()
                .unwrap_or(default.unique_extensions),
            last_indexed: translations
                .get("last_indexed")
                .cloned()
                .unwrap_or(default.last_indexed),
            root: translations.get("root").cloned().unwrap_or(default.root),
            error: translations.get("error").cloned().unwrap_or(default.error),
            not_found: translations
                .get("not_found")
                .cloned()
                .unwrap_or(default.not_found),
            permission_denied: translations
                .get("permission_denied")
                .cloned()
                .unwrap_or(default.permission_denied),
            indexing: translations
                .get("indexing")
                .cloned()
                .unwrap_or(default.indexing),
            watching: translations
                .get("watching")
                .cloned()
                .unwrap_or(default.watching),
            press_ctrl_c: translations
                .get("press_ctrl_c")
                .cloned()
                .unwrap_or(default.press_ctrl_c),
            created: translations
                .get("created")
                .cloned()
                .unwrap_or(default.created),
            modified: translations
                .get("modified")
                .cloned()
                .unwrap_or(default.modified),
            deleted: translations
                .get("deleted")
                .cloned()
                .unwrap_or(default.deleted),
            renamed: translations
                .get("renamed")
                .cloned()
                .unwrap_or(default.renamed),
            configuration: translations
                .get("configuration")
                .cloned()
                .unwrap_or(default.configuration),
            search_root: translations
                .get("search_root")
                .cloned()
                .unwrap_or(default.search_root),
            cache_enabled: translations
                .get("cache_enabled")
                .cloned()
                .unwrap_or(default.cache_enabled),
            cache_ttl: translations
                .get("cache_ttl")
                .cloned()
                .unwrap_or(default.cache_ttl),
            default_limit: translations
                .get("default_limit")
                .cloned()
                .unwrap_or(default.default_limit),
            color: translations.get("color").cloned().unwrap_or(default.color),
            database: translations
                .get("database")
                .cloned()
                .unwrap_or(default.database),
            running_diagnostics: translations
                .get("running_diagnostics")
                .cloned()
                .unwrap_or(default.running_diagnostics),
            all_checks_passed: translations
                .get("all_checks_passed")
                .cloned()
                .unwrap_or(default.all_checks_passed),
            removed_stale: translations
                .get("removed_stale")
                .cloned()
                .unwrap_or(default.removed_stale),
            no_stale: translations
                .get("no_stale")
                .cloned()
                .unwrap_or(default.no_stale),
            version: translations
                .get("version")
                .cloned()
                .unwrap_or(default.version),
            search: translations
                .get("search")
                .cloned()
                .unwrap_or(default.search),
            index: translations.get("index").cloned().unwrap_or(default.index),
            watch: translations.get("watch").cloned().unwrap_or(default.watch),
            stats: translations.get("stats").cloned().unwrap_or(default.stats),
            doctor: translations
                .get("doctor")
                .cloned()
                .unwrap_or(default.doctor),
            config: translations
                .get("config")
                .cloned()
                .unwrap_or(default.config),
            reindex: translations
                .get("reindex")
                .cloned()
                .unwrap_or(default.reindex),
            benchmark: translations
                .get("benchmark")
                .cloned()
                .unwrap_or(default.benchmark),
            open_file: translations
                .get("open_file")
                .cloned()
                .unwrap_or(default.open_file),
            open_dir: translations
                .get("open_dir")
                .cloned()
                .unwrap_or(default.open_dir),
            reveal: translations
                .get("reveal")
                .cloned()
                .unwrap_or(default.reveal),
            history: translations
                .get("history")
                .cloned()
                .unwrap_or(default.history),
            saved: translations.get("saved").cloned().unwrap_or(default.saved),
            analytics: translations
                .get("analytics")
                .cloned()
                .unwrap_or(default.analytics),
            grep: translations.get("grep").cloned().unwrap_or(default.grep),
            ml_search: translations
                .get("ml_search")
                .cloned()
                .unwrap_or(default.ml_search),
            semantic: translations
                .get("semantic")
                .cloned()
                .unwrap_or(default.semantic),
        }
    }

    fn english() -> Self {
        Self {
            indexed_files: "Indexed {} files".into(),
            indexed_dirs: "Indexed {} directories".into(),
            search_results: "Found {} results in {:.2?}".into(),
            no_results: "No results found.".into(),
            search_time: "Search completed in {:.2?}".into(),
            index_stats: "Index Statistics:".into(),
            files: "Files:".into(),
            directories: "Directories:".into(),
            total_size: "Total Size:".into(),
            hidden_files: "Hidden Files:".into(),
            unique_extensions: "Unique Extensions:".into(),
            last_indexed: "Last Indexed:".into(),
            root: "Root:".into(),
            error: "Error:".into(),
            not_found: "Path not found:".into(),
            permission_denied: "Permission denied:".into(),
            indexing: "Indexing:".into(),
            watching: "Watching:".into(),
            press_ctrl_c: "Press Ctrl+C to stop.".into(),
            created: "CREATED".into(),
            modified: "MODIFIED".into(),
            deleted: "DELETED".into(),
            renamed: "RENAMED".into(),
            configuration: "Configuration:".into(),
            search_root: "Search Root:".into(),
            cache_enabled: "Cache Enabled:".into(),
            cache_ttl: "Cache TTL:".into(),
            default_limit: "Default Limit:".into(),
            color: "Color:".into(),
            database: "Database:".into(),
            running_diagnostics: "Running diagnostics...".into(),
            all_checks_passed: "All checks passed".into(),
            removed_stale: "Removed {} stale entries".into(),
            no_stale: "No stale entries found".into(),
            version: "Version:".into(),
            search: "Search".into(),
            index: "Index".into(),
            watch: "Watch".into(),
            stats: "Stats".into(),
            doctor: "Doctor".into(),
            config: "Config".into(),
            reindex: "Reindex".into(),
            benchmark: "Benchmark".into(),
            open_file: "Open File".into(),
            open_dir: "Open Directory".into(),
            reveal: "Reveal".into(),
            history: "History".into(),
            saved: "Saved".into(),
            analytics: "Analytics".into(),
            grep: "Grep".into(),
            ml_search: "ML Search".into(),
            semantic: "Semantic".into(),
        }
    }

    fn spanish() -> Self {
        Self {
            indexed_files: "Indexados {} archivos".into(),
            indexed_dirs: "Indexados {} directorios".into(),
            search_results: "Encontrados {} resultados en {:.2?}".into(),
            no_results: "No se encontraron resultados.".into(),
            search_time: "Búsqueda completada en {:.2?}".into(),
            index_stats: "Estadísticas del índice:".into(),
            files: "Archivos:".into(),
            directories: "Directorios:".into(),
            total_size: "Tamaño total:".into(),
            hidden_files: "Archivos ocultos:".into(),
            unique_extensions: "Extensiones únicas:".into(),
            last_indexed: "Última indexación:".into(),
            root: "Raíz:".into(),
            error: "Error:".into(),
            not_found: "Ruta no encontrada:".into(),
            permission_denied: "Permiso denegado:".into(),
            indexing: "Indexando:".into(),
            watching: "Observando:".into(),
            press_ctrl_c: "Presiona Ctrl+C para detener.".into(),
            created: "CREADO".into(),
            modified: "MODIFICADO".into(),
            deleted: "ELIMINADO".into(),
            renamed: "RENOMBRADO".into(),
            configuration: "Configuración:".into(),
            search_root: "Raíz de búsqueda:".into(),
            cache_enabled: "Caché habilitado:".into(),
            cache_ttl: "TTL de caché:".into(),
            default_limit: "Límite predeterminado:".into(),
            color: "Color:".into(),
            database: "Base de datos:".into(),
            running_diagnostics: "Ejecutando diagnósticos...".into(),
            all_checks_passed: "Todas las verificaciones pasaron".into(),
            removed_stale: "Eliminados {} entradas obsoletas".into(),
            no_stale: "No se encontraron entradas obsoletas".into(),
            version: "Versión:".into(),
            search: "Buscar".into(),
            index: "Indexar".into(),
            watch: "Observar".into(),
            stats: "Estadísticas".into(),
            doctor: "Doctor".into(),
            config: "Configuración".into(),
            reindex: "Reindexar".into(),
            benchmark: "Benchmark".into(),
            open_file: "Abrir Archivo".into(),
            open_dir: "Abrir Directorio".into(),
            reveal: "Revelar".into(),
            history: "Historial".into(),
            saved: "Guardado".into(),
            analytics: "Analíticas".into(),
            grep: "Grep".into(),
            ml_search: "Búsqueda ML".into(),
            semantic: "Semántico".into(),
        }
    }

    fn french() -> Self {
        Self {
            indexed_files: "{} fichiers indexés".into(),
            indexed_dirs: "{} répertoires indexés".into(),
            search_results: "{} résultats trouvés en {:.2?}".into(),
            no_results: "Aucun résultat trouvé.".into(),
            search_time: "Recherche terminée en {:.2?}".into(),
            index_stats: "Statistiques de l'index:".into(),
            files: "Fichiers:".into(),
            directories: "Répertoires:".into(),
            total_size: "Taille totale:".into(),
            hidden_files: "Fichiers cachés:".into(),
            unique_extensions: "Extensions uniques:".into(),
            last_indexed: "Dernière indexation:".into(),
            root: "Racine:".into(),
            error: "Erreur:".into(),
            not_found: "Chemin non trouvé:".into(),
            permission_denied: "Permission refusée:".into(),
            indexing: "Indexation:".into(),
            watching: "Surveillance:".into(),
            press_ctrl_c: "Appuyez sur Ctrl+C pour arrêter.".into(),
            created: "CRÉÉ".into(),
            modified: "MODIFIÉ".into(),
            deleted: "SUPPRIMÉ".into(),
            renamed: "RENOMMÉ".into(),
            configuration: "Configuration:".into(),
            search_root: "Racine de recherche:".into(),
            cache_enabled: "Cache activé:".into(),
            cache_ttl: "TTL du cache:".into(),
            default_limit: "Limite par défaut:".into(),
            color: "Couleur:".into(),
            database: "Base de données:".into(),
            running_diagnostics: "Exécution des diagnostics...".into(),
            all_checks_passed: "Toutes les vérifications ont réussi".into(),
            removed_stale: "{} entrées obsolètes supprimées".into(),
            no_stale: "Aucune entrée obsolète trouvée".into(),
            version: "Version:".into(),
            search: "Rechercher".into(),
            index: "Indexer".into(),
            watch: "Surveiller".into(),
            stats: "Statistiques".into(),
            doctor: "Diagnostic".into(),
            config: "Configuration".into(),
            reindex: "Réindexer".into(),
            benchmark: "Benchmark".into(),
            open_file: "Ouvrir le Fichier".into(),
            open_dir: "Ouvrir le Répertoire".into(),
            reveal: "Révéler".into(),
            history: "Historique".into(),
            saved: "Enregistré".into(),
            analytics: "Analyses".into(),
            grep: "Grep".into(),
            ml_search: "Recherche ML".into(),
            semantic: "Sémantique".into(),
        }
    }

    fn german() -> Self {
        Self {
            indexed_files: "{} Dateien indiziert".into(),
            indexed_dirs: "{} Verzeichnisse indiziert".into(),
            search_results: "{} Ergebnisse gefunden in {:.2?}".into(),
            no_results: "Keine Ergebnisse gefunden.".into(),
            search_time: "Suche abgeschlossen in {:.2?}".into(),
            index_stats: "Indexstatistiken:".into(),
            files: "Dateien:".into(),
            directories: "Verzeichnisse:".into(),
            total_size: "Gesamtgröße:".into(),
            hidden_files: "Versteckte Dateien:".into(),
            unique_extensions: "Einzigartige Erweiterungen:".into(),
            last_indexed: "Zuletzt indiziert:".into(),
            root: "Stammverzeichnis:".into(),
            error: "Fehler:".into(),
            not_found: "Pfad nicht gefunden:".into(),
            permission_denied: "Zugriff verweigert:".into(),
            indexing: "Indizierung:".into(),
            watching: "Überwachung:".into(),
            press_ctrl_c: "Drücken Sie Strg+C zum Beenden.".into(),
            created: "ERSTELLT".into(),
            modified: "GEÄNDERT".into(),
            deleted: "GELÖSCHT".into(),
            renamed: "UMBENANNT".into(),
            configuration: "Konfiguration:".into(),
            search_root: "Suchstammverzeichnis:".into(),
            cache_enabled: "Cache aktiviert:".into(),
            cache_ttl: "Cache-TTL:".into(),
            default_limit: "Standardlimit:".into(),
            color: "Farbe:".into(),
            database: "Datenbank:".into(),
            running_diagnostics: "Diagnose läuft...".into(),
            all_checks_passed: "Alle Prüfungen bestanden".into(),
            removed_stale: "{} veraltete Einträge entfernt".into(),
            no_stale: "Keine veralteten Einträge gefunden".into(),
            version: "Version:".into(),
            search: "Suchen".into(),
            index: "Indexieren".into(),
            watch: "Überwachen".into(),
            stats: "Statistiken".into(),
            doctor: "Diagnose".into(),
            config: "Konfiguration".into(),
            reindex: "Neu indexieren".into(),
            benchmark: "Benchmark".into(),
            open_file: "Datei öffnen".into(),
            open_dir: "Verzeichnis öffnen".into(),
            reveal: "Anzeigen".into(),
            history: "Verlauf".into(),
            saved: "Gespeichert".into(),
            analytics: "Analysen".into(),
            grep: "Grep".into(),
            ml_search: "ML-Suche".into(),
            semantic: "Semantisch".into(),
        }
    }

    fn japanese() -> Self {
        Self {
            indexed_files: "{} ファイルをインデックス".into(),
            indexed_dirs: "{} ディレクトリをインデックス".into(),
            search_results: "{} 件の結果 ({:.2?})".into(),
            no_results: "結果が見つかりません。".into(),
            search_time: "検索完了: {:.2?}".into(),
            index_stats: "インデックス統計:".into(),
            files: "ファイル:".into(),
            directories: "ディレクトリ:".into(),
            total_size: "合計サイズ:".into(),
            hidden_files: "隠しファイル:".into(),
            unique_extensions: "ユニーク拡張子:".into(),
            last_indexed: "最終インデックス:".into(),
            root: "ルート:".into(),
            error: "エラー:".into(),
            not_found: "パスが見つかりません:".into(),
            permission_denied: "権限がありません:".into(),
            indexing: "インデックス中:".into(),
            watching: "監視中:".into(),
            press_ctrl_c: "Ctrl+C で停止。".into(),
            created: "作成".into(),
            modified: "変更".into(),
            deleted: "削除".into(),
            renamed: "名前変更".into(),
            configuration: "設定:".into(),
            search_root: "検索ルート:".into(),
            cache_enabled: "キャッシュ有効:".into(),
            cache_ttl: "キャッシュTTL:".into(),
            default_limit: "デフォルト制限:".into(),
            color: "色:".into(),
            database: "データベース:".into(),
            running_diagnostics: "診断実行中...".into(),
            all_checks_passed: "すべてのチェック合格".into(),
            removed_stale: "{} 件の古いエントリを削除".into(),
            no_stale: "古いエントリはありません".into(),
            version: "バージョン:".into(),
            search: "検索".into(),
            index: "インデックス".into(),
            watch: "監視".into(),
            stats: "統計".into(),
            doctor: "診断".into(),
            config: "設定".into(),
            reindex: "再インデックス".into(),
            benchmark: "ベンチマーク".into(),
            open_file: "ファイルを開く".into(),
            open_dir: "ディレクトリを開く".into(),
            reveal: "表示".into(),
            history: "履歴".into(),
            saved: "保存済み".into(),
            analytics: "分析".into(),
            grep: "グレップ".into(),
            ml_search: "ML検索".into(),
            semantic: "意味的".into(),
        }
    }

    fn chinese() -> Self {
        Self {
            indexed_files: "已索引 {} 个文件".into(),
            indexed_dirs: "已索引 {} 个目录".into(),
            search_results: "找到 {} 个结果 ({:.2?})".into(),
            no_results: "未找到结果。".into(),
            search_time: "搜索完成: {:.2?}".into(),
            index_stats: "索引统计:".into(),
            files: "文件:".into(),
            directories: "目录:".into(),
            total_size: "总大小:".into(),
            hidden_files: "隐藏文件:".into(),
            unique_extensions: "唯一扩展名:".into(),
            last_indexed: "最后索引:".into(),
            root: "根目录:".into(),
            error: "错误:".into(),
            not_found: "路径未找到:".into(),
            permission_denied: "权限被拒绝:".into(),
            indexing: "索引中:".into(),
            watching: "监视中:".into(),
            press_ctrl_c: "按 Ctrl+C 停止。".into(),
            created: "已创建".into(),
            modified: "已修改".into(),
            deleted: "已删除".into(),
            renamed: "已重命名".into(),
            configuration: "配置:".into(),
            search_root: "搜索根目录:".into(),
            cache_enabled: "缓存已启用:".into(),
            cache_ttl: "缓存TTL:".into(),
            default_limit: "默认限制:".into(),
            color: "颜色:".into(),
            database: "数据库:".into(),
            running_diagnostics: "运行诊断...".into(),
            all_checks_passed: "所有检查通过".into(),
            removed_stale: "已删除 {} 个过期条目".into(),
            no_stale: "未找到过期条目".into(),
            version: "版本:".into(),
            search: "搜索".into(),
            index: "索引".into(),
            watch: "监视".into(),
            stats: "统计".into(),
            doctor: "诊断".into(),
            config: "配置".into(),
            reindex: "重建索引".into(),
            benchmark: "基准测试".into(),
            open_file: "打开文件".into(),
            open_dir: "打开目录".into(),
            reveal: "显示".into(),
            history: "历史".into(),
            saved: "已保存".into(),
            analytics: "分析".into(),
            grep: " grep".into(),
            ml_search: "ML搜索".into(),
            semantic: "语义".into(),
        }
    }

    fn portuguese() -> Self {
        Self {
            indexed_files: "{} arquivos indexados".into(),
            indexed_dirs: "{} diretórios indexados".into(),
            search_results: "{} resultados encontrados em {:.2?}".into(),
            no_results: "Nenhum resultado encontrado.".into(),
            search_time: "Busca concluída em {:.2?}".into(),
            index_stats: "Estatísticas do índice:".into(),
            files: "Arquivos:".into(),
            directories: "Diretórios:".into(),
            total_size: "Tamanho total:".into(),
            hidden_files: "Arquivos ocultos:".into(),
            unique_extensions: "Extensões únicas:".into(),
            last_indexed: "Última indexação:".into(),
            root: "Raiz:".into(),
            error: "Erro:".into(),
            not_found: "Caminho não encontrado:".into(),
            permission_denied: "Permissão negada:".into(),
            indexing: "Indexando:".into(),
            watching: "Observando:".into(),
            press_ctrl_c: "Pressione Ctrl+C para parar.".into(),
            created: "CRIADO".into(),
            modified: "MODIFICADO".into(),
            deleted: "EXCLUÍDO".into(),
            renamed: "RENOMEADO".into(),
            configuration: "Configuração:".into(),
            search_root: "Raiz de busca:".into(),
            cache_enabled: "Cache habilitado:".into(),
            cache_ttl: "TTL do cache:".into(),
            default_limit: "Limite padrão:".into(),
            color: "Cor:".into(),
            database: "Banco de dados:".into(),
            running_diagnostics: "Executando diagnósticos...".into(),
            all_checks_passed: "Todas as verificações passaram".into(),
            removed_stale: "{} entradas obsoletas removidas".into(),
            no_stale: "Nenhuma entrada obsoleta encontrada".into(),
            version: "Versão:".into(),
            search: "Pesquisar".into(),
            index: "Indexar".into(),
            watch: "Observar".into(),
            stats: "Estatísticas".into(),
            doctor: "Diagnóstico".into(),
            config: "Configuração".into(),
            reindex: "Reindexar".into(),
            benchmark: "Benchmark".into(),
            open_file: "Abrir Arquivo".into(),
            open_dir: "Abrir Diretório".into(),
            reveal: "Revelar".into(),
            history: "Histórico".into(),
            saved: "Salvo".into(),
            analytics: "Análises".into(),
            grep: "Grep".into(),
            ml_search: "Busca ML".into(),
            semantic: "Semântico".into(),
        }
    }

    fn russian() -> Self {
        Self {
            indexed_files: "Проиндексировано {} файлов".into(),
            indexed_dirs: "Проиндексировано {} каталогов".into(),
            search_results: "Найдено {} результатов за {:.2?}".into(),
            no_results: "Результаты не найдены.".into(),
            search_time: "Поиск завершен за {:.2?}".into(),
            index_stats: "Статистика индекса:".into(),
            files: "Файлы:".into(),
            directories: "Каталоги:".into(),
            total_size: "Общий размер:".into(),
            hidden_files: "Скрытые файлы:".into(),
            unique_extensions: "Уникальные расширения:".into(),
            last_indexed: "Последняя индексация:".into(),
            root: "Корень:".into(),
            error: "Ошибка:".into(),
            not_found: "Путь не найден:".into(),
            permission_denied: "Доступ запрещен:".into(),
            indexing: "Индексация:".into(),
            watching: "Наблюдение:".into(),
            press_ctrl_c: "Нажмите Ctrl+C для остановки.".into(),
            created: "СОЗДАН".into(),
            modified: "ИЗМЕНЕН".into(),
            deleted: "УДАЛЕН".into(),
            renamed: "ПЕРЕИМЕНОВАН".into(),
            configuration: "Конфигурация:".into(),
            search_root: "Корневая директория поиска:".into(),
            cache_enabled: "Кэш включен:".into(),
            cache_ttl: "TTL кэша:".into(),
            default_limit: "Лимит по умолчанию:".into(),
            color: "Цвет:".into(),
            database: "База данных:".into(),
            running_diagnostics: "Запуск диагностики...".into(),
            all_checks_passed: "Все проверки пройдены".into(),
            removed_stale: "Удалено {} устаревших записей".into(),
            no_stale: "Устаревшие записи не найдены".into(),
            version: "Версия:".into(),
            search: "Поиск".into(),
            index: "Индексация".into(),
            watch: "Наблюдение".into(),
            stats: "Статистика".into(),
            doctor: "Диагностика".into(),
            config: "Конфигурация".into(),
            reindex: "Переиндексация".into(),
            benchmark: "Тест производительности".into(),
            open_file: "Открыть файл".into(),
            open_dir: "Открыть директорию".into(),
            reveal: "Показать".into(),
            history: "История".into(),
            saved: "Сохранено".into(),
            analytics: "Аналитика".into(),
            grep: "Grep".into(),
            ml_search: "ML поиск".into(),
            semantic: "Семантический".into(),
        }
    }

    fn arabic() -> Self {
        Self {
            indexed_files: "تم فهرسة {} ملف".into(),
            indexed_dirs: "تم فهرسة {} دليل".into(),
            search_results: "وجدت {} نتائج في {:.2?}".into(),
            no_results: "لم يتم العثور على نتائج.".into(),
            search_time: "اكتمل البحث في {:.2?}".into(),
            index_stats: "إحصائيات الفهرس:".into(),
            files: "الملفات:".into(),
            directories: "الدلائل:".into(),
            total_size: "الحجم الإجمالي:".into(),
            hidden_files: "الملفات المخفية:".into(),
            unique_extensions: "الامتدادات الفريدة:".into(),
            last_indexed: "آخر فهرسة:".into(),
            root: "الجذر:".into(),
            error: "خطأ:".into(),
            not_found: "المسار غير موجود:".into(),
            permission_denied: "تم رفض الإذن:".into(),
            indexing: "الفهرسة:".into(),
            watching: "المراقبة:".into(),
            press_ctrl_c: "اضغط Ctrl+C للتوقف.".into(),
            created: "تم الإنشاء".into(),
            modified: "تم التعديل".into(),
            deleted: "تم الحذف".into(),
            renamed: "تمت إعادة التسمية".into(),
            configuration: "الإعدادات:".into(),
            search_root: "جذر البحث:".into(),
            cache_enabled: "الذاكرة مؤقتة مفعلة:".into(),
            cache_ttl: "مدة الذاكرة المؤقتة:".into(),
            default_limit: "الحد الافتراضي:".into(),
            color: "اللون:".into(),
            database: "قاعدة البيانات:".into(),
            running_diagnostics: "تشغيل التشخيص...".into(),
            all_checks_passed: "تم اجتياز جميع الفحوصات".into(),
            removed_stale: "تم إزالة {} سجلات قديمة".into(),
            no_stale: "لم يتم العثور على سجلات قديمة".into(),
            version: "الإصدار:".into(),
            search: "بحث".into(),
            index: "فهرسة".into(),
            watch: "مراقبة".into(),
            stats: "إحصائيات".into(),
            doctor: "تشخيص".into(),
            config: "إعدادات".into(),
            reindex: "إعادة الفهرسة".into(),
            benchmark: "اختبار الأداء".into(),
            open_file: "فتح ملف".into(),
            open_dir: "فتح دليل".into(),
            reveal: "إظهار".into(),
            history: "التاريخ".into(),
            saved: "محفوظ".into(),
            analytics: "تحليلات".into(),
            grep: "بحث نصي".into(),
            ml_search: "بحث ML".into(),
            semantic: "دلالي".into(),
        }
    }

    fn hindi() -> Self {
        Self {
            indexed_files: "{} फ़ाइलें अनुक्रमित".into(),
            indexed_dirs: "{} निर्देशिकाएँ अनुक्रमित".into(),
            search_results: "{} परिणाम मिले ({:.2?})".into(),
            no_results: "कोई परिणाम नहीं मिला।".into(),
            search_time: "खोज {:.2?} में पूर्ण".into(),
            index_stats: "अनुक्रमण सांख्यिकी:".into(),
            files: "फ़ाइलें:".into(),
            directories: "निर्देशिकाएँ:".into(),
            total_size: "कुल आकार:".into(),
            hidden_files: "छिपी फ़ाइलें:".into(),
            unique_extensions: "अद्वितीय एक्सटेंशन:".into(),
            last_indexed: "अंतिम अनुक्रमण:".into(),
            root: "मूल:".into(),
            error: "त्रुटि:".into(),
            not_found: "पथ नहीं मिला:".into(),
            permission_denied: "अनुमति अस्वीकृत:".into(),
            indexing: "अनुक्रमण:".into(),
            watching: "निगरानी:".into(),
            press_ctrl_c: "रोकने के लिए Ctrl+C दबाएं।".into(),
            created: "बनाया गया".into(),
            modified: "संशोधित".into(),
            deleted: "हटाया गया".into(),
            renamed: "पुनर्नामित".into(),
            configuration: "कॉन्फ़िगरेशन:".into(),
            search_root: "खोल मूल:".into(),
            cache_enabled: "कैश सक्षम:".into(),
            cache_ttl: "कैश TTL:".into(),
            default_limit: "डिफ़ॉल्ट सीमा:".into(),
            color: "रंग:".into(),
            database: "डेटाबेस:".into(),
            running_diagnostics: "निदान चल रहा है...".into(),
            all_checks_passed: "सभी जांच पास".into(),
            removed_stale: "{} पुराने प्रविष्टियाँ हटाई गईं".into(),
            no_stale: "कोई पुरानी प्रविष्टि नहीं मिली".into(),
            version: "संस्करण:".into(),
            search: "खोजें".into(),
            index: "अनुक्रमित करें".into(),
            watch: "देखें".into(),
            stats: "सांख्यिकी".into(),
            doctor: "निदान".into(),
            config: "कॉन्फ़िगरेशन".into(),
            reindex: "पुनः अनुक्रमित करें".into(),
            benchmark: "बेंचमार्क".into(),
            open_file: "फ़ाइल खोलें".into(),
            open_dir: "निर्देशिका खोलें".into(),
            reveal: "दिखाएं".into(),
            history: "इतिहास".into(),
            saved: "सहेजा गया".into(),
            analytics: "विश्लेषण".into(),
            grep: "खोजें".into(),
            ml_search: "ML खोज".into(),
            semantic: "अर्थात्मक".into(),
        }
    }

    fn korean() -> Self {
        Self {
            indexed_files: "{} 파일 인덱싱됨".into(),
            indexed_dirs: "{} 디렉토리 인덱싱됨".into(),
            search_results: "{} 개 결과 발견 ({:.2?})".into(),
            no_results: "결과를 찾을 수 없습니다.".into(),
            search_time: "검색 완료: {:.2?}".into(),
            index_stats: "인덱스 통계:".into(),
            files: "파일:".into(),
            directories: "디렉토리:".into(),
            total_size: "총 크기:".into(),
            hidden_files: "숨김 파일:".into(),
            unique_extensions: "고유 확장자:".into(),
            last_indexed: "마지막 인덱싱:".into(),
            root: "루트:".into(),
            error: "오류:".into(),
            not_found: "경로를 찾을 수 없음:".into(),
            permission_denied: "권한 거부됨:".into(),
            indexing: "인덱싱 중:".into(),
            watching: "감시 중:".into(),
            press_ctrl_c: "Ctrl+C로 중지.".into(),
            created: "생성됨".into(),
            modified: "수정됨".into(),
            deleted: "삭제됨".into(),
            renamed: "이름 변경됨".into(),
            configuration: "설정:".into(),
            search_root: "검색 루트:".into(),
            cache_enabled: "캐시 활성화:".into(),
            cache_ttl: "캐시 TTL:".into(),
            default_limit: "기본 제한:".into(),
            color: "색상:".into(),
            database: "데이터베이스:".into(),
            running_diagnostics: "진단 실행 중...".into(),
            all_checks_passed: "모든 검사 통과".into(),
            removed_stale: "{}개 오래된 항목 제거됨".into(),
            no_stale: "오래된 항목 없음".into(),
            version: "버전:".into(),
            search: "검색".into(),
            index: "인덱스".into(),
            watch: "감시".into(),
            stats: "통계".into(),
            doctor: "진단".into(),
            config: "설정".into(),
            reindex: "재인덱싱".into(),
            benchmark: "벤치마크".into(),
            open_file: "파일 열기".into(),
            open_dir: "디렉토리 열기".into(),
            reveal: "표시".into(),
            history: "기록".into(),
            saved: "저장됨".into(),
            analytics: "분석".into(),
            grep: "그렙".into(),
            ml_search: "ML 검색".into(),
            semantic: "의미적".into(),
        }
    }

    fn italian() -> Self {
        Self {
            indexed_files: "{} file indicizzati".into(),
            indexed_dirs: "{} directory indicizzate".into(),
            search_results: "{} risultati trovati in {:.2?}".into(),
            no_results: "Nessun risultato trovato.".into(),
            search_time: "Ricerca completata in {:.2?}".into(),
            index_stats: "Statistiche indice:".into(),
            files: "File:".into(),
            directories: "Directory:".into(),
            total_size: "Dimensione totale:".into(),
            hidden_files: "File nascosti:".into(),
            unique_extensions: "Estensioni uniche:".into(),
            last_indexed: "Ultimo indicizzazione:".into(),
            root: "Root:".into(),
            error: "Errore:".into(),
            not_found: "Percorso non trovato:".into(),
            permission_denied: "Permesso negato:".into(),
            indexing: "Indicizzazione:".into(),
            watching: "Monitoraggio:".into(),
            press_ctrl_c: "Premi Ctrl+C per fermare.".into(),
            created: "CREATO".into(),
            modified: "MODIFICATO".into(),
            deleted: "ELIMINATO".into(),
            renamed: "RINOMINATO".into(),
            configuration: "Configurazione:".into(),
            search_root: "Root di ricerca:".into(),
            cache_enabled: "Cache abilitata:".into(),
            cache_ttl: "TTL cache:".into(),
            default_limit: "Limite predefinito:".into(),
            color: "Colore:".into(),
            database: "Database:".into(),
            running_diagnostics: "Esecuzione diagnosi...".into(),
            all_checks_passed: "Tutti i controlli superati".into(),
            removed_stale: "{} voci obsolete rimosse".into(),
            no_stale: "Nessuna voce obsoleta trovata".into(),
            version: "Versione:".into(),
            search: "Cerca".into(),
            index: "Indicizza".into(),
            watch: "Monitora".into(),
            stats: "Statistiche".into(),
            doctor: "Diagnosi".into(),
            config: "Configurazione".into(),
            reindex: "Re-indicizza".into(),
            benchmark: "Benchmark".into(),
            open_file: "Apri File".into(),
            open_dir: "Apri Directory".into(),
            reveal: "Mostra".into(),
            history: "Cronologia".into(),
            saved: "Salvato".into(),
            analytics: "Analisi".into(),
            grep: "Grep".into(),
            ml_search: "Ricerca ML".into(),
            semantic: "Semantico".into(),
        }
    }

    fn turkish() -> Self {
        Self {
            indexed_files: "{} dosya indekslendi".into(),
            indexed_dirs: "{} dizin indekslendi".into(),
            search_results: "{} sonuç bulundu ({:.2?})".into(),
            no_results: "Sonuç bulunamadı.".into(),
            search_time: "Arama {:.2?} süresinde tamamlandı".into(),
            index_stats: "İndeks İstatistikleri:".into(),
            files: "Dosyalar:".into(),
            directories: "Dizinler:".into(),
            total_size: "Toplam Boyut:".into(),
            hidden_files: "Gizli Dosyalar:".into(),
            unique_extensions: "Benzersiz Uzantılar:".into(),
            last_indexed: "Son İndeksleme:".into(),
            root: "Kök:".into(),
            error: "Hata:".into(),
            not_found: "Yol bulunamadı:".into(),
            permission_denied: "İzin reddedildi:".into(),
            indexing: "İndeksleme:".into(),
            watching: "İzleme:".into(),
            press_ctrl_c: "Durdurmak için Ctrl+C'ye basın.".into(),
            created: "OLUŞTURULDU".into(),
            modified: "DEĞİŞTİRİLDİ".into(),
            deleted: "SİLİNDİ".into(),
            renamed: "YENİDEN ADLANDIRILDI".into(),
            configuration: "Yapılandırma:".into(),
            search_root: "Arama Kökü:".into(),
            cache_enabled: "Önbellek Etkin:".into(),
            cache_ttl: "Önbellek TTL:".into(),
            default_limit: "Varsayılan Limit:".into(),
            color: "Renk:".into(),
            database: "Veritabanı:".into(),
            running_diagnostics: "Tanılama çalışıyor...".into(),
            all_checks_passed: "Tüm kontroller geçildi".into(),
            removed_stale: "{} eski girdi kaldırıldı".into(),
            no_stale: "Eski girdi bulunamadı".into(),
            version: "Sürüm:".into(),
            search: "Ara".into(),
            index: "İndeksle".into(),
            watch: "İzle".into(),
            stats: "İstatistikler".into(),
            doctor: "Tanılama".into(),
            config: "Yapılandırma".into(),
            reindex: "Yeniden İndeksle".into(),
            benchmark: "Karşılaştırma".into(),
            open_file: "Dosya Aç".into(),
            open_dir: "Dizin Aç".into(),
            reveal: "Göster".into(),
            history: "Geçmiş".into(),
            saved: "Kaydedildi".into(),
            analytics: "Analiz".into(),
            grep: "Grep".into(),
            ml_search: "ML Arama".into(),
            semantic: "Anlamsal".into(),
        }
    }

    fn polish() -> Self {
        Self {
            indexed_files: "Zaindeksowano {} plików".into(),
            indexed_dirs: "Zaindeksowano {} katalogów".into(),
            search_results: "Znaleziono {} wyników w {:.2?}".into(),
            no_results: "Nie znaleziono wyników.".into(),
            search_time: "Wyszukiwanie zakończone w {:.2?}".into(),
            index_stats: "Statystyki indeksu:".into(),
            files: "Pliki:".into(),
            directories: "Katalogi:".into(),
            total_size: "Całkowity rozmiar:".into(),
            hidden_files: "Ukryte pliki:".into(),
            unique_extensions: "Unikalne rozszerzenia:".into(),
            last_indexed: "Ostatnia indeksacja:".into(),
            root: "Korzeń:".into(),
            error: "Błąd:".into(),
            not_found: "Nie znaleziono ścieżki:".into(),
            permission_denied: "Odmowa dostępu:".into(),
            indexing: "Indeksacja:".into(),
            watching: "Nadzór:".into(),
            press_ctrl_c: "Naciśnij Ctrl+C, aby zatrzymać.".into(),
            created: "UTWORZONO".into(),
            modified: "ZMODYFIKOWANO".into(),
            deleted: "USUNIĘTO".into(),
            renamed: "PRZENAZWANO".into(),
            configuration: "Konfiguracja:".into(),
            search_root: "Katalog główny wyszukiwania:".into(),
            cache_enabled: "Pamięć podręczna włączona:".into(),
            cache_ttl: "TTL pamięci podręcznej:".into(),
            default_limit: "Domyślny limit:".into(),
            color: "Kolor:".into(),
            database: "Baza danych:".into(),
            running_diagnostics: "Uruchamianie diagnostyki...".into(),
            all_checks_passed: "Wszystkie kontrole przeszły".into(),
            removed_stale: "Usunięto {} przestarzałych wpisów".into(),
            no_stale: "Nie znaleziono przestarzałych wpisów".into(),
            version: "Wersja:".into(),
            search: "Szukaj".into(),
            index: "Indeksuj".into(),
            watch: "Nadzoruj".into(),
            stats: "Statystyki".into(),
            doctor: "Diagnostyka".into(),
            config: "Konfiguracja".into(),
            reindex: "Reindeksuj".into(),
            benchmark: "Benchmark".into(),
            open_file: "Otwórz plik".into(),
            open_dir: "Otwórz katalog".into(),
            reveal: "Pokaż".into(),
            history: "Historia".into(),
            saved: "Zapisano".into(),
            analytics: "Analityka".into(),
            grep: "Grep".into(),
            ml_search: "Wyszukiwanie ML".into(),
            semantic: "Semantyczne".into(),
        }
    }

    fn dutch() -> Self {
        Self {
            indexed_files: "{} bestanden geïndexeerd".into(),
            indexed_dirs: "{} mappen geïndexeerd".into(),
            search_results: "{} resultaten gevonden in {:.2?}".into(),
            no_results: "Geen resultaten gevonden.".into(),
            search_time: "Zoeking voltooid in {:.2?}".into(),
            index_stats: "Index Statistieken:".into(),
            files: "Bestanden:".into(),
            directories: "Mappen:".into(),
            total_size: "Totale grootte:".into(),
            hidden_files: "Verborgen bestanden:".into(),
            unique_extensions: "Unieke extensies:".into(),
            last_indexed: "Laatst geïndexeerd:".into(),
            root: "Root:".into(),
            error: "Fout:".into(),
            not_found: "Pad niet gevonden:".into(),
            permission_denied: "Toegang geweigerd:".into(),
            indexing: "Indexering:".into(),
            watching: "Bewaking:".into(),
            press_ctrl_c: "Druk op Ctrl+C om te stoppen.".into(),
            created: "AANGEMAAKT".into(),
            modified: "GEWIJZIGD".into(),
            deleted: "VERWIJDERD".into(),
            renamed: "HERGEDEELD".into(),
            configuration: "Configuratie:".into(),
            search_root: "Zoekroot:".into(),
            cache_enabled: "Cache ingeschakeld:".into(),
            cache_ttl: "Cache TTL:".into(),
            default_limit: "Standaard limiet:".into(),
            color: "Kleur:".into(),
            database: "Database:".into(),
            running_diagnostics: "Diagnostiek uitvoeren...".into(),
            all_checks_passed: "Alle controles geslaagd".into(),
            removed_stale: "{} verouderde items verwijderd".into(),
            no_stale: "Geen verouderde items gevonden".into(),
            version: "Versie:".into(),
            search: "Zoeken".into(),
            index: "Indexeren".into(),
            watch: "Bewaken".into(),
            stats: "Statistieken".into(),
            doctor: "Diagnostiek".into(),
            config: "Configuratie".into(),
            reindex: "Opnieuw indexeren".into(),
            benchmark: "Benchmark".into(),
            open_file: "Bestand openen".into(),
            open_dir: "Map openen".into(),
            reveal: "Weergeven".into(),
            history: "Geschiedenis".into(),
            saved: "Opgeslagen".into(),
            analytics: "Analyses".into(),
            grep: "Grep".into(),
            ml_search: "ML Zoeken".into(),
            semantic: "Semantisch".into(),
        }
    }

    fn swedish() -> Self {
        Self {
            indexed_files: "{} filer indexerade".into(),
            indexed_dirs: "{} kataloger indexerade".into(),
            search_results: "{} resultat hittade på {:.2?}".into(),
            no_results: "Inga resultat hittades.".into(),
            search_time: "Sökning slutförd på {:.2?}".into(),
            index_stats: "Indexstatistik:".into(),
            files: "Filer:".into(),
            directories: "Kataloger:".into(),
            total_size: "Total storlek:".into(),
            hidden_files: "Dolda filer:".into(),
            unique_extensions: "Unika tillägg:".into(),
            last_indexed: "Senast indexerad:".into(),
            root: "Root:".into(),
            error: "Fel:".into(),
            not_found: "Sökväg hittades inte:".into(),
            permission_denied: "Behörighet nekad:".into(),
            indexing: "Indexering:".into(),
            watching: "Övervakning:".into(),
            press_ctrl_c: "Tryck på Ctrl+C för att stoppa.".into(),
            created: "SKAPAD".into(),
            modified: "ÄNDRAD".into(),
            deleted: "BORTTAGEN".into(),
            renamed: "OMDÖPT".into(),
            configuration: "Konfiguration:".into(),
            search_root: "Sökroot:".into(),
            cache_enabled: "Cache aktiverad:".into(),
            cache_ttl: "Cache TTL:".into(),
            default_limit: "Standardgräns:".into(),
            color: "Färg:".into(),
            database: "Databas:".into(),
            running_diagnostics: "Kör diagnostik...".into(),
            all_checks_passed: "Alla kontroller godkända".into(),
            removed_stale: "{} föråldrade poster borttagna".into(),
            no_stale: "Inga föråldrade poster hittades".into(),
            version: "Version:".into(),
            search: "Sök".into(),
            index: "Indexera".into(),
            watch: "Övervaka".into(),
            stats: "Statistik".into(),
            doctor: "Diagnostik".into(),
            config: "Konfiguration".into(),
            reindex: "Omindexera".into(),
            benchmark: "Benchmark".into(),
            open_file: "Öppna fil".into(),
            open_dir: "Öppna katalog".into(),
            reveal: "Visa".into(),
            history: "Historik".into(),
            saved: "Sparad".into(),
            analytics: "Analys".into(),
            grep: "Grep".into(),
            ml_search: "ML-sökning".into(),
            semantic: "Semantisk".into(),
        }
    }

    fn thai() -> Self {
        Self {
            indexed_files: "ทำดัชนี {} ไฟล์แล้ว".into(),
            indexed_dirs: "ทำดัชนี {} ไดเรกทอรีแล้ว".into(),
            search_results: "พบ {} ผลลัพธ์ใน {:.2?}".into(),
            no_results: "ไม่พบผลลัพธ์".into(),
            search_time: "การค้นหาเสร็จสิ้นใน {:.2?}".into(),
            index_stats: "สถิติดัชนี:".into(),
            files: "ไฟล์:".into(),
            directories: "ไดเรกทอรี:".into(),
            total_size: "ขนาดรวม:".into(),
            hidden_files: "ไฟล์ที่ซ่อน:".into(),
            unique_extensions: "นามสกุลที่ไม่ซ้ำ:".into(),
            last_indexed: "ดัชนีล่าสุด:".into(),
            root: "รูท:".into(),
            error: "ข้อผิดพลาด:".into(),
            not_found: "ไม่พบเส้นทาง:".into(),
            permission_denied: "สิทธิ์ถูกปฏิเสธ:".into(),
            indexing: "การดัชนี:".into(),
            watching: "การเฝ้าระวัง:".into(),
            press_ctrl_c: "กด Ctrl+C เพื่อหยุด".into(),
            created: "สร้างแล้ว".into(),
            modified: "แก้ไขแล้ว".into(),
            deleted: "ลบแล้ว".into(),
            renamed: "เปลี่ยนชื่อแล้ว".into(),
            configuration: "การกำหนดค่า:".into(),
            search_root: "รูทการค้นหา:".into(),
            cache_enabled: "แคชเปิดใช้งาน:".into(),
            cache_ttl: "แคช TTL:".into(),
            default_limit: "ขีดจำกัดเริ่มต้น:".into(),
            color: "สี:".into(),
            database: "ฐานข้อมูล:".into(),
            running_diagnostics: "กำลังวินิจฉัย...".into(),
            all_checks_passed: "ผ่านการตรวจสอบทั้งหมด".into(),
            removed_stale: "ลบ {} รายการที่ล้าสมัย".into(),
            no_stale: "ไม่พบรายการที่ล้าสมัย".into(),
            version: "เวอร์ชัน:".into(),
            search: "ค้นหา".into(),
            index: "ดัชนี".into(),
            watch: "เฝ้าระวัง".into(),
            stats: "สถิติ".into(),
            doctor: "วินิจฉัย".into(),
            config: "การกำหนดค่า".into(),
            reindex: "สร้างดัชนีใหม่".into(),
            benchmark: "เกณฑ์มาตรฐาน".into(),
            open_file: "เปิดไฟล์".into(),
            open_dir: "เปิดไดเรกทอรี".into(),
            reveal: "แสดง".into(),
            history: "ประวัติ".into(),
            saved: "บันทึกแล้ว".into(),
            analytics: "การวิเคราะห์".into(),
            grep: "ค้นหาข้อความ".into(),
            ml_search: "ค้นหา ML".into(),
            semantic: "ความหมาย".into(),
        }
    }

    fn vietnamese() -> Self {
        Self {
            indexed_files: "Đã lập chỉ mục {} tệp".into(),
            indexed_dirs: "Đã lập chỉ mục {} thư mục".into(),
            search_results: "Tìm thấy {} kết quả trong {:.2?}".into(),
            no_results: "Không tìm thấy kết quả.".into(),
            search_time: "Tìm kiếm hoàn thành trong {:.2?}".into(),
            index_stats: "Thống kê chỉ mục:".into(),
            files: "Tệp:".into(),
            directories: "Thư mục:".into(),
            total_size: "Tổng kích thước:".into(),
            hidden_files: "Tệp ẩn:".into(),
            unique_extensions: "Phần mở rộng duy nhất:".into(),
            last_indexed: "Chỉ mục gần nhất:".into(),
            root: "Gốc:".into(),
            error: "Lỗi:".into(),
            not_found: "Không tìm thấy đường dẫn:".into(),
            permission_denied: "Từ chối quyền:".into(),
            indexing: "Đang lập chỉ mục:".into(),
            watching: "Đang giám sát:".into(),
            press_ctrl_c: "Nhấn Ctrl+C để dừng.".into(),
            created: "ĐÃ TẠO".into(),
            modified: "ĐÃ SỬA ĐỔI".into(),
            deleted: "ĐÃ XÓA".into(),
            renamed: "ĐÃ ĐỔI TÊN".into(),
            configuration: "Cấu hình:".into(),
            search_root: "Gốc tìm kiếm:".into(),
            cache_enabled: "Bộ nhớ đệm bật:".into(),
            cache_ttl: "TTL bộ nhớ đệm:".into(),
            default_limit: "Giới hạn mặc định:".into(),
            color: "Màu sắc:".into(),
            database: "Cơ sở dữ liệu:".into(),
            running_diagnostics: "Đang chạy chẩn đoán...".into(),
            all_checks_passed: "Tất cả kiểm tra đã qua".into(),
            removed_stale: "Đã xóa {} mục cũ".into(),
            no_stale: "Không tìm thấy mục cũ".into(),
            version: "Phiên bản:".into(),
            search: "Tìm kiếm".into(),
            index: "Chỉ mục".into(),
            watch: "Giám sát".into(),
            stats: "Thống kê".into(),
            doctor: "Chẩn đoán".into(),
            config: "Cấu hình".into(),
            reindex: "Lập lại chỉ mục".into(),
            benchmark: "Đánh giá".into(),
            open_file: "Mở tệp".into(),
            open_dir: "Mở thư mục".into(),
            reveal: "Hiển thị".into(),
            history: "Lịch sử".into(),
            saved: "Đã lưu".into(),
            analytics: "Phân tích".into(),
            grep: "Tìm kiếm".into(),
            ml_search: "Tìm kiếm ML".into(),
            semantic: "Ngữ nghĩa".into(),
        }
    }

    fn indonesian() -> Self {
        Self {
            indexed_files: "Sudah mengindeks {} file".into(),
            indexed_dirs: "Sudah mengindeks {} direktori".into(),
            search_results: "Ditemukan {} hasil dalam {:.2?}".into(),
            no_results: "Tidak ada hasil ditemukan.".into(),
            search_time: "Pencarian selesai dalam {:.2?}".into(),
            index_stats: "Statistik Indeks:".into(),
            files: "File:".into(),
            directories: "Direktori:".into(),
            total_size: "Ukuran Total:".into(),
            hidden_files: "File Tersembunyi:".into(),
            unique_extensions: "Ekstensi Unik:".into(),
            last_indexed: "Terakhir Diindeks:".into(),
            root: "Root:".into(),
            error: "Kesalahan:".into(),
            not_found: "Path tidak ditemukan:".into(),
            permission_denied: "Izin ditolak:".into(),
            indexing: "Mengindeks:".into(),
            watching: "Mengawasi:".into(),
            press_ctrl_c: "Tekan Ctrl+C untuk berhenti.".into(),
            created: "DIBUAT".into(),
            modified: "DIMODIFIKASI".into(),
            deleted: "DIHAPUS".into(),
            renamed: "DIGANTI NAMA".into(),
            configuration: "Konfigurasi:".into(),
            search_root: "Root Pencarian:".into(),
            cache_enabled: "Cache Diaktifkan:".into(),
            cache_ttl: "Cache TTL:".into(),
            default_limit: "Batas Default:".into(),
            color: "Warna:".into(),
            database: "Basis Data:".into(),
            running_diagnostics: "Menjalankan diagnostik...".into(),
            all_checks_passed: "Semua pemeriksaan lulus".into(),
            removed_stale: "Menghapus {} entri usang".into(),
            no_stale: "Tidak ada entri usang ditemukan".into(),
            version: "Versi:".into(),
            search: "Cari".into(),
            index: "Indeks".into(),
            watch: "Awasi".into(),
            stats: "Statistik".into(),
            doctor: "Diagnostik".into(),
            config: "Konfigurasi".into(),
            reindex: "Ulangi Indeks".into(),
            benchmark: "Benchmark".into(),
            open_file: "Buka File".into(),
            open_dir: "Buka Direktori".into(),
            reveal: "Tampilkan".into(),
            history: "Riwayat".into(),
            saved: "Tersimpan".into(),
            analytics: "Analitik".into(),
            grep: "Grep".into(),
            ml_search: "Pencarian ML".into(),
            semantic: "Semantik".into(),
        }
    }

    fn czech() -> Self {
        Self {
            indexed_files: "Indexováno {} souborů".into(),
            indexed_dirs: "Indexováno {} adresářů".into(),
            search_results: "Nalezeno {} výsledků za {:.2?}".into(),
            no_results: "Žádné výsledky nenalezeny.".into(),
            search_time: "Hledání dokončeno za {:.2?}".into(),
            index_stats: "Statistiky indexu:".into(),
            files: "Soubory:".into(),
            directories: "Adresáře:".into(),
            total_size: "Celková velikost:".into(),
            hidden_files: "Skryté soubory:".into(),
            unique_extensions: "Unikátní přípony:".into(),
            last_indexed: "Poslední indexace:".into(),
            root: "Kořen:".into(),
            error: "Chyba:".into(),
            not_found: "Cesta nenalezena:".into(),
            permission_denied: "Přístup odepřen:".into(),
            indexing: "Indexace:".into(),
            watching: "Sledování:".into(),
            press_ctrl_c: "Stiskněte Ctrl+C pro zastavení.".into(),
            created: "VYTVOŘENO".into(),
            modified: "ZMĚNĚNO".into(),
            deleted: "SMAZÁNO".into(),
            renamed: "PŘEJMENOVÁNO".into(),
            configuration: "Konfigurace:".into(),
            search_root: "Kořen hledání:".into(),
            cache_enabled: "Mezipaměť povolena:".into(),
            cache_ttl: "TTL mezipaměti:".into(),
            default_limit: "Výchozí limit:".into(),
            color: "Barva:".into(),
            database: "Databáze:".into(),
            running_diagnostics: "Spouštění diagnostiky...".into(),
            all_checks_passed: "Všechny kontroly prošly".into(),
            removed_stale: "Odstraněno {} zastaralých záznamů".into(),
            no_stale: "Žádné zastaralé záznamy nenalezeny".into(),
            version: "Verze:".into(),
            search: "Hledat".into(),
            index: "Indexovat".into(),
            watch: "Sledovat".into(),
            stats: "Statistiky".into(),
            doctor: "Diagnostika".into(),
            config: "Konfigurace".into(),
            reindex: "Reindexovat".into(),
            benchmark: "Benchmark".into(),
            open_file: "Otevřít soubor".into(),
            open_dir: "Otevřít adresář".into(),
            reveal: "Zobrazit".into(),
            history: "Historie".into(),
            saved: "Uloženo".into(),
            analytics: "Analýzy".into(),
            grep: "Grep".into(),
            ml_search: "ML Hledání".into(),
            semantic: "Sémantické".into(),
        }
    }

    fn greek() -> Self {
        Self {
            indexed_files: "Ευρετηριάστηκαν {} αρχεία".into(),
            indexed_dirs: "Ευρετηριάστηκαν {} καταλόγοι".into(),
            search_results: "Βρέθηκαν {} αποτελέσματα σε {:.2?}".into(),
            no_results: "Δεν βρέθηκαν αποτελέσματα.".into(),
            search_time: "Η αναζήτηση ολοκληρώθηκε σε {:.2?}".into(),
            index_stats: "Στατιστικά ευρετηρίου:".into(),
            files: "Αρχεία:".into(),
            directories: "Κατάλογοι:".into(),
            total_size: "Συνολικό μέγεθος:".into(),
            hidden_files: "Κρυμμένα αρχεία:".into(),
            unique_extensions: "Μοναδικές επεκτάσεις:".into(),
            last_indexed: "Τελευταία ευρετηρίαση:".into(),
            root: "Ρίζα:".into(),
            error: "Σφάλμα:".into(),
            not_found: "Η διαδρομή δεν βρέθηκε:".into(),
            permission_denied: "Άρνηση πρόσβασης:".into(),
            indexing: "Ευρετηρίαση:".into(),
            watching: "Παρακολούθηση:".into(),
            press_ctrl_c: "Πατήστε Ctrl+C για σταμάτημα.".into(),
            created: "ΔΗΜΙΟΥΡΓΗΘΗΚΕ".into(),
            modified: "ΤΡΟΠΟΠΟΙΗΘΗΚΕ".into(),
            deleted: "ΔΙΑΓΡΑΦΗΚΕ".into(),
            renamed: "ΜΕΤΟΝΟΜΑΣΤΗΚΕ".into(),
            configuration: "Ρυθμίσεις:".into(),
            search_root: "Ρίζα αναζήτησης:".into(),
            cache_enabled: "Λανθάνουσα μνήμη ενεργοποιημένη:".into(),
            cache_ttl: "TTL λανθάνουσας μνήμης:".into(),
            default_limit: "Προεπιλεγμένο όριο:".into(),
            color: "Χρώμα:".into(),
            database: "Βάση δεδομένων:".into(),
            running_diagnostics: "Εκτέλεση διάγνωσης...".into(),
            all_checks_passed: "Όλοι οι έλεγχοι πέρασαν".into(),
            removed_stale: "Αφαιρέθηκαν {} παλιά αρχεία".into(),
            no_stale: "Δεν βρέθηκαν παλιά αρχεία".into(),
            version: "Έκδοση:".into(),
            search: "Αναζήτηση".into(),
            index: "Ευρετηρίαση".into(),
            watch: "Παρακολούθηση".into(),
            stats: "Στατιστικά".into(),
            doctor: "Διάγνωση".into(),
            config: "Ρυθμίσεις".into(),
            reindex: "Επανευρετηρίαση".into(),
            benchmark: "Benchmark".into(),
            open_file: "Άνοιγμα αρχείου".into(),
            open_dir: "Άνοιγμα καταλόγου".into(),
            reveal: "Εμφάνιση".into(),
            history: "Ιστορικό".into(),
            saved: "Αποθηκεύτηκε".into(),
            analytics: "Αναλύσεις".into(),
            grep: "Grep".into(),
            ml_search: "Αναζήτηση ML".into(),
            semantic: "Σημασιολογικό".into(),
        }
    }

    fn hebrew() -> Self {
        Self {
            indexed_files: "אוורסרו {} קבצים".into(),
            indexed_dirs: "אוורסרו {} תיקיות".into(),
            search_results: "נמצאו {} תוצאות ב-{:.2?}".into(),
            no_results: "לא נמצאו תוצאות.".into(),
            search_time: "החיפוש הושלם ב-{:.2?}".into(),
            index_stats: "סטטיסטיקות אינדקס:".into(),
            files: "קבצים:".into(),
            directories: "תיקיות:".into(),
            total_size: "גודל כולל:".into(),
            hidden_files: "קבצים נסתרים:".into(),
            unique_extensions: "הרחבות ייחודיות:".into(),
            last_indexed: "אינדקס אחרון:".into(),
            root: "שורש:".into(),
            error: "שגיאה:".into(),
            not_found: "נתיב לא נמצא:".into(),
            permission_denied: "הרשאה נדחתה:".into(),
            indexing: "אינדקסציה:".into(),
            watching: "מעקב:".into(),
            press_ctrl_c: "לחץ Ctrl+C לעצירה.".into(),
            created: "נוצר".into(),
            modified: "שונה".into(),
            deleted: "נמחק".into(),
            renamed: "שונה שם".into(),
            configuration: "הגדרות:".into(),
            search_root: "שורש חיפוש:".into(),
            cache_enabled: "זיכרון מטמון מופעל:".into(),
            cache_ttl: "TTL זיכרון מטמון:".into(),
            default_limit: "ברירת מחדל:".into(),
            color: "צבע:".into(),
            database: "מסד נתונים:".into(),
            running_diagnostics: "מריץ אבחון...".into(),
            all_checks_passed: "כל הבדיקות עברו".into(),
            removed_stale: "הוסרו {} רשומות ישנות".into(),
            no_stale: "לא נמצאו רשומות ישנות".into(),
            version: "גרסה:".into(),
            search: "חיפוש".into(),
            index: "אינדקס".into(),
            watch: "מעקב".into(),
            stats: "סטטיסטיקות".into(),
            doctor: "אבחון".into(),
            config: "הגדרות".into(),
            reindex: "אינדקס מחדש".into(),
            benchmark: "מבחן ביצועים".into(),
            open_file: "פתח קובץ".into(),
            open_dir: "פתח תיקייה".into(),
            reveal: "הצג".into(),
            history: "היסטוריה".into(),
            saved: "נשמר".into(),
            analytics: "אנליטיקה".into(),
            grep: "חיפוש".into(),
            ml_search: "חיפוש ML".into(),
            semantic: "סמנטי".into(),
        }
    }

    fn finnish() -> Self {
        Self {
            indexed_files: "Indeksoitu {} tiedostoa".into(),
            indexed_dirs: "Indeksoitu {} hakemistoa".into(),
            search_results: "Löytyi {} tulosta ajassa {:.2?}".into(),
            no_results: "Ei tuloksia.".into(),
            search_time: "Haku valmis ajassa {:.2?}".into(),
            index_stats: "Indeksitilastot:".into(),
            files: "Tiedostot:".into(),
            directories: "Hakemistot:".into(),
            total_size: "Kokonaiskoko:".into(),
            hidden_files: "Piilotetut tiedostot:".into(),
            unique_extensions: "Ainutlaakset laajennukset:".into(),
            last_indexed: "Viimeksi indeksoitu:".into(),
            root: "Juuri:".into(),
            error: "Virhe:".into(),
            not_found: "Polkua ei löydy:".into(),
            permission_denied: "Kielletty:".into(),
            indexing: "Indeksöinti:".into(),
            watching: "Valvonta:".into(),
            press_ctrl_c: "Paina Ctrl+C lopettaaksesi.".into(),
            created: "LUOTU".into(),
            modified: "MUOKATTU".into(),
            deleted: "POISTETTU".into(),
            renamed: "NIMETTY UUDELLEEN".into(),
            configuration: "Asetukset:".into(),
            search_root: "Haun juuri:".into(),
            cache_enabled: "Välimuisti käytössä:".into(),
            cache_ttl: "Välimuisti TTL:".into(),
            default_limit: "Oletusraja:".into(),
            color: "Väri:".into(),
            database: "Tietokanta:".into(),
            running_diagnostics: "Suoritetaan diagnostiikkaa...".into(),
            all_checks_passed: "Kaikki tarkistukset läpäisty".into(),
            removed_stale: "Poistettu {} vanhentunutta merkintää".into(),
            no_stale: "Ei vanhentuneita merkintöjä".into(),
            version: "Versio:".into(),
            search: "Hae".into(),
            index: "Indeksoi".into(),
            watch: "Valvo".into(),
            stats: "Tilastot".into(),
            doctor: "Diagnostiikka".into(),
            config: "Asetukset".into(),
            reindex: "Uudelleenindeksoi".into(),
            benchmark: "Vertailu".into(),
            open_file: "Avaa tiedosto".into(),
            open_dir: "Avaa hakemisto".into(),
            reveal: "Näytä".into(),
            history: "Historia".into(),
            saved: "Tallennettu".into(),
            analytics: "Analytiikka".into(),
            grep: "Hae".into(),
            ml_search: "ML-haku".into(),
            semantic: "Semanttinen".into(),
        }
    }

    fn norwegian() -> Self {
        Self {
            indexed_files: "Indeksert {} filer".into(),
            indexed_dirs: "Indeksert {} kataloger".into(),
            search_results: "Fant {} resultater på {:.2?}".into(),
            no_results: "Ingen resultater funnet.".into(),
            search_time: "Søk fullført på {:.2?}".into(),
            index_stats: "Indeksstatistikk:".into(),
            files: "Filer:".into(),
            directories: "Kataloger:".into(),
            total_size: "Total størrelse:".into(),
            hidden_files: "Skjulte filer:".into(),
            unique_extensions: "Unike filtyper:".into(),
            last_indexed: "Sist indeksert:".into(),
            root: "Rot:".into(),
            error: "Feil:".into(),
            not_found: "Sti ikke funnet:".into(),
            permission_denied: "Tilgang nektet:".into(),
            indexing: "Indeksering:".into(),
            watching: "Overvåking:".into(),
            press_ctrl_c: "Trykk Ctrl+C for å stoppe.".into(),
            created: "OPPRETTET".into(),
            modified: "ENDRET".into(),
            deleted: "SLETTET".into(),
            renamed: "OMØPPT".into(),
            configuration: "Konfigurasjon:".into(),
            search_root: "Søkerot:".into(),
            cache_enabled: "Buffer aktivert:".into(),
            cache_ttl: "Buffer TTL:".into(),
            default_limit: "Standardgrense:".into(),
            color: "Farge:".into(),
            database: "Database:".into(),
            running_diagnostics: "Kjører diagnostikk...".into(),
            all_checks_passed: "Alle sjekker bestått".into(),
            removed_stale: "Fjernet {} utdaterte oppføringer".into(),
            no_stale: "Ingen utdaterte oppføringer funnet".into(),
            version: "Versjon:".into(),
            search: "Søk".into(),
            index: "Indekser".into(),
            watch: "Overvåk".into(),
            stats: "Statistikk".into(),
            doctor: "Diagnostikk".into(),
            config: "Konfigurasjon".into(),
            reindex: "Reindekser".into(),
            benchmark: "Benchmark".into(),
            open_file: "Åpne fil".into(),
            open_dir: "Åpne katalog".into(),
            reveal: "Vis".into(),
            history: "Historikk".into(),
            saved: "Lagret".into(),
            analytics: "Analyser".into(),
            grep: "Grep".into(),
            ml_search: "ML-søk".into(),
            semantic: "Semantisk".into(),
        }
    }

    fn danish() -> Self {
        Self {
            indexed_files: "Indekseret {} filer".into(),
            indexed_dirs: "Indekseret {} mapper".into(),
            search_results: "Fandt {} resultater på {:.2?}".into(),
            no_results: "Ingen resultater fundet.".into(),
            search_time: "Søgning fuldført på {:.2?}".into(),
            index_stats: "Indeksstatistik:".into(),
            files: "Filer:".into(),
            directories: "Mapper:".into(),
            total_size: "Samlet størrelse:".into(),
            hidden_files: "Skjulte filer:".into(),
            unique_extensions: "Unikke filtyper:".into(),
            last_indexed: "Senest indekseret:".into(),
            root: "Rod:".into(),
            error: "Fejl:".into(),
            not_found: "Sti ikke fundet:".into(),
            permission_denied: "Adgang nægtet:".into(),
            indexing: "Indeksering:".into(),
            watching: "Overvågning:".into(),
            press_ctrl_c: "Tryk på Ctrl+C for at stoppe.".into(),
            created: "OPRETTET".into(),
            modified: "ÆNDRET".into(),
            deleted: "SLETTET".into(),
            renamed: "OMØBT".into(),
            configuration: "Konfiguration:".into(),
            search_root: "Søgerod:".into(),
            cache_enabled: "Buffer aktiveret:".into(),
            cache_ttl: "Buffer TTL:".into(),
            default_limit: "Standardgrænse:".into(),
            color: "Farve:".into(),
            database: "Database:".into(),
            running_diagnostics: "Kører diagnostik...".into(),
            all_checks_passed: "Alle kontroller bestået".into(),
            removed_stale: "Fjernet {} forældede poster".into(),
            no_stale: "Ingen forældede poster fundet".into(),
            version: "Version:".into(),
            search: "Søg".into(),
            index: "Indekser".into(),
            watch: "Overvåg".into(),
            stats: "Statistik".into(),
            doctor: "Diagnostik".into(),
            config: "Konfiguration".into(),
            reindex: "Genindkser".into(),
            benchmark: "Benchmark".into(),
            open_file: "Åbn fil".into(),
            open_dir: "Åbn mappe".into(),
            reveal: "Vis".into(),
            history: "Historik".into(),
            saved: "Gemt".into(),
            analytics: "Analyser".into(),
            grep: "Grep".into(),
            ml_search: "ML-søgning".into(),
            semantic: "Semantisk".into(),
        }
    }

    fn hungarian() -> Self {
        Self {
            indexed_files: "{} fájl indexelve".into(),
            indexed_dirs: "{} könyvtár indexelve".into(),
            search_results: "{} találat {:.2?} alatt".into(),
            no_results: "Nincs találat.".into(),
            search_time: "Keresés befejezve {:.2?} alatt".into(),
            index_stats: "Index statisztikák:".into(),
            files: "Fájlok:".into(),
            directories: "Könyvtárak:".into(),
            total_size: "Összes méret:".into(),
            hidden_files: "Rejtett fájlok:".into(),
            unique_extensions: "Egyedi kiterjesztések:".into(),
            last_indexed: "Utolsó indexelés:".into(),
            root: "Gyökér:".into(),
            error: "Hiba:".into(),
            not_found: "Útvonal nem található:".into(),
            permission_denied: "Hozzáférés megtagadva:".into(),
            indexing: "Indexelés:".into(),
            watching: "Megfigyelés:".into(),
            press_ctrl_c: "Nyomd meg a Ctrl+C-t a leállításhoz.".into(),
            created: "LÉTREHOZVA".into(),
            modified: "MÓDOSÍTVA".into(),
            deleted: "TÖRÖLVE".into(),
            renamed: "ÁTNEVEZVE".into(),
            configuration: "Konfiguráció:".into(),
            search_root: "Keresés gyökere:".into(),
            cache_enabled: "Gyorsítótár engedélyezve:".into(),
            cache_ttl: "Gyorsítótár TTL:".into(),
            default_limit: "Alapértelmezett korlát:".into(),
            color: "Szín:".into(),
            database: "Adatbázis:".into(),
            running_diagnostics: "Diagnosztika futtatása...".into(),
            all_checks_passed: "Minden ellenőrzés átment".into(),
            removed_stale: "{} elavult bejegyzés eltávolítva".into(),
            no_stale: "Nincs elavult bejegyzés".into(),
            version: "Verzió:".into(),
            search: "Keresés".into(),
            index: "Indexelés".into(),
            watch: "Megfigyelés".into(),
            stats: "Statisztikák".into(),
            doctor: "Diagnosztika".into(),
            config: "Konfiguráció".into(),
            reindex: "Újraindexelés".into(),
            benchmark: "Benchmark".into(),
            open_file: "Fájl megnyitása".into(),
            open_dir: "Könyvtár megnyitása".into(),
            reveal: "Megjelenítés".into(),
            history: "Előzmények".into(),
            saved: "Mentve".into(),
            analytics: "Analitika".into(),
            grep: "Grep".into(),
            ml_search: "ML keresés".into(),
            semantic: "Szemantikus".into(),
        }
    }

    fn romanian() -> Self {
        Self {
            indexed_files: "{} fișiere indexate".into(),
            indexed_dirs: "{} directoare indexate".into(),
            search_results: "{} rezultate găsite în {:.2?}".into(),
            no_results: "Niciun rezultat găsit.".into(),
            search_time: "Căutare finalizată în {:.2?}".into(),
            index_stats: "Statistici index:".into(),
            files: "Fișiere:".into(),
            directories: "Directoare:".into(),
            total_size: "Dimensiune totală:".into(),
            hidden_files: "Fișiere ascunse:".into(),
            unique_extensions: "Extensii unice:".into(),
            last_indexed: "Ultimul indexat:".into(),
            root: "Rădăcină:".into(),
            error: "Eroare:".into(),
            not_found: "Cale negăsită:".into(),
            permission_denied: "Acces interzis:".into(),
            indexing: "Indexare:".into(),
            watching: "Monitorizare:".into(),
            press_ctrl_c: "Apasă Ctrl+C pentru a opri.".into(),
            created: "CREAT".into(),
            modified: "MODIFICAT".into(),
            deleted: "ȘTERS".into(),
            renamed: "REDENUMIT".into(),
            configuration: "Configurație:".into(),
            search_root: "Rădăcina căutării:".into(),
            cache_enabled: "Cache activat:".into(),
            cache_ttl: "Cache TTL:".into(),
            default_limit: "Limită implicită:".into(),
            color: "Culoare:".into(),
            database: "Bază de date:".into(),
            running_diagnostics: "Rulare diagnostic...".into(),
            all_checks_passed: "Toate verificările au trecut".into(),
            removed_stale: "{} intrări învechite eliminate".into(),
            no_stale: "Nicio intrare învechită găsită".into(),
            version: "Versiune:".into(),
            search: "Căutare".into(),
            index: "Indexare".into(),
            watch: "Monitorizare".into(),
            stats: "Statistici".into(),
            doctor: "Diagnostic".into(),
            config: "Configurație".into(),
            reindex: "Reindexare".into(),
            benchmark: "Benchmark".into(),
            open_file: "Deschide fișier".into(),
            open_dir: "Deschide director".into(),
            reveal: "Afișează".into(),
            history: "Istoric".into(),
            saved: "Salvat".into(),
            analytics: "Analize".into(),
            grep: "Grep".into(),
            ml_search: "Căutare ML".into(),
            semantic: "Semantic".into(),
        }
    }

    fn ukrainian() -> Self {
        Self {
            indexed_files: "Проіндексовано {} файлів".into(),
            indexed_dirs: "Проіндексовано {} каталогів".into(),
            search_results: "Знайдено {} результатів за {:.2?}".into(),
            no_results: "Результатів не знайдено.".into(),
            search_time: "Пошук завершено за {:.2?}".into(),
            index_stats: "Статистика індексу:".into(),
            files: "Файли:".into(),
            directories: "Каталоги:".into(),
            total_size: "Загальний розмір:".into(),
            hidden_files: "Приховані файли:".into(),
            unique_extensions: "Унікальні розширення:".into(),
            last_indexed: "Остання індексація:".into(),
            root: "Корінь:".into(),
            error: "Помилка:".into(),
            not_found: "Шлях не знайдено:".into(),
            permission_denied: "Доступ заборонено:".into(),
            indexing: "Індексація:".into(),
            watching: "Спостереження:".into(),
            press_ctrl_c: "Натисніть Ctrl+C для зупинки.".into(),
            created: "СТВОРЕНО".into(),
            modified: "ЗМІНЕНО".into(),
            deleted: "ВИДАЛЕНО".into(),
            renamed: "ПЕРЕЙМЕНОВАНО".into(),
            configuration: "Конфігурація:".into(),
            search_root: "Корінь пошуку:".into(),
            cache_enabled: "Кеш увімкнено:".into(),
            cache_ttl: "TTL кешу:".into(),
            default_limit: "Стандартний ліміт:".into(),
            color: "Колір:".into(),
            database: "База даних:".into(),
            running_diagnostics: "Запуск діагностики...".into(),
            all_checks_passed: "Усі перевірки пройдено".into(),
            removed_stale: "Видалено {} застарілих записів".into(),
            no_stale: "Застарілих записів не знайдено".into(),
            version: "Версія:".into(),
            search: "Пошук".into(),
            index: "Індексація".into(),
            watch: "Спостереження".into(),
            stats: "Статистика".into(),
            doctor: "Діагностика".into(),
            config: "Конфігурація".into(),
            reindex: "Повторна індексація".into(),
            benchmark: "Тест продуктивності".into(),
            open_file: "Відкрити файл".into(),
            open_dir: "Відкрити каталог".into(),
            reveal: "Показати".into(),
            history: "Історія".into(),
            saved: "Збережено".into(),
            analytics: "Аналітика".into(),
            grep: "Пошук".into(),
            ml_search: "ML пошук".into(),
            semantic: "Семантичний".into(),
        }
    }

    fn bengali() -> Self {
        Self {
            indexed_files: "{} ফাইল ইনডেক্স করা হয়েছে".into(),
            indexed_dirs: "{} ডিরেক্টরি ইনডেক্স করা হয়েছে".into(),
            search_results: "{} টি ফলাফল পাওয়া গেছে ({:.2?})".into(),
            no_results: "কোনো ফলাফল পাওয়া যায়নি।".into(),
            search_time: "অনুসন্ধান {:.2?} এ সম্পন্ন".into(),
            index_stats: "ইনডেক্স পরিসংখ্যান:".into(),
            files: "ফাইল:".into(),
            directories: "ডিরেক্টরি:".into(),
            total_size: "মোট আকার:".into(),
            hidden_files: "লুকানো ফাইল:".into(),
            unique_extensions: "অনন্য এক্সটেনশন:".into(),
            last_indexed: "শেষ ইনডেক্সিং:".into(),
            root: "রুট:".into(),
            error: "ত্রুটি:".into(),
            not_found: "পাথ পাওয়া যায়নি:".into(),
            permission_denied: "অনুমতি নিষেধ:".into(),
            indexing: "ইনডেক্সিং:".into(),
            watching: "পর্যবেক্ষণ:".into(),
            press_ctrl_c: "বন্ধ করতে Ctrl+C চাপুন।".into(),
            created: "তৈরি হয়েছে".into(),
            modified: "পরিবর্তন হয়েছে".into(),
            deleted: "মুছে ফেলা হয়েছে".into(),
            renamed: "নাম পরিবর্তন হয়েছে".into(),
            configuration: "কনফিগারেশন:".into(),
            search_root: "অনুসন্ধান রুট:".into(),
            cache_enabled: "ক্যাশ সক্রিয়:".into(),
            cache_ttl: "ক্যাশ TTL:".into(),
            default_limit: "ডিফল্ট সীমা:".into(),
            color: "রং:".into(),
            database: "ডাটাবেস:".into(),
            running_diagnostics: "ডায়াগনস্টিক চালাচ্ছে...".into(),
            all_checks_passed: "সব পরীক্ষা পাস".into(),
            removed_stale: "{} টি পুরানো এন্ট্রি সরানো হয়েছে".into(),
            no_stale: "কোনো পুরানো এন্ট্রি পাওয়া যায়নি".into(),
            version: "সংস্করণ:".into(),
            search: "অনুসন্ধান".into(),
            index: "ইনডেক্স".into(),
            watch: "পর্যবেক্ষণ".into(),
            stats: "পরিসংখ্যান".into(),
            doctor: "ডায়াগনস্টিক".into(),
            config: "কনফিগারেশন".into(),
            reindex: "পুনঃইনডেক্স".into(),
            benchmark: "বেঞ্চমার্ক".into(),
            open_file: "ফাইল খুলুন".into(),
            open_dir: "ডিরেক্টরি খুলুন".into(),
            reveal: "দেখান".into(),
            history: "ইতিহাস".into(),
            saved: "সংরক্ষিত".into(),
            analytics: "বিশ্লেষণ".into(),
            grep: "অনুসন্ধান".into(),
            ml_search: "ML অনুসন্ধান".into(),
            semantic: "অর্থগত".into(),
        }
    }

    fn tamil() -> Self {
        Self {
            indexed_files: "{} கோப்புகள் சுட்டிடப்பட்டன".into(),
            indexed_dirs: "{} அடைவுகள் சுட்டிடப்பட்டன".into(),
            search_results: "{} முடிவுகள் கிடைத்தன ({:.2?})".into(),
            no_results: "முடிவுகள் இல்லை.".into(),
            search_time: "தேடல் முடிந்தது {:.2?}".into(),
            index_stats: "சுட்டி புள்ளிவிவரங்கள்:".into(),
            files: "கோப்புகள்:".into(),
            directories: "அடைவுகள்:".into(),
            total_size: "மொத்த அளவு:".into(),
            hidden_files: "மறைக்கப்பட்ட கோப்புகள்:".into(),
            unique_extensions: "தனித்துவ நீட்டிப்புகள்:".into(),
            last_indexed: "கடைசி சுட்டிடம்:".into(),
            root: "மூல:".into(),
            error: "பிழை:".into(),
            not_found: "பாதை கிடைக்கவில்லை:".into(),
            permission_denied: "அனுமதி மறுக்கப்பட்டது:".into(),
            indexing: "சுட்டிடம்:".into(),
            watching: "கண்காணிப்பு:".into(),
            press_ctrl_c: "நிறுத்த Ctrl+C அழுத்தவும்.".into(),
            created: "உருவாக்கப்பட்டது".into(),
            modified: "மாற்றப்பட்டது".into(),
            deleted: "நீக்கப்பட்டது".into(),
            renamed: "மறுபெயரிடப்பட்டது".into(),
            configuration: "உள்ளமைவு:".into(),
            search_root: "தேடல் மூல:".into(),
            cache_enabled: "தற்காலிக சேமிப்பு இயக்கம்:".into(),
            cache_ttl: "தற்காலிக சேமிப்பு TTL:".into(),
            default_limit: "இயல்புநிலை வரம்பு:".into(),
            color: "நிறம்:".into(),
            database: "தரவுத்தளம்:".into(),
            running_diagnostics: "நோய் கண்டறிதல் இயங்குகிறது...".into(),
            all_checks_passed: "அனைத்து சோதனைகளும் கடந்தன".into(),
            removed_stale: "{} பழைய உள்ளீடுகள் நீக்கப்பட்டன".into(),
            no_stale: "பழைய உள்ளீடுகள் இல்லை".into(),
            version: "பதிப்பு:".into(),
            search: "தேடு".into(),
            index: "சுட்டிடம்".into(),
            watch: "கண்காணி".into(),
            stats: "புள்ளிவிவரங்கள்".into(),
            doctor: "நோய் கண்டறிதல்".into(),
            config: "உள்ளமைவு".into(),
            reindex: "மறுசுட்டிடம்".into(),
            benchmark: "அளவுகோல்".into(),
            open_file: "கோப்பை திற".into(),
            open_dir: "அடைவை திற".into(),
            reveal: "காட்டு".into(),
            history: "வரலாறு".into(),
            saved: "சேமிக்கப்பட்டது".into(),
            analytics: "பகுப்பாய்வு".into(),
            grep: "தேடு".into(),
            ml_search: "ML தேடல்".into(),
            semantic: "பொருள்".into(),
        }
    }
}

/// Detects the system locale.
pub fn detect_locale() -> Language {
    if let Ok(lang) = std::env::var("LANG") {
        let code = lang.split('.').next().unwrap_or(&lang);
        let code = code.split('_').next().unwrap_or(code);
        if let Some(language) = Language::from_code(code) {
            return language;
        }
    }

    if let Ok(lang) = std::env::var("LC_ALL") {
        let code = lang.split('.').next().unwrap_or(&lang);
        let code = code.split('_').next().unwrap_or(code);
        if let Some(language) = Language::from_code(code) {
            return language;
        }
    }

    Language::English
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_codes() {
        assert_eq!(Language::English.code(), "en");
        assert_eq!(Language::Spanish.code(), "es");
        assert_eq!(Language::French.code(), "fr");
        assert_eq!(Language::German.code(), "de");
        assert_eq!(Language::Japanese.code(), "ja");
        assert_eq!(Language::Chinese.code(), "zh");
        assert_eq!(Language::Portuguese.code(), "pt");
        assert_eq!(Language::Russian.code(), "ru");
        assert_eq!(Language::Arabic.code(), "ar");
        assert_eq!(Language::Hindi.code(), "hi");
        assert_eq!(Language::Korean.code(), "ko");
        assert_eq!(Language::Italian.code(), "it");
        assert_eq!(Language::Turkish.code(), "tr");
        assert_eq!(Language::Polish.code(), "pl");
        assert_eq!(Language::Dutch.code(), "nl");
        assert_eq!(Language::Swedish.code(), "sv");
        assert_eq!(Language::Thai.code(), "th");
        assert_eq!(Language::Vietnamese.code(), "vi");
        assert_eq!(Language::Indonesian.code(), "id");
        assert_eq!(Language::Czech.code(), "cs");
        assert_eq!(Language::Greek.code(), "el");
        assert_eq!(Language::Hebrew.code(), "he");
        assert_eq!(Language::Finnish.code(), "fi");
        assert_eq!(Language::Norwegian.code(), "no");
        assert_eq!(Language::Danish.code(), "da");
        assert_eq!(Language::Hungarian.code(), "hu");
        assert_eq!(Language::Romanian.code(), "ro");
        assert_eq!(Language::Ukrainian.code(), "uk");
        assert_eq!(Language::Bengali.code(), "bn");
        assert_eq!(Language::Tamil.code(), "ta");
    }

    #[test]
    fn test_native_names() {
        assert_eq!(Language::English.native_name(), "English");
        assert_eq!(Language::Spanish.native_name(), "Español");
        assert_eq!(Language::Japanese.native_name(), "日本語");
        assert_eq!(Language::Korean.native_name(), "한국어");
        assert_eq!(Language::Arabic.native_name(), "العربية");
    }

    #[test]
    fn test_english_names() {
        assert_eq!(Language::English.english_name(), "English");
        assert_eq!(Language::Spanish.english_name(), "Spanish");
        assert_eq!(Language::Japanese.english_name(), "Japanese");
        assert_eq!(Language::Korean.english_name(), "Korean");
        assert_eq!(Language::Arabic.english_name(), "Arabic");
    }

    #[test]
    fn test_from_code() {
        assert_eq!(Language::from_code("en"), Some(Language::English));
        assert_eq!(Language::from_code("es"), Some(Language::Spanish));
        assert_eq!(Language::from_code("ko"), Some(Language::Korean));
        assert_eq!(Language::from_code("it"), Some(Language::Italian));
        assert_eq!(Language::from_code("xx"), None);
    }

    #[test]
    fn test_from_name() {
        assert_eq!(Language::from_name("English"), Some(Language::English));
        assert_eq!(Language::from_name("spanish"), Some(Language::Spanish));
        assert_eq!(Language::from_name("Español"), Some(Language::Spanish));
        assert_eq!(Language::from_name("français"), Some(Language::French));
        assert_eq!(Language::from_name("korean"), Some(Language::Korean));
        assert_eq!(Language::from_name("한국어"), Some(Language::Korean));
        assert_eq!(Language::from_name("unknown"), None);
    }

    #[test]
    fn test_all_languages() {
        let all = Language::all();
        assert_eq!(all.len(), 30);
    }

    #[test]
    fn test_translations_for_language() {
        let t = Translations::for_language(Language::English);
        assert_eq!(t.files, "Files:");
        let t = Translations::for_language(Language::Spanish);
        assert_eq!(t.files, "Archivos:");
        let t = Translations::for_language(Language::Korean);
        assert_eq!(t.files, "파일:");
    }

    #[test]
    fn test_custom_translations() {
        let mut custom = std::collections::HashMap::new();
        custom.insert("files".into(), "My Files:".into());
        custom.insert("error".into(), "Oops:".into());
        let t = Translations::from_custom(custom);
        assert_eq!(t.files, "My Files:");
        assert_eq!(t.error, "Oops:");
        assert_eq!(t.directories, "Directories:");
    }

    #[test]
    fn test_detect_locale_returns_english_by_default() {
        let lang = detect_locale();
        assert!(Language::all().contains(&lang));
    }
}
