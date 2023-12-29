use std::process;

use clap::{Parser, ValueEnum, Subcommand};
use config::Account;
mod config;


#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    mode: Commands,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Subcommand)]
enum Commands {
    /// lists accounts
    List,
    /// adds a new account
    Add,
    /// set current account
    Set {
        account: usize
    }
}

fn main() {
    let config_path = config::get_config();
    let mut accounts = config::get_accounts(&config_path);

    let args = Args::parse();
    match args.mode {
        Commands::List => {
            // get current config
            let git_name = process::Command::new("git").args(["config", "user.name"]).output().expect("unable to interact with git CLI").stdout;
            let git_email = process::Command::new("git").args(["config", "user.email"]).output().expect("unable to interact with git CLI").stdout;
            let mut git_name = String::from_utf8(git_name).unwrap();
            let mut git_email = String::from_utf8(git_email).unwrap();
            //get rid of trailing \n
            git_name.pop();
            git_email.pop();
            let git_account = Account::new(git_name, git_email);
            let pos = accounts.iter().position(|a| a == &git_account);
            match pos {
                None => {
                    println!("Current: {}", git_account);
                    println!("---Available---");
                    for i in 0..accounts.len() {
                        println!("{}) {}", i, accounts[i]);
                    }
                },
                Some(n) => {
                    println!("---Available---");
                    for account in 0..accounts.len() {
                        if account == n {
                            println!("Current: {}", git_account);
                        } else {
                            println!("{}) {}", account, accounts[account]);
                        }
                    }
                }
            }
        },
        Commands::Add => {
            let mut email: String = String::new();
            let mut name: String = String::new();
            print!("Name: ");
            scanf::scanf!("{}", name).expect("unable to read name");
            print!("Email: ");
            scanf::scanf!("{}", email).expect("unable to read email");
            accounts.push(Account::new(name, email));
            config::update_acounts(accounts, &config_path);
        },
        Commands::Set { account } => {
            process::Command::new("git").args(["config", "--local", "user.name", &accounts[account].name]);
            process::Command::new("git").args(["config", "--local", "user.email", &accounts[account].email]);
        }
    }
}
