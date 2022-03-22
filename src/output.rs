use std::io::Write;

use crate::models::Ledger;
use csv::Writer;

#[derive(Debug)]
pub enum OutputError {
    FailedToWrite,
    FailedToFlush,
}

pub fn export_csv<W: Write>(ledger: &Ledger, wtr: &mut Writer<W>) -> Result<(), OutputError> {
    wtr.write_record(&["client", "available", "total", "held", "locked"])
        .or(Err(OutputError::FailedToWrite))?;
    for (client_id, account) in ledger.accounts.iter() {
        wtr.write_record(&[
            client_id.to_string(),
            account.available.to_string(),
            account.total.to_string(),
            account.held.to_string(),
            account.locked.to_string(),
        ])
        .or(Err(OutputError::FailedToWrite))?;
    }
    wtr.flush().or(Err(OutputError::FailedToFlush))?;
    Ok(())
}
