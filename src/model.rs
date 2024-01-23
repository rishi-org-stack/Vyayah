use serde::{Deserialize, Serialize};
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

#[derive(Serialize, Deserialize, Debug)]
//Account is bucket to keep accounting of transcation
struct Account {
    id: AccountID,
    name: String,
    account_type: AccountType,
    balance: f64,
    transaction: Option<Vec<TransactionID>>,
}

#[derive(Serialize, Deserialize, Debug)]
//different types of account
enum AccountType {
    Saving,
    Emi,
    Active,
}

#[derive(Serialize, Deserialize, Debug)]
//transaction could be debit(-) or credit(+) in account
enum TransactionType {
    Debit,
    Credit,
}

#[derive(Serialize, Deserialize, Debug)]
//self explaintory
struct Amount {
    principal: f64,
    final_amt: f64,
    taxes: Vec<f64>,
}

//unique id type for transaction and account
type AccountID = String;
type TransactionID = String;

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
    // refrence other transaction in case of refunds, refusals etc.
    refrence: Option<TransactionID>,
}
