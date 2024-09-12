use crate::alias::param_alias::{ConfigName, FromLang, TargetLang};
use crate::util;
use anyhow::{anyhow, Result};
use std::path::Path;
use tokio::fs::File;
use tokio::io::{ AsyncReadExt};

const SPLIT_NAME: &str = "=";
const SPLIT_FLAG: &str = "|";
#[derive(Debug)]
pub struct TransConfig {
    pub name: String,
    pub from_lang: String,
    pub target_lang: String,
}
impl TransConfig {
    pub async fn from_file_or_create(path: &str) -> Result<Vec<TransConfig>> {
        let p = Path::new(path);
        if p.exists() {
            return Ok(Self::from_file(path).await?);
        }
        util::file_sys::create_parent_dir(path).await?;
        File::create(p).await?;
        Ok(Vec::new())
    }
    pub async fn from_file(path: &str) -> Result<Vec<TransConfig>> {
        let mut read_buf = Vec::new();
        File::open(path)
            .await?
            .read_to_end(&mut read_buf)
            .await?;
        let res = read_buf
            .split(|&c| c == b'\n')
            .map(|x| Ok(TransConfig::from_one_line(std::str::from_utf8(x)?)?))
            .filter(|x| x.is_ok())
            .map(|x: anyhow::Result<Option<TransConfig>>| x.unwrap())
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .collect::<Vec<_>>();
        Ok(res)
    }
    pub fn from_one_line(line: &str) -> Result<Option<TransConfig>> {
        if line.is_empty() {
            return Ok(None);
        }
        let units = line.split(SPLIT_NAME).collect::<Vec<_>>();
        if units.len() != 2 {
            return Err(anyhow!("配置文件存在异常，需要 SPLIT_NAME_FLAG"));
        }
        let flags = units[1].split(SPLIT_FLAG).collect::<Vec<_>>();
        if flags.len() != 2 {
            return Err(anyhow!("配置文件存在异常，需要 SPLIT_LANG_FLAG"));
        }
        let (name, from_lang, target_lang) = (units[0].to_string(), flags[0].to_string(), flags[1].to_string());
        Ok(Some(TransConfig { name, from_lang, target_lang }))
    }
    pub fn to_line(&self) -> Vec<u8> {
        Vec::from(format!("{}{}{}{}{}\n", &self.name, SPLIT_NAME, &self.from_lang, SPLIT_FLAG, &self.target_lang))
    }
    pub fn from_val_to_line(name: &ConfigName, from_lang: &FromLang, target_lang: &TargetLang) -> Vec<u8> {
        Vec::from(format!("{}{}{}{}{}\n", name, SPLIT_NAME, from_lang, SPLIT_FLAG, target_lang))
    }
}