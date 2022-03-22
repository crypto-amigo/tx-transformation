use rust_decimal::prelude::Decimal;
use std::collections::HashMap;

pub type ClientID = u32;
pub type TxID = u32;

#[derive(Debug, PartialEq)]
pub enum TxCategory {
    Deposit,
    Withdrawal,
}

#[derive(Debug, PartialEq)]
pub enum TxState {
    Normal,
    Disputed,
    Chargedback,
}

#[derive(Debug, PartialEq)]
pub struct Transaction {
    pub category: TxCategory,
    pub state: TxState,
    pub amount: Decimal,
    pub client_id: ClientID,
}

#[derive(Debug, Default)]
pub struct Account {
    pub available: Decimal,
    pub held: Decimal,
    pub total: Decimal,
    pub locked: bool,
}

#[derive(Debug, Default)]
pub struct Ledger {
    pub accounts: HashMap<ClientID, Account>,
    pub transactions: HashMap<TxID, Transaction>, // assume tx ids not overlapping
}
