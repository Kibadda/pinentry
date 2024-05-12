mod commands;
mod state;

use std::process;
use std::str::FromStr;
use std::{
    fs::File,
    io::{stdin, Error},
};

use tempfile::tempdir;

use crate::{
    commands::{Command, GetinfoSubCommand},
    state::State,
};

fn main() -> Result<(), Error> {
    let mut state = State::default();

    println!("OK");

    loop {
        let mut input = String::new();

        stdin().read_line(&mut input)?;

        match handle(&input, &mut state) {
            Some(message) => message.iter().for_each(|m| println!("{m}")),
            None => {
                break;
            }
        }
    }

    Ok(())
}

fn handle(s: &str, state: &mut State) -> Option<Vec<String>> {
    match Command::from_str(s) {
        Ok(command) => match command {
            Command::Option => Some(vec![String::from("OK")]),
            Command::Getinfo { data } => match data {
                GetinfoSubCommand::Flavor => Some(vec![String::from("D rust"), String::from("OK")]),
                GetinfoSubCommand::Version => Some(vec![String::from("D 1.0"), String::from("OK")]),
                GetinfoSubCommand::Ttyinfo => Some(vec![
                    String::from("D - - - - 1000/1000 0"),
                    String::from("OK"),
                ]),
                GetinfoSubCommand::Pid => Some(vec![String::from("D 1"), String::from("OK")]),
            },
            Command::Setkeyinfo { data } => {
                if data == *"--clear" {
                    Some(vec![String::from("OK")])
                } else {
                    None
                }
            }
            Command::Setdesc { data } => {
                state.description = data;
                Some(vec![String::from("OK")])
            }
            Command::Setprompt { data } => {
                state.prompt = data;
                Some(vec![String::from("OK")])
            }
            Command::Getpin => getpin(state).map(|pin| vec![pin, String::from("OK")]),
            Command::Bye => Some(vec![String::from("OK")]),
        },
        Err(_) => None,
    }
}

fn getpin(state: &State) -> Option<String> {
    let tmpdir = tempdir().unwrap();
    let path = tmpdir.path().join("pin");
    File::create(&path).expect("test");

    match process::Command::new("kitty")
        .env("DESCRIPTION", state.description.clone())
        .env("PROMPT", state.prompt.clone())
        .env("TMP_FILE", path.clone())
        .arg("--class")
        .arg("kitty-pinentry")
        .arg("pinentry-terminal-client")
        .output()
    {
        Ok(_) => {
            let contents = std::fs::read_to_string(&path).expect("test");
            std::fs::remove_file(path).expect("test");

            Some(format!("D {contents}"))
        }
        Err(_) => None,
    }
}
