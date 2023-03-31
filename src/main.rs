mod db;
mod flags;

use clap::Parser;
use db::db::connect;
use flags::flags::{add, edit, list, login, logout, remove, show, Args, SubCommand};

fn main() {
    let args = Args::parse();

    let connection = match connect() {
        Ok(connection) => {
            println!("Connected to database");
            connection
        }
        Err(_) => {
            println!("Error connecting to database");
            std::process::exit(1);
        }
    };

    match args.subcmd {
        Some(SubCommand::Login) => {
            login();
        }
        Some(SubCommand::Logout) => {
            logout();
        }
        Some(SubCommand::List) => {
            list();
        }
        Some(SubCommand::Add) => {
            match add(connection) {
                Ok(_) => println!("Secret added"),
                Err(_) => println!("Error adding secret"),
            };
        }
        Some(SubCommand::Remove) => {
            match remove(connection) {
                Ok(_) => println!("Secret removed"),
                Err(_) => println!("Error removing secret"),
            };
        }
        Some(SubCommand::Edit) => {
            edit();
        }
        Some(SubCommand::Show) => {
            match show(connection) {
                Ok(secret) => println!("{}", secret),
                Err(_) => println!("Error reading secret"),
            };
        }
        None => {
            println!("No subcommand was used");
        }
    }
}
