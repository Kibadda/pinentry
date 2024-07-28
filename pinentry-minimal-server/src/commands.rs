use std::{error::Error, fmt::Display, fs::File, process, str::FromStr};
use tempfile::tempdir;

use crate::state::State;

pub enum Request {
    Option,
    Getinfo { data: GetinfoRequest },
    Setkeyinfo,
    Setdesc { data: String },
    Setprompt { data: String },
    Getpin,
    Bye,
}

pub enum GetinfoRequest {
    Flavor,
    Version,
    Ttyinfo,
    Pid,
}

pub enum Response {
    Ok,
    D { data: String },
}

impl Request {
    pub fn handle(&self, state: &mut State) -> Vec<Response> {
        match self {
            Self::Option => vec![Response::Ok],
            Self::Getinfo { data } => match data {
                GetinfoRequest::Flavor => vec![
                    Response::D {
                        data: String::from("rust"),
                    },
                    Response::Ok,
                ],
                GetinfoRequest::Version => vec![
                    Response::D {
                        data: env!("CARGO_PKG_VERSION").to_owned(),
                    },
                    Response::Ok,
                ],
                GetinfoRequest::Ttyinfo => vec![
                    Response::D {
                        data: String::from("- - - - 1000/1000 0"),
                    },
                    Response::Ok,
                ],
                GetinfoRequest::Pid => vec![
                    Response::D {
                        data: process::id().to_string(),
                    },
                    Response::Ok,
                ],
            },
            Self::Setkeyinfo => vec![Response::Ok],
            Self::Setdesc { data } => {
                state.description = data.to_string();
                vec![Response::Ok]
            }
            Self::Setprompt { data } => {
                state.prompt = data.to_string();
                vec![Response::Ok]
            }
            Self::Getpin => match getpin(state) {
                Ok(response) => response,
                Err(_) => vec![],
            },
            Self::Bye => vec![Response::Ok],
        }
    }
}

impl FromStr for Request {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s.split_whitespace().collect();

        if split.is_empty() {
            return Err(());
        }

        match split[0] {
            "OPTION" => Ok(Self::Option),
            "GETINFO" => Ok(Self::Getinfo {
                data: GetinfoRequest::from_str(split[1])?,
            }),
            "SETKEYINFO" => Ok(Self::Setkeyinfo),
            "SETDESC" => Ok(Self::Setdesc {
                data: split[1..].join(" "),
            }),
            "SETPROMPT" => Ok(Self::Setprompt {
                data: split[1..].join(" "),
            }),
            "GETPIN" => Ok(Self::Getpin),
            "BYE" => Ok(Self::Bye),
            _ => Err(()),
        }
    }
}

impl FromStr for GetinfoRequest {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "flavor" => Ok(Self::Flavor),
            "version" => Ok(Self::Version),
            "ttyinfo" => Ok(Self::Ttyinfo),
            "pid" => Ok(Self::Pid),
            _ => Err(()),
        }
    }
}

impl Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Ok => String::from("OK"),
                Self::D { data } => format!("D {}", data),
            }
        )
    }
}

fn getpin(state: &State) -> Result<Vec<Response>, Box<dyn Error>> {
    let tmpdir = tempdir()?;
    let path = tmpdir.path().join("pin");
    File::create(&path)?;

    match process::Command::new("kitty")
        .env("DESCRIPTION", state.description.clone())
        .env("PROMPT", state.prompt.clone())
        .env("TMP_FILE", path.clone())
        .arg("--class")
        .arg("kitty-pinentry")
        .arg("pinentry-minimal-client")
        .output()
    {
        Ok(_) => {
            let contents = std::fs::read_to_string(&path)?;
            std::fs::remove_file(path)?;

            Ok(match contents.is_empty() {
                false => vec![Response::D { data: contents }, Response::Ok],
                true => vec![],
            })
        }
        Err(_) => Ok(vec![]),
    }
}
