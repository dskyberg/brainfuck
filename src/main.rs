#![doc = include_str!("../README.md")]
use std::fs;
use std::io::Cursor;

use clap::{command, Arg};
use error::BrainFuckError;

use crate::{error::Result, interpreter::Interpreter};

mod error;
mod interpreter;

/// Process command line args and execute the interpreter
fn main() -> Result<()> {
    let args = command!()
        .arg(Arg::new("data").short('d').long("data").default_value("10"))
        .arg(Arg::new("file"))
        .get_matches();

    // let input = "++++++++[>+++++++++<-]>.";
    let input = fs::read_to_string(
        args.get_one::<String>("file")
            .ok_or_else(|| Box::new(BrainFuckError::BadArgs))?,
    )?;
    let max_data = args
        .get_one::<String>("data")
        .ok_or_else(|| Box::new(BrainFuckError::BadArgs))?
        .parse::<usize>()?;
    //let mut stdin = std::io::stdin();
    let in_bytes: Vec<u8> = vec![0, 1, 2];
    let mut file = Cursor::new(in_bytes);

    let mut stdout = std::io::stdout();

    let mut interpreter = Interpreter::builder()
        .max_data(max_data)
        .tokens(&input)?
        .inbuf(&mut file)
        .outbuf(&mut stdout)
        .build()
        .expect("failed to build interpreter");

    interpreter.run().expect("failed to execute");

    Ok(())
}
