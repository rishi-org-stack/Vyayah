use sled::Db;

use crate::{
    model::{Account, Transaction}, protocol::{request::{EntityTypes, Function}, response::Response}, service::ServiceTrait
};

struct AccountingService {
    acc_db: Db,
    txn_db: Db,
    snap_db: Db,
}
impl AccountingService {
    pub fn new(acc_db: Db, txn_db: Db, snap_db: Db) -> AccountingService {
        AccountingService {
            acc_db,
            txn_db,
            snap_db,
        }
    }
}

impl ServiceTrait for AccountingService {
    fn create(&self, function: Function) -> Response {
        match function.entity {
            EntityTypes::ACC=> {
                self.get_msg();
                self.create_account(account: Account);
                
            },
            EntityTypes::TXN=>{
                 self.get_msg();
                self.create_transaction(account: Transaction);
                
            }
        }
    }
    
    fn delete(&self, function: Function) -> Response {
        match function.entity {
            EntityTypes::ACC=> {
                self.get_msg();
                self.delete_account(account: Account);
                
            },
           _=>{
             todo!() 
                
            }
        }
    }

    fn describe(&self, function: Function) -> Response {
        match function.entity {
            EntityTypes::ACC=> {
                self.get_msg();
                self.describe_account(account: Account);
                
            },
           EntityTypes::TXN=>{
                self.get_msg();
                self.describe_transaction(txn: Transaction);
                
            }
        }
    }


    // fn delete(&self, function: Function) -> Response {
    //     match function.entity {
    //         EntityTypes::ACC=> {
    //             self.get_msg();
    //             self.delete_account(account: Account);
                
    //         },
    //        _=>{
    //          todo!() 
                
    //         }
    //     }
    // }

}
