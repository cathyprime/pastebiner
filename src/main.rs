mod parsers;
mod utils;

use crate::parsers::info;
use crate::parsers::list;
use clap::{arg, ArgMatches, Command};
use dotenv_codegen::dotenv;
use reqwest::blocking::{self, multipart};
use std::fs;
use std::io::{self, BufRead};
use std::path::Path;
use std::result::Result;
use utils::private::Privacy;

const API_URL: &str = "https://pastebin.com/api/api_post.php";

#[derive(Debug)]
enum PastebinError {
    Connection(reqwest::Error),
    Encoding(serde_xml_rs::Error),
    File(String),
    Io(io::Error),
    Arg(String, String),
}

impl std::error::Error for PastebinError {}

impl From<reqwest::Error> for PastebinError {
    fn from(value: reqwest::Error) -> Self {
        Self::Connection(value)
    }
}

impl From<serde_xml_rs::Error> for PastebinError {
    fn from(value: serde_xml_rs::Error) -> Self {
        Self::Encoding(value)
    }
}

impl From<io::Error> for PastebinError {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

impl std::fmt::Display for PastebinError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PastebinError::Connection(e) => {
                write!(f, "Pastebin returned an error: {}", e)
            }
            PastebinError::Encoding(e) => {
                write!(f, "There was an error parsing results: {}", e)
            }
            PastebinError::File(file) => {
                write!(f, "File `{}` wasn't found", file)
            }
            PastebinError::Io(io) => {
                write!(f, "Io error: {}", io)
            }
            PastebinError::Arg(argument, content) => {
                write!(f, "Argument `{}` error: {}", argument, content)
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

fn cli() -> Command {
    Command::new("pastebin")
        .about("Tool to interact with pastebin.com from CLI")
        .subcommand_required(true)
        .args(&[
            arg!(-u --apiuserkey <USERKEY> "provide a different api_user_key"),
            arg!(-d --apidevkey <DEVKEY> "provide a different api_dev_key"),
        ])
        .arg_required_else_help(true)
        .subcommand(Command::new("info").about("get user account information"))
        .subcommand(Command::new("list").about("get all the pastes made by the user"))
        .subcommand(
            Command::new("get")
                .about("get the contents of a paste")
                .arg(arg!(<CODE> "code of a paste to get")),
        )
        .subcommand(
            Command::new("new").about("create a new paste").args(&[
                arg!([FILE] "name of a file to upload"),
                arg!(-g --guest "upload as a guest").action(clap::ArgAction::SetTrue),
                arg!(-t --title <TITLE> "set the title of the paste"),
                arg!(-s --syntax <SYNTAX> "set the syntax of the paste"),
                arg!(-e --expire <EXPIRE> "set expiration time")
                    .long_help(
                        [
                            "possible values:",
                            "N -> Never expire",
                            "10M -> 10 minutes",
                            "1H -> 1 hour",
                            "1D -> 1 day",
                            "1W -> 1 week",
                            "2W -> 2 weeks",
                            "1M -> 1 month",
                            "6M -> 6 months",
                            "1Y -> 1 year",
                        ]
                        .join("\n"),
                    )
                    .value_parser(["N", "10M", "1H", "1D", "1W", "2W", "1M", "6M", "1Y"])
                    .default_value("N"),
                arg!(-p --privacy <LEVEL> "set the level of privacy").long_help(
                    [
                        "`0` or `public` => will be Public",
                        "`1` or `unlisted` => will be Unlisted",
                        "`2` or `private` => will be Private",
                    ]
                    .join("\n"),
                ),
            ]),
        )
        .subcommand(
            Command::new("delete")
                .about("delete an existing paste")
                .arg(arg!(<CODE> "code of a paste to delete")),
        )
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
            let text = v?.text()?;
            if &text
                == "Bad API request, invalid permission to view this paste or invalid api_paste_key"
            {
                println!("{}", get_public_paste(&paste_code)?.text()?);
            } else {
                println!("{}", text);
            }
            Ok(())
        }
        Some(("delete", sub_matches)) => {
            let code = match sub_matches.get_one::<String>("CODE") {
                Some(v) => v.to_string(),
                None => Err(PastebinError::Arg("code".to_string(), "error getting the code".to_string()))?,
            };
            let v = api(multipart::Form::new()
                .text("api_dev_key", api_user_dev_key)
                .text("api_user_key", api_user_key)
                .text("api_option", "delete")
                .text("api_paste_key", code.to_string())
            );
            println!("{}", v?.text()?);
            Ok(())
        }
        Some(("new", sub_matches)) => {
            let file = match sub_matches.get_one::<String>("FILE") {
                Some(user_input) => {
                    let path = Path::new(user_input);
                    match path.to_str() {
                        None => Err(PastebinError::File(user_input.to_string()))?,
                        Some(p) => {
                            if path.is_file() {
                                fs::read_to_string(p)?.to_string()
                            } else {
                                Err(PastebinError::File(p.to_string()))?
                            }
                        }
                    }
                }
                None => {
                    if atty::isnt(atty::Stream::Stdin) {
                        io::stdin()
                            .lock()
                            .lines()
                            .map(|l| l.expect("Couldn't read from stdin"))
                            .collect::<Vec<String>>()
                            .join("\n")
                    } else {
                        Err(PastebinError::File("".to_string()))?
                    }
                }
            };
            let mut form = multipart::Form::new()
                .text("api_dev_key", api_user_dev_key)
                .text("api_option", "paste")
                .text("api_paste_code", file);

            if !sub_matches.get_flag("guest") {
                form = form.text("api_user_key", api_user_key);
            }

            if let Some(expire) = sub_matches.get_one::<String>("expire") {
                form = form.text("api_paste_expire_date", expire.to_string());
            }

            if let Some(title) = sub_matches.get_one::<String>("title") {
                form = form.text("api_paste_name", title.to_string());
            }

            if let Some(level) = sub_matches.get_one::<String>("privacy") {
                match Privacy::try_from(level.to_string()) {
                    Ok(l) => form = form.text("api_paste_private", l.form_ready()),
                    Err(e) => Err(PastebinError::Arg("privacy".to_string(), e))?,
                };
            }

            if let Some(syntax) = sub_matches.get_one::<String>("syntax") {
                form = form.text("api_paste_format", syntax.to_string());
            }

            let v = api(form);
            println!("{}", v?.text()?);

            Ok(())
        }
        _ => unimplemented!(),
    }
}

fn main() {
    let api_user_key = dotenv!("APIUSERKEY").to_string();
    let api_user_dev_key = dotenv!("APIUSERDEVKEY").to_string();

    let matches = cli().get_matches();

    if let Err(e) = match_command(matches, api_user_dev_key, api_user_key) {
        eprintln!("{}", e);
    }
}
