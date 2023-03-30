use std::io;

use clap::{Parser, Subcommand};

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

pub fn add() -> Result<(), io::Error> {
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

    save_secret_in_database(secret);

    Ok(())
}

#[allow(unused_variables)]
fn save_secret_in_database(secret: String) {

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
