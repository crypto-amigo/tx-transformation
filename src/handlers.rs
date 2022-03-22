use rust_decimal::prelude::Decimal;

use crate::models::{Account, ClientID, Ledger, Transaction, TxCategory, TxID, TxState};
impl Transaction {
    pub fn from_deposit(client_id: ClientID, amount: Decimal) -> Self {
        Transaction {
            category: TxCategory::Deposit,
            state: TxState::Normal,
            amount,
            client_id,
        }
    }

    pub fn from_withdrawal(client_id: ClientID, amount: Decimal) -> Self {
        Transaction {
            category: TxCategory::Withdrawal,
            state: TxState::Normal,
            amount,
            client_id,
        }
    }
}

impl Account {
    // read only
    pub fn can_withdraw(&self, amount: &Decimal) -> bool {
        &self.available >= amount && &self.total >= amount && !self.locked
    }

    pub fn can_dispute(&self, amount: &Decimal) -> bool {
        &self.available >= amount
    }

    pub fn can_resolve(&self, amount: &Decimal) -> bool {
        &self.held >= amount
    }

    pub fn can_chargeback(&self, amount: &Decimal) -> bool {
        &self.total >= amount && &self.held >= amount
    }
}

impl Account {
    // mutable
    pub fn handle_deposit(&mut self, amount: &Decimal) {
        self.available += amount;
        self.total += amount;
    }

    pub fn handle_withdrawal(&mut self, amount: &Decimal) {
        assert!(self.can_withdraw(amount));
        self.available -= amount;
        self.total -= amount;
    }

    pub fn handle_dispute(&mut self, amount: &Decimal) {
        assert!(self.can_dispute(amount));
        self.available -= amount;
        self.held += amount;
    }

    pub fn handle_resolve(&mut self, amount: &Decimal) {
        assert!(self.can_resolve(amount));
        self.available += amount;
        self.held -= amount;
    }

    pub fn handle_chargeback(&mut self, amount: &Decimal) {
        assert!(self.can_chargeback(amount));
        self.total -= amount;
        self.held -= amount;
        self.locked = true;
    }
}

impl Ledger {
    // mutable
    fn __create_account(&mut self, client_id: &ClientID) {
        let account = Account::default();
        self.accounts.insert(*client_id, account);
    }

    pub fn handle_deposit(&mut self, client_id: &ClientID, tx_id: &TxID, amount: &Decimal) {
        self.transactions
            .insert(*tx_id, Transaction::from_deposit(*client_id, *amount));

        if let Some(account) = self.accounts.get_mut(client_id) {
            account.handle_deposit(amount);
        } else {
            let mut account = Account::default();
            account.handle_deposit(amount);
            self.accounts.insert(*client_id, account);
        }
    }

    pub fn handle_withdrawal(&mut self, client_id: &ClientID, tx_id: &TxID, amount: &Decimal) {
        if let Some(account) = self.accounts.get_mut(client_id) {
            if account.can_withdraw(amount) {
                self.transactions
                    .insert(*tx_id, Transaction::from_withdrawal(*client_id, *amount));
                account.handle_withdrawal(amount);
            }
        } else {
            self.__create_account(client_id);
        }
    }

    pub fn handle_dispute(&mut self, client_id: &ClientID, tx_id: &TxID) {
        if let Some(account) = self.accounts.get_mut(client_id) {
            if let Some(tx) = self.transactions.get_mut(tx_id) {
                if tx.category == TxCategory::Deposit
                    && tx.state == TxState::Normal
                    && account.can_dispute(&tx.amount)
                {
                    tx.state = TxState::Disputed;
                    account.handle_dispute(&tx.amount);
                }
            }
        } else {
            self.__create_account(client_id);
        }
    }

    pub fn handle_resolve(&mut self, client_id: &ClientID, tx_id: &TxID) {
        if let Some(account) = self.accounts.get_mut(client_id) {
            if let Some(tx) = self.transactions.get_mut(tx_id) {
                if tx.client_id == *client_id
                    && tx.state == TxState::Disputed
                    && account.can_resolve(&tx.amount)
                {
                    tx.state = TxState::Normal;
                    account.handle_resolve(&tx.amount);
                }
            }
        } else {
            self.__create_account(client_id);
        }
    }

    pub fn handle_chargeback(&mut self, client_id: &ClientID, tx_id: &TxID) {
        if let Some(account) = self.accounts.get_mut(client_id) {
            if let Some(tx) = self.transactions.get_mut(tx_id) {
                if tx.client_id == *client_id
                    && tx.state == TxState::Disputed
                    && account.can_chargeback(&tx.amount)
                {
                    tx.state = TxState::Chargedback;
                    account.handle_chargeback(&tx.amount);
                }
            }
        } else {
            self.__create_account(client_id);
        }
    }
}

#[cfg(test)]
mod handler_tests {
    use super::*;

    #[test]
    #[should_panic]
    pub fn test_invalid() {
        let mut account = Account::default();
        account.handle_withdrawal(&Decimal::from(1));
        account.handle_dispute(&Decimal::from(1));
        account.handle_resolve(&Decimal::from(1));
        account.handle_chargeback(&Decimal::from(1));
    }
    #[test]
    pub fn test_accounts() {
        let mut account = Account::default();
        account.handle_deposit(&Decimal::from(100));
        assert_eq!(account.available, Decimal::from(100));
        assert_eq!(account.total, Decimal::from(100));

        account.handle_withdrawal(&Decimal::from(99));
        assert_eq!(account.available, Decimal::from(1));
        assert_eq!(account.total, Decimal::from(1));

        account.handle_dispute(&Decimal::from(1));
        assert_eq!(account.available, Decimal::from(0));
        assert_eq!(account.held, Decimal::from(1));
        assert_eq!(account.total, Decimal::from(1));

        account.handle_resolve(&Decimal::from(1));
        assert_eq!(account.available, Decimal::from(1));
        assert_eq!(account.held, Decimal::from(0));
        assert_eq!(account.total, Decimal::from(1));

        account.handle_dispute(&Decimal::from(1));
        account.handle_chargeback(&Decimal::from(1));
        assert_eq!(account.available, Decimal::from(0));
        assert_eq!(account.held, Decimal::from(0));
        assert_eq!(account.total, Decimal::from(0));
    }
}
