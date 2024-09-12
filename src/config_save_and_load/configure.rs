use crate::util;
use anyhow::{anyhow, Result};
use std::fs::File;
use std::io::BufRead;
use std::path::Path;
use std::io;
use crate::alias::param_alias::{ConfigName, FromLang, TargetLang};

pub const SPLIT_NAME: &str = "=";
pub const SPLIT_FLAG: &str = "|";
pub struct TransConfig {
    pub name: String,
    pub from_lang: String,
    pub target_lang: String,
}
impl TransConfig {
    pub fn from_file_or_create(path: &str) -> Result<Vec<TransConfig>> {
        let p = Path::new(path);
        if p.exists() {
            return Self::from_file(path);
        }
        util::file_sys::create_parent_dir(path)?;
        File::create(p)?;
        Ok(Vec::new())
    }
    pub fn from_file(path: &str) -> Result<Vec<TransConfig>> {
        let file = File::open(path)?;
        let reader = io::BufReader::new(file);
        let res = reader
            .lines()
            .map(|x| {
                let line = x?;
                TransConfig::from_one_line(&line)
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(res)
    }
    pub fn from_one_line(line: &str) -> Result<TransConfig> {
        let units = line.split(SPLIT_NAME).collect::<Vec<_>>();
        if units.len() != 2 {
            return Err(anyhow!("Config Is Err"));
        }
        let flags = units[1].split(SPLIT_FLAG).collect::<Vec<_>>();
        if flags.len() != 2 {
            return Err(anyhow!("Config Is Err"));
        }
        let (name, from_lang, target_lang) = (units[0].to_string(), flags[0].to_string(), flags[1].to_string());
        Ok(TransConfig { name, from_lang, target_lang })
    }
    pub fn to_line(&self) -> Vec<u8> {
        Vec::from(format!("{}{}{}{}{}\n", &self.name, SPLIT_NAME, &self.from_lang, SPLIT_FLAG, &self.target_lang))
    }
    pub fn from_val_to_line(name: &ConfigName, from_lang: &FromLang, target_lang: &TargetLang) -> Vec<u8> {
        Vec::from(format!("{}{}{}{}{}\n", name, SPLIT_NAME, from_lang, SPLIT_FLAG, target_lang))
    }
}