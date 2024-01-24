pub mod cli;
pub mod id;
pub mod model;
use clap::{Arg, ArgAction, ArgMatches, Command};
use cli::input;
fn handle_acc(subcmd: &ArgMatches) {
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
            )
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
            )
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

    match input.subcommand() {
        Some(("acc", subcommand)) => handle_acc(subcommand),
        Some(("txn", commands)) => handle_txn(commands),
        Some(("describe", command)) => handle_describe(command),
        _ => unreachable!(),
    }
}
