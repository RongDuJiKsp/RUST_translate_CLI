use crate::alias::param_alias::{ConfigName, FromLang, TargetLang};
use crate::config_save_and_load::configure::{TransConfig};
use anyhow::anyhow;
use std::cmp::PartialEq;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{BufRead, Write};
#[derive(PartialEq)]
enum ConfigChangeLog {
    New(ConfigName, FromLang, TargetLang),
    Delete(ConfigName),
}
impl ConfigChangeLog {
    fn as_delete(&self) -> ConfigName {
        if let ConfigChangeLog::Delete(ref name) = *self {
            return name.clone();
        }
        panic!("ConfigChangeLog::as_delete called on non-delete ConfigChangeLog");
    }
    fn is_delete(&self) -> bool {
        if let ConfigChangeLog::Delete(_) = *self {
            true
        } else {
            false
        }
    }
}
pub struct ConfigLoader {
    config_path: String,
    loaded_config: HashMap<String, TransConfig>,
    change_log: Vec<ConfigChangeLog>,
}
impl ConfigLoader {
    pub fn from_path(config_path: &str) -> anyhow::Result<ConfigLoader> {
        let cfg = TransConfig::from_file_or_create(config_path)?;
        let loaded_config = cfg.into_iter().fold(HashMap::new(), |mut acc, x| {
            acc.insert(x.name.clone(), x);
            return acc;
        });
        let change_log = Vec::new();
        Ok(ConfigLoader { config_path: config_path.to_string(), loaded_config, change_log })
    }
    pub fn load_config(&self, config_name: &ConfigName) -> anyhow::Result<(FromLang, TargetLang)> {
        if let Some(conf) = self.loaded_config.get(config_name) {
            return Ok((conf.from_lang.clone(), conf.target_lang.clone()));
        }
        Err(anyhow!("不存在叫做{}的配置文件", config_name))
    }
    pub fn save_config(&mut self, config_name: &ConfigName, from_lang: &FromLang, target: &TargetLang) -> anyhow::Result<()> {
        self.change_log.push(ConfigChangeLog::New(config_name.clone(), from_lang.clone(), target.clone()));
        Ok(())
    }
    pub fn delete_config(&mut self, config_name: &ConfigName) -> anyhow::Result<()> {
        self.change_log.push(ConfigChangeLog::Delete(config_name.clone()));
        Ok(())
    }
}


impl Drop for ConfigLoader {
    fn drop(&mut self) {
        let to_delete = self.change_log.iter().filter(|x| x.is_delete()).map(|x| x.as_delete()).collect::<Vec<ConfigName>>();
        let mut buf_writer: Vec<u8> = Vec::new();
        {
            let file = File::options()
                .read(true)
                .open(self.config_path.as_str())
                .expect("写回配置文件时打开文件读取发生错误！");
            for line in io::BufReader::new(file).lines() {
                let cfg = TransConfig::from_one_line(&line.expect("写回配置时读取配置文件发生错误"));
                if let Err(_) = cfg {
                    continue;
                }
                let cfg = cfg.unwrap();
                if to_delete.contains(&cfg.name) {
                    continue;
                }
                buf_writer.extend_from_slice(&cfg.to_line());
            }
        }
        for log in &self.change_log {
            if let ConfigChangeLog::New(config_name, from, target) = log {
                buf_writer.extend_from_slice(&TransConfig::from_val_to_line(config_name, from, target))
            }
        }
        File::options()
            .write(true)
            .open(&self.config_path)
            .expect("写回配置文件时打开文件发生错误！")
            .write(&buf_writer)
            .expect("写回配置文件时打开文件发生错误！");
    }
}