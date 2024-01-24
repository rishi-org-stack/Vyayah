// finctl acc add --name=<name> --type=saving|emi|active
//     --initial_balance=<initial_balance_amt>

// findctl acc del <name>

// findctl acc statement <name> --tail=<tail_count> --head=<head_count>

// findctl describe <acc>/<name>

// findctl txn --name=<name> debit  --amt=<amount> --tax=1,2,3 --type=
// <category of txn> --info=<description>

// findctl txn --name=<name> refund  <txn_identifier>
use clap::{arg, command, Arg, Command};

pub fn input() -> Command {
    Command::new("finctl")
        .subcommand(
            Command::new("acc")
                .about("account releated operations")
                .subcommand(
                    Command::new("add")
                        .about("add new account")
                        .arg(Arg::new("name").short('n').long("name"))
                        .arg(Arg::new("type").long("type"))
                        .arg(
                            Arg::new("initial_balance")
                                .long("initial_balance")
                                .short('b')
                                .required(false),
                        ),
                )
                .subcommand(
                    Command::new("del")
                        .about("delete a account")
                        .arg(Arg::new("name").short('n').long("name")),
                )
                .subcommand(
                    Command::new("statement")
                        .about("statement of a account")
                        .arg(Arg::new("name").short('n').long("name"))
                        .arg(Arg::new("tail").long("tail").required(false))
                        .arg(Arg::new("head").long("head").required(false)),
                ),
        )
        .subcommand(
            Command::new("describe")
                .about("describes the unit")
                .arg(arg!(<UNIT> "unit to describe"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("txn")
                .about("transaction operations")
                .subcommand(
                    Command::new("refund")
                        .arg(Arg::new("refund"))
                        .arg(Arg::new("name").short('n').long("name"))
                        .arg(Arg::new("id").long("id")),
                )
                .subcommand(
                    Command::new("new")
                        .arg(arg!(<TYPE> "type"))
                        .arg(Arg::new("name").short('n').long("name"))
                        .arg(Arg::new("amount").long("amt"))
                        .arg(Arg::new("category").long("kind"))
                        .arg(Arg::new("information").long("info"))
                        .arg(Arg::new("taxes").long("tax").num_args(1..)),
                ),
        )
}
