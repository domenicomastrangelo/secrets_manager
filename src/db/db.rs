use rusqlite::Connection;

pub fn connect() -> Result<Connection, rusqlite::Error> {
    let connection = Connection::open("db.sqlite")?;

    create_table(&connection)?;

    Ok(connection)
}

fn create_table(connection: &Connection) -> Result<(), rusqlite::Error> {
    connection.execute(
        "CREATE TABLE IF NOT EXISTS secrets (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            secret TEXT NOT NULL
        )",
        [],
    )?;

    Ok(())
}

pub fn add_secret(
    connection: &Connection,
    name: String,
    secret: String,
) -> Result<(), rusqlite::Error> {
    connection
        .prepare("INSERT INTO secrets (name, secret) VALUES (?, ?)")?
        .execute(&[&name.trim(), &secret.trim()])?;

    Ok(())
}

pub fn read_secret(connection: &Connection, name: String) -> Result<String, rusqlite::Error> {
    let mut stmt = connection.prepare("SELECT secret FROM secrets WHERE name = ?")?;
    let mut stmt = stmt.query(&[&name])?;

    let Some(stmt) = stmt.next()? else {
        return Err(rusqlite::Error::QueryReturnedNoRows);
    };
    let secret: String = stmt.get(0)?;

    Ok(secret)
}

pub fn remove_secret(connection: &Connection, name: String) -> Result<(), rusqlite::Error> {
    match connection
        .prepare("DELETE FROM secrets WHERE name = ?")?
        .execute(&[&name])?
    {
        0 => Err(rusqlite::Error::QueryReturnedNoRows),
        _ => Ok(()),
    }
}

pub fn list_secrets(connection: &Connection) -> Result<Vec<String>, rusqlite::Error> {
    let mut stmt = connection.prepare("SELECT name FROM secrets")?;
    let mut stmt = stmt.query([])?;

    let mut list = Vec::new();

    while let Some(stmt) = stmt.next()? {
        let name: String = stmt.get(0)?;
        list.push(name);
    }

    Ok(list)
}
