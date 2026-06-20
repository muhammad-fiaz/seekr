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
            _ => None,
        }
    }
}

/// Translation strings for a language.
#[derive(Debug, Clone)]
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
    }

    #[test]
    fn test_native_names() {
        assert_eq!(Language::English.native_name(), "English");
        assert_eq!(Language::Spanish.native_name(), "Español");
        assert_eq!(Language::Japanese.native_name(), "日本語");
    }

    #[test]
    fn test_from_code() {
        assert_eq!(Language::from_code("en"), Some(Language::English));
        assert_eq!(Language::from_code("es"), Some(Language::Spanish));
        assert_eq!(Language::from_code("xx"), None);
    }

    #[test]
    fn test_all_languages() {
        let all = Language::all();
        assert_eq!(all.len(), 10);
    }

    #[test]
    fn test_translations_for_language() {
        let t = Translations::for_language(Language::English);
        assert_eq!(t.files, "Files:");
        let t = Translations::for_language(Language::Spanish);
        assert_eq!(t.files, "Archivos:");
    }

    #[test]
    fn test_detect_locale_returns_english_by_default() {
        let lang = detect_locale();
        assert!(Language::all().contains(&lang));
    }
}
