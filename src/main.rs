// use std::{fs::File, error::Error};
// pub mod expense;
// struct Connection<'a> {
//     data: Vec<&'a str>,
// }

// #[derive(Debug)]
// struct ErrFileNotFound{
//     msg: String,
// }
// impl Error for ErrFileNotFound{

// }

// impl ErrFileNotFound{
//     fn Display(){

//     }
// }
// struct ErrorConnection{

// }

// impl<'a> Connection<'a> {
//     fn new(file_path: &str) -> Connection<'a> {
//         if let File::open(file_path);
//     }
// }

// fn main() {

// }
pub mod cli;
use clap::{Arg, ArgAction, Command};
use cli::input;
use serde::de;

fn main() {
    // let matches = Command::new("pacman")
    //     .about("package manager utility")
    //     .version("5.2.1")
    //     .subcommand_required(true)
    //     .arg_required_else_help(true)
    //     // Query subcommand
    //     //
    //     // Only a few of its arguments are implemented below.
    //     .subcommand(
    //         Command::new("query")
    //             .short_flag('Q')
    //             .long_flag("query")
    //             .about("Query the package database.")
    //             .arg(
    //                 Arg::new("search")
    //                     .short('s')
    //                     .long("search")
    //                     .help("search locally installed packages for matching strings")
    //                     .conflicts_with("info")
    //                     .action(ArgAction::Set)
    //                     .num_args(1),
    //             )
    //             .arg(
    //                 Arg::new("info")
    //                     .long("info")
    //                     .short('i')
    //                     .conflicts_with("search")
    //                     .help("view package information")
    //                     .action(ArgAction::Set)
    //                     .num_args(1),
    //             ),
    //     )
    //     // Sync subcommand
    //     //
    //     // Only a few of its arguments are implemented below.
    //     .get_matches();

    // match matches.subcommand() {
    //     Some(("query", query_matches)) => {
    //         if let Some(packages) = query_matches.get_many::<String>("info") {
    //             let comma_sep = packages.map(|s| s.as_str()).collect::<Vec<_>>().join(", ");
    //             println!("Retrieving info for {comma_sep}...");
    //         } else if let Some(queries) = query_matches.get_many::<String>("search") {
    //             let comma_sep = queries.map(|s| s.as_str()).collect::<Vec<_>>().join(", ");
    //             println!("Searching Locally for {comma_sep}...");
    //         } else {
    //             println!("Displaying all locally installed packages...");
    //         }
    //     }
    //     _ => unreachable!(), // If all subcommands are defined above, anything else is unreachable
    // }

    let input = input().get_matches();

    match input.subcommand() {
        Some(("acc", subcommand)) => match subcommand.subcommand() {
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
        },
        _ => unreachable!(),
    }
}
