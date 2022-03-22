# TX transformation

The design is elaborated below.

## Hierarchy

There are mainly five modules:

- input: To read CSV and parse the records to transactions
- output: To write CSV from accounts
- models: Structs and enums to hold data
- handlers: Handler implementation of models on input. It's deliberately designed to be format-agnostic
- main: Main entry point. Every program should have one

## Models

We may consider deposit and withdrawal transactions as data, dispute, resolve and chargeback as commands. Deposit and withdrawal transactions change accounts directly while others change accounts indirectly and modify some existing deposit transaction's state.

### Transaction

There are five categories ("type" is not picked here since it's a Rust keyword): deposit, withdrawal, dispute, resolve and chargeback. Obviously deposit and withdrawal transactions are data since they hold amounts - and the other three are commands which just change the state of a previous transaction.

It's suggested that withdrawals cannot be disputed, so in order to simplify the logic, the commands only infect deposit transactions. Simply we have two categories and three states:

```rust
pub enum TxCategory {
    Deposit,
    Withdrawal,
}

pub enum TxState {
    Normal,
    Disputed,
    Chargedback,
}
```

And transaction is just a composition:

```rust
pub struct Transaction {
    pub category: TxCategory,
    pub state: TxState,
    pub amount: Decimal,
    pub client_id: ClientID,
}
```

### Account

Accounts hold client's current financial data: as documented, an account has 4 fields: available amount, held amount, total amount and a flag indicating whether it's locked.

```rust
pub struct Account {
    pub available: Decimal,
    pub held: Decimal,
    pub total: Decimal,
    pub locked: bool,
}
```

For each transaction we implement a handler respectively.

```rust
impl Account {
    // assertions are redacted here
    pub fn handle_deposit(&mut self, amount: &Decimal) {
        self.available += amount;
        self.total += amount;
    }

    pub fn handle_withdrawal(&mut self, amount: &Decimal) {
        self.available -= amount;
        self.total -= amount;
    }

    pub fn handle_dispute(&mut self, amount: &Decimal) {
        self.available -= amount;
        self.held += amount;
    }

    pub fn handle_resolve(&mut self, amount: &Decimal) {
        self.available += amount;
        self.held -= amount;
    }

    pub fn handle_chargeback(&mut self, amount: &Decimal) {
        self.total -= amount;
        self.held -= amount;
        self.locked = true;
    }
}
```

### Ledger

A ledger holds all the data of transactions and accounts. We assume that transaction IDs are not overlapping so `HashMap` can be a good data structure.

```rust
pub struct Ledger {
    pub accounts: HashMap<ClientID, Account>,
    pub transactions: HashMap<TxID, Transaction>,
}
```

## Handlers

For a deposit transaction, insert it to `ledger.transactions` then check whether the client ID is in `ledger.accounts`. If not, create an account with default values and insert it into `ledger.accounts`. Otherwise, delegate the calculation to `account.handle_deposit` by invoking it.

For a withdrawal transaction, do not insert it to `ledger.transactions` since we don't handle it (surely we could if we want). Check whether the client ID exists, if not, ignore it. Otherwise check the account's available amount is greater or equal than input amount, if not, ignore it. Otherwise call `account.handle_withdrawal`.

For a dispute transaction, check whether the account id and the transaction id exist. If so, check the transaction is valid to be disputed. If it's valid, set the transaction state to `TxState::Disputed` and call `account.handle_dispute`.

For a resolve transaction, check whether the account id and the transaction id exist. If so, check the transaction is valid to be resolved. If it's valid, reset the transaction state to `TxState::Normal` and call `account.handle_resolve`.

For a chargeback transaction, check whether the account id and the transaction id exist. If so, check the transaction is valid to be charged back. If it's valid, set the transaction state to `TxState::ChargedBack` and call `account.handle_chargeback`.

## Error handling

For simplification, any illformed file input and output error will result in exit of the program and any invalid transaction will be ignored. Third party errors will be converted into custom errors like below.

```rust
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

pub enum OutputError {
    FailedToWrite,
    FailedToFlush,
}

pub enum Error {
    Input(InputError),
    Output(OutputError),
}
```

## Performance

The ledger has to keep the record of accounts and deposit transactions with two hash tables. Each transaction from the input will be handled at most once, so the time and space complexity is O(n).
