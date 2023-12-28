use std::process;

use clap::{Parser, ValueEnum};
use config::Account;
mod config;


#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(value_enum)]
    mode: Commands,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Commands {
    /// lists accounts
    List ,
    /// adds a new account
    Add 
}

fn main() {
    let config_path = config::get_config();
    let accounts = config::get_accounts(&config_path);

    let args = Args::parse();
    match args.mode {
        Commands::List => {
            // get current config
            let git_name = process::Command::new("git").args(["config", "user.name"]).output().expect("unable to interact with git CLI").stdout;
            let git_email = process::Command::new("git").args(["config", "user.email"]).output().expect("unable to interact with git CLI").stdout;
            let git_name = String::from_utf8(git_name).unwrap();
            let git_email = String::from_utf8(git_email).unwrap();
            let git_account = Account::new(git_name, git_email);
            let pos = accounts.iter().position(|a| a == &git_account);
            match pos {
                None => {
                    println!("Current: {}", git_account);
                    println!("---Available---");
                    for account in accounts {
                        println!("{}", account);
                    }
                },
                Some(n) => {
                    println!("---Available---");
                    for i in 0..accounts.len() {
                        if i == n {
                            println!("Current: {}", git_account);
                        } else {
                            println!("{}", account);
                        }
                    }
                }
            }
        },
        Commands::Add => {

            config::update_acounts(accounts, &config_path);
        }
    }
}
