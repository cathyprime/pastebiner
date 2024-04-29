mod parsers;
mod utils;

// TODO: make code work

use crate::parsers::list;
use crate::parsers::info;
use clap::Command;
use dotenv_codegen::dotenv;
use reqwest::blocking::{self, multipart};

const API_URL: &str = "https://pastebin.com/api/api_post.php";

#[allow(dead_code)]
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

fn cli() -> Command {
    Command::new("pastebin")
        .about("Tool to interact with pastebin.com from CLI")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(Command::new("info").about("get user account information"))
        .subcommand(Command::new("list").about("get all the pastes made by the user"))
        .subcommand(Command::new("get").about("get the contents of a paste"))
        .subcommand(Command::new("new").about("create a new paste"))
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
        },
        Some(("list", _)) => {
            match list_pastes(api_user_dev_key.to_string(), api_user_key.to_string(), 50) {
                Ok(v) => match serde_xml_rs::from_reader::<blocking::Response, Vec<list::Paste>>(v) {
                    Ok(v) => v.into_iter().for_each(|x| print!("{}", x)),
                    Err(e) => eprintln!("{}", e),
                },
                Err(e) => eprintln!("{}", e),
            }
        }
        Some(("get", _)) => {
            todo!()
            // TODO: pastebin get -> get specific paste
        }
        Some(("new", _)) => {
            todo!()
            // TODO: pastebin new -> create new paste
        }
        Some(("delete", _)) => {
            todo!()
            // TODO: pastebin delete -> delete a paste
        }
        _ => unimplemented!(),
    };
}
