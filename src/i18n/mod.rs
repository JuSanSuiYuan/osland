// Internationalization (i18n) module for OSland
// Copyright (c) 2025 OSland Project Team
// SPDX-License-Identifier: MulanPSL-2.0

use std::collections::HashMap;
use std::sync::RwLock;

/// Supported languages
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Language {
    /// Chinese (Simplified)
    Chinese,
    /// English
    English,
    /// Japanese
    Japanese,
    /// Korean
    Korean,
    /// Russian
    Russian,
}

impl Language {
    /// Get language code
    pub fn code(&self) -> &str {
        match self {
            Language::Chinese => "zh-CN",
            Language::English => "en-US",
            Language::Japanese => "ja-JP",
            Language::Korean => "ko-KR",
            Language::Russian => "ru-RU",
        }
    }
    
    /// Get display name in the language itself
    pub fn display_name(&self) -> &str {
        match self {
            Language::Chinese => "中文",
            Language::English => "English",
            Language::Japanese => "日本語",
            Language::Korean => "한국어",
            Language::Russian => "Русский",
        }
    }
    
    /// Parse language from code
    pub fn from_code(code: &str) -> Option<Self> {
        match code.to_lowercase().as_str() {
            "zh-cn" | "zh" => Some(Language::Chinese),
            "en-us" | "en" => Some(Language::English),
            "ja-jp" | "ja" => Some(Language::Japanese),
            "ko-kr" | "ko" => Some(Language::Korean),
            "ru-ru" | "ru" => Some(Language::Russian),
            _ => None,
        }
    }
    
    /// Get system default language
    pub fn system_default() -> Self {
        // Get system language from environment variables
        if let Some(lang) = std::env::var("LANG").ok() {
            if let Some(language) = Language::from_code(&lang) {
                return language;
            }
        }
        
        // Default to Chinese if system language cannot be determined
        Language::Chinese
    }
}

/// Translations container
pub struct Translations {
    /// Language mappings
    translations: RwLock<HashMap<String, HashMap<Language, String>>>,
    
    /// Default language
    default_language: Language,
}

impl Translations {
    /// Create new translations container
    pub fn new(default_language: Language) -> Self {
        Self {
            translations: RwLock::new(HashMap::new()),
            default_language,
        }
    }
    
    /// Add a translation
    pub fn add_translation(&self, key: &str, language: Language, value: &str) {
        let mut translations = self.translations.write().unwrap();
        
        let entry = translations.entry(key.to_string()).or_insert_with(HashMap::new);
        entry.insert(language, value.to_string());
    }
    
    /// Get translation for a key
    pub fn translate(&self, key: &str, language: Option<Language>) -> String {
        let translations = self.translations.read().unwrap();
        
        let lang = language.unwrap_or(self.default_language);
        
        if let Some(translations) = translations.get(key) {
            if let Some(translation) = translations.get(&lang) {
                return translation.clone();
            }
            
            // Fallback to default language
            if let Some(translation) = translations.get(&self.default_language) {
                return translation.clone();
            }
        }
        
        // Fallback to key itself if no translation found
        key.to_string()
    }
    
    /// Load default translations
    pub fn load_default_translations(&self) {
        // Common UI translations
        self.add_translation("app.title", Language::Chinese, "OSland 操作系统可视化编程IDE");
        self.add_translation("app.title", Language::English, "OSland Operating System Visual Programming IDE");
        
        self.add_translation("menu.file", Language::Chinese, "文件");
        self.add_translation("menu.file", Language::English, "File");
        
        self.add_translation("menu.edit", Language::Chinese, "编辑");
        self.add_translation("menu.edit", Language::English, "Edit");
        
        self.add_translation("menu.view", Language::Chinese, "视图");
        self.add_translation("menu.view", Language::English, "View");
        
        self.add_translation("menu.tools", Language::Chinese, "工具");
        self.add_translation("menu.tools", Language::English, "Tools");
        
        self.add_translation("menu.help", Language::Chinese, "帮助");
        self.add_translation("menu.help", Language::English, "Help");
        
        // Project translations
        self.add_translation("project.new", Language::Chinese, "新建项目");
        self.add_translation("project.new", Language::English, "New Project");
        
        self.add_translation("project.open", Language::Chinese, "打开项目");
        self.add_translation("project.open", Language::English, "Open Project");
        
        self.add_translation("project.save", Language::Chinese, "保存项目");
        self.add_translation("project.save", Language::English, "Save Project");
        
        // Component translations
        self.add_translation("component.panel.title", Language::Chinese, "组件面板");
        self.add_translation("component.panel.title", Language::English, "Component Panel");
        
        self.add_translation("component.add", Language::Chinese, "添加组件");
        self.add_translation("component.add", Language::English, "Add Component");
        
        // Property translations
        self.add_translation("property.panel.title", Language::Chinese, "属性面板");
        self.add_translation("property.panel.title", Language::English, "Property Panel");
        
        // Canvas translations
        self.add_translation("canvas.title", Language::Chinese, "画布");
        self.add_translation("canvas.title", Language::English, "Canvas");
        
        // Build engine translations
        self.add_translation("build.start", Language::Chinese, "开始构建");
        self.add_translation("build.start", Language::English, "Start Build");
        
        self.add_translation("build.success", Language::Chinese, "构建成功");
        self.add_translation("build.success", Language::English, "Build Successful");
        
        self.add_translation("build.failed", Language::Chinese, "构建失败");
        self.add_translation("build.failed", Language::English, "Build Failed");
        
        // Kernel extractor translations
        self.add_translation("extract.start", Language::Chinese, "开始提取组件");
        self.add_translation("extract.start", Language::English, "Start Component Extraction");
        
        self.add_translation("extract.success", Language::Chinese, "组件提取成功");
        self.add_translation("extract.success", Language::English, "Component Extraction Successful");
        
        self.add_translation("extract.failed", Language::Chinese, "组件提取失败");
        self.add_translation("extract.failed", Language::English, "Component Extraction Failed");
        
        // CLI translations
        self.add_translation("cli.run", Language::Chinese, "运行OSland IDE");
        self.add_translation("cli.run", Language::English, "Start the OSland IDE");
        
        self.add_translation("cli.extract", Language::Chinese, "从开源内核提取组件");
        self.add_translation("cli.extract", Language::English, "Extract components from open source kernels");
        
        self.add_translation("cli.extract.source", Language::Chinese, "内核源代码目录");
        self.add_translation("cli.extract.source", Language::English, "Kernel source directory");
        
        self.add_translation("cli.extract.output", Language::Chinese, "提取组件输出目录");
        self.add_translation("cli.extract.output", Language::English, "Output directory for extracted components");
        
        self.add_translation("cli.build", Language::Chinese, "构建操作系统镜像");
        self.add_translation("cli.build", Language::English, "Build an operating system image");
        
        self.add_translation("cli.build.config", Language::Chinese, "项目配置文件");
        self.add_translation("cli.build.config", Language::English, "Project configuration file");
        
        self.add_translation("cli.build.output", Language::Chinese, "输出镜像文件路径");
        self.add_translation("cli.build.output", Language::English, "Output image file path");
        
        self.add_translation("cli.debug", Language::Chinese, "启用调试日志");
        self.add_translation("cli.debug", Language::English, "Enable debug logging");
        
        // Status messages
        self.add_translation("status.starting", Language::Chinese, "正在启动OSland v0.1.0...");
        self.add_translation("status.starting", Language::English, "Starting OSland v0.1.0...");
        
        self.add_translation("status.ide_started", Language::Chinese, "OSland IDE已启动");
        self.add_translation("status.ide_started", Language::English, "OSland IDE started");
        
        self.add_translation("status.extracting", Language::Chinese, "正在从{0}提取组件到{1}...");
        self.add_translation("status.extracting", Language::English, "Extracting components from {0} to {1}...");
        
        self.add_translation("status.building", Language::Chinese, "正在从{0}构建OS镜像到{1}...");
        self.add_translation("status.building", Language::English, "Building OS image from {0} to {1}...");
        
        self.add_translation("status.no_command", Language::Chinese, "未指定命令，默认启动IDE...");
        self.add_translation("status.no_command", Language::English, "No command specified, starting IDE by default...");
        
        self.add_translation("status.exiting", Language::Chinese, "正在退出OSland...");
        self.add_translation("status.exiting", Language::English, "Exiting OSland...");
        
        // Merge sort demo translations
        self.add_translation("merge_sort.demo.description", Language::Chinese, "OSland归并排序可视化演示");
        self.add_translation("merge_sort.demo.description", Language::English, "OSland Merge Sort Visualization Demo");
        
        self.add_translation("merge_sort.component.input_array", Language::Chinese, "输入数组");
        self.add_translation("merge_sort.component.input_array", Language::English, "Input Array");
        
        self.add_translation("merge_sort.component.length_check", Language::Chinese, "长度检查");
        self.add_translation("merge_sort.component.length_check", Language::English, "Length Check");
        
        self.add_translation("merge_sort.component.direct_return", Language::Chinese, "直接返回");
        self.add_translation("merge_sort.component.direct_return", Language::English, "Direct Return");
        
        self.add_translation("merge_sort.component.split_array", Language::Chinese, "分割数组");
        self.add_translation("merge_sort.component.split_array", Language::English, "Split Array");
        
        self.add_translation("merge_sort.component.merge_sort", Language::Chinese, "归并排序");
        self.add_translation("merge_sort.component.merge_sort", Language::English, "Merge Sort");
        
        self.add_translation("merge_sort.component.merge_array", Language::Chinese, "合并数组");
        self.add_translation("merge_sort.component.merge_array", Language::English, "Merge Array");
        
        self.add_translation("merge_sort.component.output_result", Language::Chinese, "输出结果");
        self.add_translation("merge_sort.component.output_result", Language::English, "Output Result");
    }
}

/// Global translations instance
lazy_static::lazy_static! {
    pub static ref TRANSLATIONS: Translations = {
        let translations = Translations::new(Language::Chinese);
        translations.load_default_translations();
        translations
    };
}

/// Translate a key with optional language
pub fn translate(key: &str, language: Option<Language>) -> String {
    TRANSLATIONS.translate(key, language)
}

/// Translate a key with formatting arguments
pub fn translate_fmt(key: &str, language: Option<Language>, args: &[&str]) -> String {
    use std::fmt::Write;
    
    let translation = translate(key, language);
    
    // Simple format string replacement for {0}, {1}, etc.
    let mut result = String::new();
    let mut idx = 0;
    
    while idx < translation.len() {
        let c = translation.chars().nth(idx).unwrap();
        
        if c == '{' && idx + 1 < translation.len() {
            let next_c = translation.chars().nth(idx + 1).unwrap();
            
            if next_c.is_digit(10) {
                // Find the closing brace
                let end_idx = translation[idx + 2..].find('}');
                if let Some(end_idx) = end_idx {
                    let num_str = &translation[idx + 1..idx + 2 + end_idx];
                    if let Ok(arg_idx) = num_str.parse::<usize>() {
                        // Replace with argument if available
                        if arg_idx < args.len() {
                            result.push_str(args[arg_idx]);
                        } else {
                            // Keep the original placeholder if argument is missing
                            result.push_str(&translation[idx..idx + 2 + end_idx + 1]);
                        }
                    } else {
                        // Invalid placeholder, keep as is
                        result.push_str(&translation[idx..idx + 2 + end_idx + 1]);
                    }
                    
                    idx += 2 + end_idx + 1;
                    continue;
                }
            }
        }
        
        result.push(c);
        idx += 1;
    }
    
    result
}
