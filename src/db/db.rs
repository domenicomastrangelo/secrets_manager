use rusqlite::Connection;

pub fn connect() -> Result<Connection, rusqlite::Error> {
    let connection = Connection::open_in_memory()?;

    create_table(&connection)?;

    Ok(connection)
}

fn create_table(connection: &Connection) -> Result<(), rusqlite::Error> {
    let stmt = connection.execute(
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
        .execute(&[&name, &secret])?;

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
