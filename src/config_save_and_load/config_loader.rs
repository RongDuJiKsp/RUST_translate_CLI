use crate::alias::param_alias::{ConfigName, FromLang, TargetLang};
use crate::config_save_and_load::configure::TransConfig;
use crate::util::impls::AsyncClose;
use anyhow::anyhow;
use std::cmp::PartialEq;
use std::collections::HashMap;
use tokio::fs::File;
use tokio::io;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[derive(PartialEq, Debug)]
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
#[derive(Debug)]
pub struct ConfigLoader {
    config_path: String,
    loaded_config: HashMap<String, TransConfig>,
    change_log: Vec<ConfigChangeLog>,
}
impl ConfigLoader {
    pub async fn from_path(config_path: &str) -> anyhow::Result<ConfigLoader> {
        let cfg = TransConfig::from_file_or_create(config_path).await?;
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
impl AsyncClose for ConfigLoader {
    async fn async_close(self) -> anyhow::Result<()> {
        if self.change_log.is_empty() {
            return Ok(());
        }
        let to_delete = self.change_log.iter().filter(|x| x.is_delete()).map(|x| x.as_delete()).collect::<Vec<ConfigName>>();
        let mut last_file_buf: Vec<u8> = Vec::new();
        File::options()
            .read(true)
            .open(&self.config_path)
            .await?
            .read_to_end(&mut last_file_buf)
            .await?;
        let file = File::options()
            .write(true)
            .open(&self.config_path)
            .await?;
        file.set_len(0).await?;
        let mut buf_writer = io::BufWriter::new(file);
        for line in last_file_buf.split(|x| *x == b'\n') {
            if let Ok(Some(cfg)) = TransConfig::from_one_line(std::str::from_utf8(line)?) {
                if to_delete.contains(&cfg.name) {
                    continue;
                }
                buf_writer.write(&cfg.to_line())
                    .await?;
            }
        }
        for log in &self.change_log {
            if let ConfigChangeLog::New(config_name, from, target) = log {
                buf_writer.write(&TransConfig::from_val_to_line(config_name, from, target)).await?;
            }
        }
        buf_writer.flush().await?;
        Ok(())
    }
}

