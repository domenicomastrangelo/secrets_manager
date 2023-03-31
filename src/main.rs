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
                Ok(_) => println!("\nSecret added"),
                Err(_) => println!("\nError adding secret"),
            };
        }
        Some(SubCommand::Remove) => {
            remove();
        }
        Some(SubCommand::Edit) => {
            edit();
        }
        Some(SubCommand::Show) => {
            show();
        }
        None => {
            println!("No command");
        }
    }
}
