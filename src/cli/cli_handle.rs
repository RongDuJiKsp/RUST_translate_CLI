use clap::Parser;
pub type FromLang = String;
pub type TargetLang = String;
pub type ConfigName = String;
pub type ToTranslatePlain = String;
pub enum CliParsedWay {
    AddConfig((ConfigName, FromLang, TargetLang)),
    DelConfig(ConfigName),
    TranslateWithParam(ToTranslatePlain, FromLang, TargetLang),
    TranslateWithConfig(ToTranslatePlain, ConfigName),
    Unknown,
}

#[derive(Parser)]
#[command(version, author, about, long_about = None)]
pub struct CliHandler {
    #[arg(short = 'c', long = "config_name")]
    pub config_name: Option<ConfigName>,
    #[arg(short = 'd', long = "del_config")]
    pub del_config: bool,
    #[arg(short = 's', long = "save_config")]
    pub save_config: bool,
    #[arg(short = 'l', long = "load_config")]
    pub load_config: bool,
    #[arg(short = 'p', long = "plain")]
    pub to_translate_text: Option<ToTranslatePlain>,
    #[arg(short = 'f', long = "from_lang")]
    pub from_lang: Option<FromLang>,
    #[arg(short = 't', long = "target_lang")]
    pub target_lang: Option<TargetLang>,
}
impl CliHandler {
    pub fn user_to_do(&self) -> CliParsedWay {
        //When Config Is Full , Save Config Or Translate With Config
        if let ((Some(from_lang), Some(target_lang))) = (&self.from_lang, &self.target_lang) {
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
        if self.load_config {
            if let (Some(config_name), Some(to_trans_text)) = (&self.config_name, &self.to_translate_text) {
                return CliParsedWay::TranslateWithConfig(to_trans_text.clone(), config_name.clone());
            }
        }
        if self.del_config {
            if let Some(config_name) = &self.config_name {
                return CliParsedWay::DelConfig(config_name.clone());
            }
        }
        CliParsedWay::Unknown
    }
}