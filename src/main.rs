mod parsers;
mod utils;

// TODO: make code work

use crate::parsers::info;
use crate::parsers::list;
use clap::{arg, ArgMatches, Command};
use dotenv_codegen::dotenv;
use reqwest::blocking::{self, multipart};
use std::result::Result;

const API_URL: &str = "https://pastebin.com/api/api_post.php";

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

fn api(form: multipart::Form) -> Result<blocking::Response, PastebinError> {
    Ok(blocking::Client::new()
        .post(API_URL)
        .multipart(form)
        .send()?)
}

fn get_public_paste(paste_code: &str) -> Result<blocking::Response, PastebinError> {
    Ok(blocking::Client::new()
        .get(format!("https://pastebin.com/raw/{}", paste_code))
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

fn match_command(
    matches: ArgMatches,
    api_user_dev_key: String,
    api_user_key: String,
) -> Result<(), PastebinError> {
    match matches.subcommand() {
        Some(("info", _)) => {
            let v = api(multipart::Form::new()
                .text("api_dev_key", api_user_dev_key)
                .text("api_user_key", api_user_key)
                .text("api_option", "userdetails"));
            println!(
                "{}",
                serde_xml_rs::from_reader::<blocking::Response, info::Info>(v?)?
            );
            Ok(())
        }
        Some(("list", _)) => {
            let v = api(multipart::Form::new()
                .text("api_dev_key", api_user_dev_key)
                .text("api_user_key", api_user_key)
                .text("api_result_limit", "50")
                .text("api_option", "list"));
            serde_xml_rs::from_reader::<blocking::Response, Vec<list::Paste>>(v?)?
                .into_iter()
                .for_each(|p| println!("{}", p));
            Ok(())
        }
        Some(("get", sub_matches)) => {
            let paste_code = match sub_matches.get_one::<String>("CODE") {
                Some(v) => v.to_string(),
                None => "".to_string(),
            };
            let v = api(multipart::Form::new()
                .text("api_dev_key", api_user_dev_key)
                .text("api_user_key", api_user_key)
                .text("api_paste_key", paste_code.clone())
                .text("api_option", "show_paste"));
            if v.is_err() {
                println!("{}", get_public_paste(&paste_code)?.text()?);
            } else {
                println!("{}", v?.text()?);
            }
            Ok(())
        }
        Some(("delete", _)) => {
            todo!("pastebin delete -> delete a paste")
        }
        Some(("new", _)) => {
            todo!("pastebin new -> create new paste")
        }
        _ => unimplemented!(),
    }
}

fn main() {
    let api_user_key = dotenv!("APIUSERKEY").to_string();
    let api_user_dev_key = dotenv!("APIUSERDEVKEY").to_string();

    let matches = cli().get_matches();

    if let Err(e) = match_command(matches, api_user_dev_key, api_user_key) {
        println!("{}", e);
    }
}
