use clap::Parser;

mod cli;
mod config_save_and_load;
mod alias;
mod sdk;
#[tokio::main]
async fn main() {
    let sdk = sdk::caller::TencentCloudTranslateSDK::from_env().expect("SDK Key未配置！请配置后使用");
    sdk.translate_text("Hello", &"en".to_string(), &"zh".to_string()).await.unwrap();
    // match cli::cli_handle::CliHandler::parse().user_to_do() {
    //     CliParsedWay::AddConfig(_) => {
    //         println!("AddConfig command not implemented yet");
    //     }
    //     CliParsedWay::DelConfig(_) => {
    //         println!("DelConfig command not implemented yet");
    //     }
    //     CliParsedWay::TranslateWithParam(_, _, _) => {
    //         println!("TranslateWithParam command not implemented yet");
    //     }
    //     CliParsedWay::TranslateWithConfig(_, _) => {
    //         println!("TranslateWithConfig command not implemented yet");
    //     }
    //     CliParsedWay::Unknown => {
    //         panic!("未知的命令形式！输入-help查看命令");
    //     }
    // }
}
