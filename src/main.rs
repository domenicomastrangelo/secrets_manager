mod flags;

use flags::flags::{SubCommand, Args, login, logout, list, add, remove, edit, show};
use clap::Parser;

fn main() {
    let args = Args::parse();

    match args.subcmd {
        Some(SubCommand::Login) => {
            login();
        },
        Some(SubCommand::Logout) => {
            logout();
        },
        Some(SubCommand::List) => {
            list();
        },
        Some(SubCommand::Add) => {
            match add() {
                Ok(_) => println!("\nSecret added"),
                Err(_) => println!("\nError adding secret")
            };
        },
        Some(SubCommand::Remove) => {
            remove();
        },
        Some(SubCommand::Edit) => {
            edit();
        },
        Some(SubCommand::Show) => {
            show();
        },
        None => {
            println!("No command");
        }
    }   
}
