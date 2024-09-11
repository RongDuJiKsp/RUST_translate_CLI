use clap::Parser;
use crate::cli::cli_handle::CliParsedWay;

mod cli;
mod config_save_and_load;
mod alias;

fn main() {
    match cli::cli_handle::CliHandler::parse().user_to_do() {
        CliParsedWay::AddConfig(_) => {
            println!("AddConfig command not implemented yet");
        }
        CliParsedWay::DelConfig(_) => {
            println!("DelConfig command not implemented yet");
        }
        CliParsedWay::TranslateWithParam(_, _, _) => {
            println!("TranslateWithParam command not implemented yet");
        }
        CliParsedWay::TranslateWithConfig(_, _) => {
            println!("TranslateWithConfig command not implemented yet");
        }
        CliParsedWay::Unknown => {
            panic!("未知的命令形式！输入-help查看命令");
        }
    }
}
