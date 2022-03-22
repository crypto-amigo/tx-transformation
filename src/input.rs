use crate::models::{ClientID, Ledger, TxID};
use csv::{Reader, StringRecord};
use rust_decimal::Decimal;
use std::{env, io::Read};
#[derive(Debug)]
pub enum InputError {
    NoEnoughArgs,
    TooManyArgs,
    CannotOpenFile,
    IllformedRecord,
    InvalidTxType,
    InvalidTxID,
    InvalidClientID,
    InvalidAmount,
}

pub fn parse_args() -> Result<Vec<String>, InputError> {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => Err(InputError::NoEnoughArgs),
        2 => Ok(args),
        _ => Err(InputError::TooManyArgs),
    }
}

pub fn parse_csv<R: Read>(ledger: &mut Ledger, reader: &mut Reader<R>) -> Result<(), InputError> {
    for result in reader.records() {
        let record = result.or(Err(InputError::IllformedRecord))?;
        __handle_record(ledger, record)?;
    }
    Ok(())
}

fn __handle_record(ledger: &mut Ledger, record: StringRecord) -> Result<(), InputError> {
    let tx_type = &record[0];
    let client_id: ClientID = record[1].parse().or(Err(InputError::InvalidClientID))?;
    let tx_id: TxID = record[2].parse().or(Err(InputError::InvalidTxID))?;
    match tx_type {
        "deposit" => {
            let amount: Decimal = record[3].parse().or(Err(InputError::InvalidAmount))?;

            ledger.handle_deposit(&client_id, &tx_id, &amount);
            Ok(())
        }
        "withdrawal" => {
            let amount: Decimal = record[3].parse().or(Err(InputError::InvalidAmount))?;

            ledger.handle_withdrawal(&client_id, &tx_id, &amount);
            Ok(())
        }
        "dispute" => {
            ledger.handle_dispute(&client_id, &tx_id);
            Ok(())
        }
        "resolve" => {
            ledger.handle_resolve(&client_id, &tx_id);
            Ok(())
        }
        "chargeback" => {
            ledger.handle_chargeback(&client_id, &tx_id);
            Ok(())
        }
        _ => Err(InputError::InvalidTxType),
    }
}
