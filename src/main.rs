pub mod cli;
pub mod db;
pub mod handler;
pub mod id;
pub mod model;
use clap::ArgMatches;
use cli::input;
use sled::{Config, Db};
use std::io::Read;

use crate::{db::AccountDB, model::Account};
fn handle_acc(subcmd: &ArgMatches, db: AccountDB) {
    match subcmd.subcommand() {
        Some(("add", commands)) => {
            let name = commands
                .get_one::<String>("name")
                .expect("name is required");

            let acc_type = commands
                .get_one::<String>("type")
                .expect("type is required");

            let default_balance = "0".to_string();
            let initial_balance = commands
                .get_one::<String>("initial_balance")
                .unwrap_or(&default_balance)
                .parse::<u64>()
                .expect("positive value only");

            println!(
                "add account requested with name={} type={} initial balance={}",
                name, acc_type, initial_balance
            );

            let account = Account::new(name.clone(), acc_type.clone(), Some(initial_balance))
                .expect("failed to create account");

            db.insert(&account.id, &account)
                .expect("failed to create account");

            println!("account_id: {}", account.id)
        }
        Some(("del", commands)) => {
            let name = commands
                .get_one::<String>("name")
                .expect("name is required");

            println!("delete account requested with name={}", name)
        }

        Some(("statement", commands)) => {
            let name = commands
                .get_one::<String>("name")
                .expect("name is required");

            let default_tail_len = "2".to_string();

            let tail_len = commands
                .get_one::<String>("tail")
                .unwrap_or(&default_tail_len)
                .parse::<u64>()
                .expect("number of transaction to from tail");

            let default_head_len = "2".to_string();

            let head_len = commands
                .get_one::<String>("head")
                .unwrap_or(&default_head_len)
                .parse::<u64>()
                .expect("number of transaction to from tail");

            println!(
                "satement of account with name={}, tail={}, head={}",
                name, tail_len, head_len
            );

            let acc = db.get("F78F".to_string()).expect("failed get");
            println!("account: {:#?}", acc)
        }
        _ => unreachable!(),
    }
}

fn handle_txn(subcmds: &ArgMatches) {
    match subcmds.subcommand() {
        Some(("refund", cmd)) => {
            let name = cmd.get_one::<String>("name").expect("name is required");
            let id = cmd.get_one::<String>("id").expect("tx. id is required");
            println!("{:#?}", name);
            println!("{:#?}", id);
        }
        Some(("new", cmd)) => {
            let txn_type = cmd.get_one::<String>("TYPE").unwrap();

            let name = cmd.get_one::<String>("name").expect("name is required");

            let def_amount = "2".to_string();

            let amount = cmd
                .get_one::<String>("amount")
                .unwrap_or(&def_amount)
                .parse::<u64>()
                .expect("value of txn");

            let category = cmd
                .get_one::<String>("category")
                .expect("category is required to group transaction");

            let default_info = "".to_string();
            let information = cmd
                .get_one::<String>("information")
                .unwrap_or(&default_info);

            let mut taxes_nums: Vec<u64> = Vec::new();

            if let Some(taxes_str) = cmd.get_many::<String>("taxes") {
                taxes_nums = taxes_str
                    .map(|s| s.parse::<u64>().expect("only num"))
                    .collect();
            }

            println!("{:#?}", txn_type);
            println!("{:#?}", name);
            println!("{:#?}", amount);
            println!("{:#?}", category);
            println!("{:#?}", information);
            println!("{:#?}", taxes_nums)
        }
        _ => unreachable!(),
    }
}

fn handle_describe(cmd: &ArgMatches) {
    let unit = cmd.get_one::<String>("UNIT").unwrap();
    println!("describe unit={}", unit)
}

fn main() {
    let input = input().get_matches();

    let db = Config::new()
        .path("./ok")
        .open()
        .expect("failed to create db");

    // let acc_db = AccountDB::new(db);
    // match input.subcommand() {
    //     Some(("acc", subcommand)) => handle_acc(subcommand, acc_db),
    //     Some(("txn", commands)) => handle_txn(commands),
    //     Some(("describe", command)) => handle_describe(command),
    //     _ => unreachable!(),
    // }

    db.insert("key", "v").expect("msg");
    let v = db.get("key").unwrap().unwrap();

    let b = v.bytes().map(|bt| bt.unwrap()).collect();
    let account_body = String::from_utf8(b).unwrap();
    println!("{}", account_body)
}
