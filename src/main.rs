use crate::cli::cli_handle::CliParsedWay;
use crate::config_save_and_load::config_loader::ConfigLoader;
use clap::Parser;
use crate::util::impls::{AsyncClose, AsyncDroppable, Closable};

mod cli;
mod config_save_and_load;
mod alias;
mod sdk;
mod util;

#[tokio::main]
async fn main() {
    let sdk = sdk::caller::TencentCloudTranslateSDK::from_env()
        .expect("SDK Key未配置！请配置后使用");
    let mut cfg = ConfigLoader::from_path("./.config/rust_trans_cli")
        .await
        .expect("加载配置失败");
    match cli::cli_handle::CliHandler::parse().user_to_do() {
        CliParsedWay::AddConfig(name, from, to) => {
            cfg.save_config(&name, &from, &to).expect("保存配置失败！");
        }
        CliParsedWay::DelConfig(name) => {
            cfg.delete_config(&name).expect("删除配置失败！");
        }
        CliParsedWay::TranslateWithParam(p, from, target) => {
            print!("{}", sdk.translate_text(&p, &from, &target).await.expect("failed to translate text"));
        }
        CliParsedWay::TranslateWithConfig(p, c_name) => {
            let (from, target) = cfg.load_config(&c_name).expect("加载配置失败！");
            print!("{}", sdk.translate_text(&p, &from, &target).await.expect("failed to translate text"));
        }
        CliParsedWay::Unknown => {
            panic!("未知的命令形式！输入--help查看命令");
        }
    }
    cfg.async_close().await.expect("在清理文件时发生问题");
}
#[tokio::test]
async fn test() {}
