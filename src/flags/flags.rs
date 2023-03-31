use std::io;

use crate::db::db::{add_secret, read_secret};

use clap::{Parser, Subcommand};
use rusqlite::Connection;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    #[command(subcommand)]
    pub subcmd: Option<SubCommand>,
}

#[derive(Subcommand, Debug)]
pub enum SubCommand {
    #[clap(name = "login")]
    Login,
    #[clap(name = "logout")]
    Logout,
    #[clap(name = "list")]
    List,
    #[clap(name = "add")]
    Add,
    #[clap(name = "remove")]
    Remove,
    #[clap(name = "edit")]
    Edit,
    #[clap(name = "show")]
    Show,
}

pub fn login() {
    println!("Login");
}

pub fn logout() {
    println!("Logout");
}

pub fn list() {
    println!("List");
}

pub fn add(connection: Connection) -> Result<(), io::Error> {
    print!("Name: ");
    io::Write::flush(&mut io::stdout())?;

    let mut name = String::new();
    io::Write::flush(&mut io::stdout())?;

    io::stdin().read_line(&mut name)?;

    print!("Secret: ");
    io::Write::flush(&mut io::stdout())?;

    // Read the secret from stdin

    let mut secret = String::new();
    io::stdin().read_line(&mut secret)?;

    add_secret(&connection, name.clone(), secret);

    let s = read_secret(&connection, name);
    let s = match s {
        Ok(s) => s,
        Err(_) => String::from(""),
    };

    println!("{}", s);

    Ok(())
}

pub fn remove() {
    println!("Remove");
}

pub fn edit() {
    println!("Edit");
}

pub fn show() {
    println!("Show");
}
