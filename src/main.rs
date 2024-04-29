mod parsers;
mod utils;

// TODO: make code work

use crate::parsers::info;
use crate::parsers::list;
use clap::{arg, Command};
use dotenv_codegen::dotenv;
use reqwest::blocking::{self, multipart};
use std::result::Result;

const API_URL: &str = "https://pastebin.com/api/api_post.php";

enum Connection<T, E> {
    Info(Result<T, E>),
    List(Result<T, E>),
    Get(Result<T, E>),
    #[allow(dead_code)]
    Delete(Result<T, E>),
    #[allow(dead_code)]
    New(Result<T, E>),
}

#[derive(Debug)]
enum PastebinError {
    ConnectionError(reqwest::Error),
    EncodingError(serde_xml_rs::Error),
}

impl std::error::Error for PastebinError {}

impl From<reqwest::Error> for PastebinError {
    fn from(value: reqwest::Error) -> Self {
        Self::ConnectionError(value)
    }
}

impl From<serde_xml_rs::Error> for PastebinError {
    fn from(value: serde_xml_rs::Error) -> Self {
        Self::EncodingError(value)
    }
}

impl std::fmt::Display for PastebinError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PastebinError::ConnectionError(e) => {
                write!(f, "Pastebin returned an error: {}", e)
            }
            PastebinError::EncodingError(e) => {
                write!(f, "There was an error parsing results: {}", e)
            }
        }
    }
}

fn get_user_info(
    api_user_dev_key: String,
    api_user_key: String,
) -> Result<blocking::Response, PastebinError> {
    let form = multipart::Form::new()
        .text("api_dev_key", api_user_dev_key)
        .text("api_user_key", api_user_key)
        .text("api_option", "userdetails");

    Ok(blocking::Client::new()
        .post(API_URL)
        .multipart(form)
        .send()?)
}

fn list_pastes(
    api_user_dev_key: String,
    api_user_key: String,
    api_results_limit: i16,
) -> Result<blocking::Response, PastebinError> {
    let form = multipart::Form::new()
        .text("api_dev_key", api_user_dev_key)
        .text("api_user_key", api_user_key)
        .text("api_result_limit", api_results_limit.to_string())
        .text("api_option", "list");

    Ok(blocking::Client::new()
        .post(API_URL)
        .multipart(form)
        .send()?)
}

fn get_paste(
    api_user_dev_key: String,
    api_user_key: String,
    paste_code: String,
) -> Result<blocking::Response, PastebinError> {
    let form = multipart::Form::new()
        .text("api_dev_key", api_user_dev_key)
        .text("api_user_key", api_user_key)
        .text("api_paste_key", paste_code)
        .text("api_option", "show_paste");
    Ok(blocking::Client::new()
        .post(API_URL)
        .multipart(form)
        .send()?)
}

fn get_public_paste(paste_code: &str) -> Result<blocking::Response, PastebinError> {
    Ok(blocking::Client::new()
        .get("https://pastebin.com/raw/".to_string() + paste_code)
        .send()?)
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
                .arg(arg!([FILE] "name of a file to upload")),
        )
        .subcommand(Command::new("delete").about("delete an existing paste"))
}

fn handle_result(
    result: Connection<blocking::Response, PastebinError>,
) -> Result<(), PastebinError> {
    match result {
        Connection::Info(v) => {
            let info = serde_xml_rs::from_reader::<blocking::Response, info::Info>(v?)?
        }
        Connection::List(v) => todo!(),
        Connection::Get(v) => todo!(),
        Connection::Delete(v) => todo!(),
        Connection::New(v) => todo!(),
    };
    Ok(())
}

fn main() {
    let api_user_key = dotenv!("APIUSERKEY");
    let api_user_dev_key = dotenv!("APIUSERDEVKEY");

    let matches = cli().get_matches();

    let result = match matches.subcommand() {
        Some(("info", _)) => Connection::Info(get_user_info(
            api_user_dev_key.to_string(),
            api_user_key.to_string(),
        )),
        Some(("list", _)) => Connection::List(list_pastes(
            api_user_dev_key.to_string(),
            api_user_key.to_string(),
            50,
        )),
        Some(("get", sub_matches)) => {
            let paste_code = match sub_matches.get_one::<String>("CODE") {
                Some(v) => v.to_string(),
                None => "".to_string(),
            };
            let paste_result = get_paste(
                api_user_dev_key.to_string(),
                api_user_key.to_string(),
                paste_code.to_string(),
            );
            match paste_result {
                Ok(_) => Connection::Get(paste_result),
                Err(_) => Connection::Get(get_public_paste(&paste_code)),
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

    if let Err(e) = handle_result(result) {
        println!("{}", e);
    }
}
