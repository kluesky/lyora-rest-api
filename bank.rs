use std::collections::HashMap;
use crate::account::Account;

pub struct Bank {
    accounts: HashMap<u32, Account>,
    next_id: u32,
}

impl Bank {
    pub fn new() -> Self {
        Bank { accounts: HashMap::new(), next_id: 1 }
    }

    pub fn create_account(&mut self, owner: String, initial: f64) -> &Account {
        let acc = Account::new(self.next_id, owner, initial);
        self.accounts.insert(self.next_id, acc);
        self.next_id += 1;
        self.accounts.get(&(self.next_id - 1)).unwrap()
    }

    pub fn get_account(&self, id: u32) -> Option<&Account> {
        self.accounts.get(&id)
    }

    pub fn get_account_mut(&mut self, id: u32) -> Option<&mut Account> {
        self.accounts.get_mut(&id)
    }

    pub fn total_assets(&self) -> f64 {
        self.accounts.values().map(|a| a.balance).sum()
    }
}
