use std::io;

use crate::db::db::{add_secret, list_secrets, read_secret, remove_secret};

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

pub fn list(connection: &Connection) -> Result<(), io::Error> {
    match list_secrets(&connection) {
        Ok(list) => {
            for secret in list {
                println!("{}", secret.trim());
            }
        }
        Err(_) => {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Error listing secrets",
            ))
        }
    };

    Ok(())
}

pub fn add(connection: &Connection, name: &Option<String>) -> Result<(), io::Error> {
    let name = match name {
        Some(name) => name.clone(),
        None => {
            print!("Name: ");
            io::Write::flush(&mut io::stdout())?;

            let mut name = String::new();
            io::Write::flush(&mut io::stdout())?;

            io::stdin().read_line(&mut name)?;

            name
        }
    };

    print!("Secret: ");
    io::Write::flush(&mut io::stdout())?;

    // Read the secret from stdin

    let mut secret = String::new();
    io::stdin().read_line(&mut secret)?;

    match add_secret(
        &connection,
        String::from(name.trim()),
        String::from(secret.trim()),
    ) {
        Ok(_) => (),
        Err(_) => return Err(io::Error::new(io::ErrorKind::Other, "Error adding secret")),
    };

    let s = read_secret(&connection, String::from(name.trim()));
    let s = match s {
        Ok(s) => s,
        Err(_) => String::from(""),
    };

    println!("{}", s);

    Ok(())
}

pub fn remove(connection: &Connection, name: &Option<String>) -> Result<(), io::Error> {
    let name = match name {
        Some(name) => name.clone(),
        None => {
            print!("Name: ");
            io::Write::flush(&mut io::stdout())?;

            let mut name = String::new();
            io::stdin().read_line(&mut name)?;

            name
        }
    };

    match remove_secret(&connection, String::from(name.trim())) {
        Ok(_) => (),
        Err(_) => {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Error removing secret",
            ))
        }
    };

    Ok(())
}

pub fn edit(connection: &Connection) -> Result<(), io::Error> {
    print!("Name: ");
    io::Write::flush(&mut io::stdout())?;

    let mut name = String::new();
    io::stdin().read_line(&mut name)?;

    remove(&connection, &Some(String::from(name.trim())))?;
    add(&connection, &Some(String::from(name.trim())))?;

    Ok(())
}

pub fn show(connection: &Connection) -> Result<String, io::Error> {
    print!("Name: ");
    io::Write::flush(&mut io::stdout())?;

    let mut name = String::new();
    io::stdin().read_line(&mut name)?;

    let secret = match read_secret(&connection, String::from(name.trim())) {
        Ok(secret) => secret,
        Err(_) => return Err(io::Error::new(io::ErrorKind::Other, "Error reading secret")),
    };

    Ok(secret)
}
