use std::time::SystemTime;

use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

fn new_trx_id() -> String {
    generate_id(10 as usize)
}

fn new_acc_id() -> String {
    generate_id(4 as usize)
}

fn generate_id(pass_len: usize) -> String {
    let seed = "1234567890abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".as_bytes();
    let seed_size: usize = 62;

    let mut pass: Vec<u8> = vec![0; pass_len];

    let mut rng = thread_rng();
    for _ in 0..pass_len {
        let rnd_index: usize = rng.gen_range(0..seed_size);
        pass.push(seed[rnd_index])
    }

    std::str::from_utf8(&pass).unwrap().to_string()
}

#[derive(Serialize, Deserialize, Debug)]
//different types of account
enum AccountType {
    Saving,
    Emi,
    Active,
}

impl AccountType {
    fn from_str(str: &str) -> Result<AccountType, String> {
        match str {
            "saving" => Ok(AccountType::Saving),
            "emi" => Ok(AccountType::Emi),
            "active" => Ok(AccountType::Active),
            _ => return Err(String::from("not a valid account type")),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
//Account is bucket to keep accounting of transcation
struct Account {
    id: AccountID,
    name: String,
    account_type: AccountType,
    balance: f64,
    created_on: u64,
    transactions: Option<Vec<TransactionID>>,
    snapshots: Option<Vec<SnapshotID>>,
}

impl Account {
    pub fn new(
        name: String,
        account_type: String,
        initial_balance: Option<f64>,
    ) -> Result<Account, String> {
        let acc_type = match AccountType::from_str(account_type.as_str()) {
            Ok(acc_t) => acc_t,
            Err(e) => return Err(e),
        };

        let init_balance = initial_balance.unwrap_or(0 as f64);
        let created_on = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Ok(Account {
            id: new_acc_id(),
            name,
            account_type: acc_type,
            balance: init_balance,
            created_on: created_on,
            transactions: None,
            snapshots: None,
        })
    }
}

//category defines in which type trnsactions belongs.
//personal: money spent on things that are personal items,
//mandatory: money spend on items that are mandatory,
//Income: money earned
#[derive(Serialize, Deserialize, Debug)]
enum Category {
    Personal,
    Mandatory,
    Scheduled,
    Income,
}

impl Category {
    fn from_str(str: &str) -> Result<Category, String> {
        match str {
            "personal" => Ok(Category::Personal),
            "mandatory" => Ok(Category::Mandatory),
            "schedule" => Ok(Category::Scheduled),
            "income" => Ok(Category::Income),
            _ => return Err(String::from("not a valid category type")),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
//transaction could be debit(-) or credit(+) in account
enum TransactionType {
    Debit,
    Credit,
}

impl TransactionType {
    fn from_str(str: &str) -> Result<TransactionType, String> {
        match str {
            "debit" => Ok(TransactionType::Debit),
            "credit" => Ok(TransactionType::Credit),
            _ => Err(String::from("invalid type")),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
//self explaintory
struct Amount {
    principal: f64,
    final_amt: f64,
    taxes: Vec<f64>,
}

impl Amount {
    fn from_str(principal: f64, taxes: Vec<f64>) -> Amount {
        let total_tax = taxes.iter().fold(0 as f64, |acc, v| acc + v);
        Amount {
            principal,
            final_amt: principal - total_tax,
            taxes,
        }
    }
}

//unique id type for transaction and account
type AccountID = String;
type TransactionID = String;
type SnapshotID = String;

#[derive(Serialize, Deserialize, Debug)]
//smallest unit of management what spent or earned from where
struct Transaction {
    id: TransactionID,
    source: AccountID,
    //credit or debit
    transaction_type: TransactionType,
    amount: Amount,
    //prime category of transaction
    category: Category,
    //sub category or grouping method
    tag: String,
    description: String,
    created_on: u64,
    // refrence other transaction in case of refunds, refusals etc.
    refrence: Option<TransactionID>,
}

impl Transaction {
    fn new(
        source: AccountID,
        tansaction_type_str: &str,
        category_str: &str,
        tag: String,
        description: String,
        principal: f64,
        taxes: Vec<f64>,
    ) -> Result<Transaction, String> {
        let id = new_trx_id();
        let category = match Category::from_str(category_str) {
            Ok(c) => c,
            Err(e) => return Err(e),
        };

        let txn_type = match TransactionType::from_str(tansaction_type_str) {
            Ok(txn_t) => txn_t,
            Err(e) => return Err(e),
        };

        let created_on = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Ok(Transaction {
            id,
            source,
            amount: Amount::from_str(principal, taxes),
            category,
            description,
            created_on,
            transaction_type: txn_type,
            refrence: None,
            tag,
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
//Account is bucket to keep accounting of transcation
struct AccountSnapshot {
    id: SnapshotID,
    balance: f64,
    transaction_id: TransactionID,
}

impl AccountSnapshot {
    fn new(balance: f64, transaction_id: String) -> AccountSnapshot {
        let id = new_trx_id();
        AccountSnapshot {
            id,
            balance,
            transaction_id,
        }
    }
}
