use clap::Parser;
use crate::alias::param_alias::{TargetLang, FromLang, ConfigName, ToTranslatePlain};
pub enum CliParsedWay {
    AddConfig((ConfigName, FromLang, TargetLang)),
    DelConfig(ConfigName),
    TranslateWithParam(ToTranslatePlain, FromLang, TargetLang),
    TranslateWithConfig(ToTranslatePlain, ConfigName),
    Unknown,
}

#[derive(Parser)]
#[command(
    version,
    author,
    about = "一个简单的翻译命令行工具，支持配置保存、删除和语言选择",
    long_about = "
    这是一个用于翻译文本的命令行工具。你可以指定源语言和目标语言，还可以保存和删除配置。
    示例:
    rust-trans-cli.exe -p <plain> -f <from_lang> -t <to_lang>  # 翻译 plain
    rust-trans-cli.exe -c <config_name> -s -f <from_lang> -t <to_lang>  # 将翻译策略保存至配置
    rust-trans-cli.exe -c <config_name> -d  # 删除翻译策略配置
    rust-trans-cli.exe -c <config_name> -p <plain>  # 使用翻译策略进行翻译
    "
)]
pub struct CliHandler {
    /// 配置名称的选项参数
    #[arg(short = 'c', long = "config_name", help = "指定要使用的配置名称")]
    pub config_name: Option<String>,
    /// 删除配置的布尔标记
    #[arg(short = 'd', long = "del_config", help = "删除指定的配置")]
    pub del_config: bool,
    /// 保存配置的布尔标记
    #[arg(short = 's', long = "save_config", help = "保存当前的配置")]
    pub save_config: bool,
    /// 要翻译的文本
    #[arg(short = 'p', long = "plain", help = "要翻译的文本内容")]
    pub to_translate_text: Option<String>,
    /// 源语言
    #[arg(short = 'f', long = "from_lang", help = "源语言代码，例如 'en' 表示英语")]
    pub from_lang: Option<String>,
    /// 目标语言
    #[arg(short = 't', long = "target_lang", help = "目标语言代码，例如 'zh' 表示中文")]
    pub target_lang: Option<String>,
}
impl CliHandler {
    pub fn user_to_do(&self) -> CliParsedWay {
        //When Config Is Full , Save Config Or Translate With Config
        if let (Some(from_lang), Some(target_lang)) = (&self.from_lang, &self.target_lang) {
            //When With Plain,Translate With Config
            if let Some(to_translate) = &self.to_translate_text {
                return CliParsedWay::TranslateWithParam(to_translate.clone(), from_lang.clone(), target_lang.clone());
            }
            //when with Config Name And Flag Save Config
            if self.save_config {
                if let Some(config_name) = &self.config_name {
                    return CliParsedWay::AddConfig((config_name.clone(), from_lang.clone(), target_lang.clone()));
                }
            }
        }
        if self.del_config {
            if let Some(config_name) = &self.config_name {
                return CliParsedWay::DelConfig(config_name.clone());
            }
        }
        if let (Some(config_name), Some(to_trans_text)) = (&self.config_name, &self.to_translate_text) {
            return CliParsedWay::TranslateWithConfig(to_trans_text.clone(), config_name.clone());
        }
        CliParsedWay::Unknown
    }
}