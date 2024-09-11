use anyhow::{anyhow, Result};
use std::fs::File;
use std::io;
use std::io::BufRead;
pub const SPLIT_NAME: &'static str = "=";
pub const SPLIT_FLAG: &'static str = "|";
pub struct TransConfig {
    name: String,
    from_lang: String,
    target_lang: String,
}
impl TransConfig {
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
}