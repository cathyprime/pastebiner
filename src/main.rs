mod parsers;
mod utils;

// TODO: make code work

use crate::parsers::info;
use crate::parsers::list;
use clap::{arg, Command};
use dotenv_codegen::dotenv;
use reqwest::blocking::{self, multipart};

const API_URL: &str = "https://pastebin.com/api/api_post.php";

fn get_user_info(
    api_user_dev_key: String,
    api_user_key: String,
) -> Result<blocking::Response, reqwest::Error> {
    let form = multipart::Form::new()
        .text("api_dev_key", api_user_dev_key)
        .text("api_user_key", api_user_key)
        .text("api_option", "userdetails");

    blocking::Client::new().post(API_URL).multipart(form).send()
}

fn list_pastes(
    api_user_dev_key: String,
    api_user_key: String,
    api_results_limit: i16,
) -> Result<blocking::Response, reqwest::Error> {
    let form = multipart::Form::new()
        .text("api_dev_key", api_user_dev_key)
        .text("api_user_key", api_user_key)
        .text("api_result_limit", api_results_limit.to_string())
        .text("api_option", "list");

    blocking::Client::new().post(API_URL).multipart(form).send()
}

fn get_paste(
    api_user_dev_key: String,
    api_user_key: String,
    paste_code: String,
) -> Result<blocking::Response, reqwest::Error> {
    let form = multipart::Form::new()
        .text("api_dev_key", api_user_dev_key)
        .text("api_user_key", api_user_key)
        .text("api_paste_key", paste_code)
        .text("api_option", "show_paste");
    blocking::Client::new().post(API_URL).multipart(form).send()
}

fn get_public_paste(paste_code: &str) -> Result<blocking::Response, reqwest::Error> {
    blocking::Client::new()
        .get("https://pastebin.com/raw/".to_string() + paste_code)
        .send()
}

// fn get_file(file_name: Path) {
//     unimplemented!();
// }

fn cli() -> Command {
    Command::new("pastebin")
        .about("Tool to interact with pastebin.com from CLI")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(Command::new("info").about("get user account information"))
        .subcommand(Command::new("list").about("get all the pastes made by the user"))
        .subcommand(
            Command::new("get")
                .about("get the contents of a paste")
                .arg(arg!(<CODE> "code of a paste to get").required(true)),
        )
        .subcommand(
            Command::new("new")
                .about("create a new paste")
                .arg(arg!([FILE] "name of a file to upload"))
        )
        .subcommand(Command::new("delete").about("delete an existing paste"))
}

fn main() {
    let api_user_key = dotenv!("APIUSERKEY");
    let api_user_dev_key = dotenv!("APIUSERDEVKEY");

    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("info", _)) => {
            match get_user_info(api_user_dev_key.to_string(), api_user_key.to_string()) {
                Ok(v) => match serde_xml_rs::from_reader::<blocking::Response, info::Info>(v) {
                    Ok(v) => println!("{}", v),
                    Err(e) => eprintln!("{}", e),
                },
                Err(e) => eprintln!("{}", e),
            }
        }
        Some(("list", _)) => {
            match list_pastes(api_user_dev_key.to_string(), api_user_key.to_string(), 50) {
                Ok(v) => match serde_xml_rs::from_reader::<blocking::Response, Vec<list::Paste>>(v)
                {
                    Ok(v) => v.into_iter().for_each(|x| print!("{}", x)),
                    Err(e) => eprintln!("{}", e),
                },
                Err(e) => eprintln!("{}", e),
            }
        }
        Some(("get", sub_matches)) => {
            let paste_code = match sub_matches.get_one::<String>("CODE") {
                Some(v) => v.to_string(),
                None => "".to_string(),
            };
            match get_paste(
                api_user_dev_key.to_string(),
                api_user_key.to_string(),
                paste_code.to_string(),
            ) {
                Ok(v) => {
                    let text = match v.text() {
                        Ok(v) => v,
                        Err(e) => e.to_string(),
                    };
                    if text == "Bad API request, invalid permission to view this paste or invalid api_paste_key" {
                        eprintln!("another user's paste detected, fetching...");
                        match get_public_paste(&paste_code) {
                            Ok(v) => match v.text() {
                                Ok(v) => println!("{}", v),
                                Err(e) => eprintln!("failed to get the text of response: {}", e),
                            },
                            Err(e) => eprintln!("paste not found: {}", e),
                        }
                    } else {
                        println!("{}", text);
                    }
                }
                Err(e) => eprintln!("{}", e),
            }
        }
        Some(("delete", _)) => {
            // TODO: pastebin delete -> delete a paste
            todo!()
        }
        Some(("new", _)) => {
            // TODO: pastebin new -> create new paste
            todo!()
        }
        _ => unimplemented!(),
    };
}
