mod handlers;
mod input;
mod models;
mod output;

use csv::{Reader, Writer};
use input::{parse_args, parse_csv, InputError};
use models::Ledger;
use output::{export_csv, OutputError};
use std::io;
use std::process;

#[derive(Debug)]
pub enum Error {
    Input(InputError),
    Output(OutputError),
}

fn run() -> Result<(), Error> {
    let args: Vec<String> = parse_args().map_err(Error::Input)?;
    let path = &args[1];
    let mut reader = Reader::from_path(path).or(Err(Error::Input(InputError::CannotOpenFile)))?;
    let mut ledger = Ledger::default();
    parse_csv(&mut ledger, &mut reader).map_err(Error::Input)?;
    let mut wtr = Writer::from_writer(io::stdout());
    export_csv(&ledger, &mut wtr).map_err(Error::Output)?;
    Ok(())
}

fn main() {
    if let Err(err) = run() {
        println!("{:?}", err);
        process::exit(1);
    }
}
