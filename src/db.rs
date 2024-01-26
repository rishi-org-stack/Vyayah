use std::io::Read;

use sled::Db;

use crate::model::Account;

pub struct AccountDB {
    db: Db,
}

impl AccountDB {
    pub fn new(db: Db) -> AccountDB {
        AccountDB { db }
    }

    pub fn insert(&self, account_id: &String, account: &Account) -> Result<(), String> {
        let s = account.encode().expect("failed insert");
        println!("s: {}", s);
        match self.db.insert(account_id, s.into_bytes().to_vec()) {
            Ok(_) => Ok(()),
            Err(e) => return Err(e.to_string()),
        }
    }
    pub fn get(&self, account_id: String) -> Result<Account, String> {
        let acc: Account = match self.db.get(account_id).unwrap() {
            Some(v) => {
                let b = v.bytes().map(|bt| bt.unwrap()).collect();
                let account_body = String::from_utf8(b).unwrap();
                Account::decode(account_body).unwrap()
            }
            None => return Err("no val found".to_string()),
        };

        Ok(acc)
    }

    pub fn delete(&self, account_id: String) -> Result<(), String> {
        match self.db.remove(account_id) {
            Err(e) => return Err(e.to_string()),
            _ => Ok(()),
        }
    }
}
