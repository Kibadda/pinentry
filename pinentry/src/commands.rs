use std::str::FromStr;

pub enum Command {
    Option,
    Getinfo { data: GetinfoSubCommand },
    Setkeyinfo { data: String },
    Setdesc { data: String },
    Setprompt { data: String },
    Getpin,
    Bye,
}

pub enum GetinfoSubCommand {
    Flavor,
    Version,
    Ttyinfo,
    Pid,
}

impl FromStr for GetinfoSubCommand {
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

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s.split_whitespace().collect();

        match split[0] {
            "OPTION" => Ok(Self::Option),
            "GETINFO" => Ok(Self::Getinfo {
                data: GetinfoSubCommand::from_str(split[1])?,
            }),
            "SETKEYINFO" => Ok(Self::Setkeyinfo {
                data: split[1..].join(" "),
            }),
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
