mod commands;
mod state;

use std::io::{stdin, Error};
use std::str::FromStr;

use crate::{
    commands::{Request, Response},
    state::State,
};

fn main() -> Result<(), Error> {
    let mut state = State::default();

    println!("{}", Response::Ok);

    loop {
        let mut input = String::new();

        stdin().read_line(&mut input)?;

        match Request::from_str(&input) {
            Ok(command) => {
                let messages = command.handle(&mut state);

                match messages.is_empty() {
                    false => messages.iter().for_each(|m| println!("{m}")),
                    true => break,
                }
            }
            Err(_) => break,
        }
    }

    Ok(())
}
