//category defines in which type trnsactions belongs.
//personal: money spent on things that are personal items,
//mandatory: money spend on items that are mandatory,
//Income: money earned
enum Category {
    Personal,
    Mandatory,
    Scheduled,
    Income,
}

//Account is bucket to keep accounting of transcation
struct Account {
    id: AccountID,
    name: String,
    account_type: AccountType,
    balance: f64,
    transaction: Option<Vec<TransactionID>>,
}

//different types of account
enum AccountType {
    Saving,
    Emi,
    Active,
}

//transaction could be debit(-) or credit(+) in account
enum TransactionType {
    Debit,
    Credit,
}

//unique id type for transaction and account
type AccountID = String;
type TransactionID = String;

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

//self explaintory
struct Amount {
    principal: f64,
    final_amt: f64,
    taxes: Vec<f64>,
}
