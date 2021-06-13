use crate::workspace::Workspace;
use dirs::home_dir;
use rusqlite::{params, Connection, Error, Result};

pub fn initialize_db() -> Result<()> {
    let conn = connect_db()?;
    let mut table_exists = false;
    let res = conn.query_row(
        "SELECT name FROM sqlite_master WHERE type='table' AND name=?1",
        params!["workspaces"],
        |_| {
            return Ok(());
        },
    );

    match res {
        Ok(r) => {
            table_exists = true;
            println!("{:?}", r);
        }
        Err(e) => {
            table_exists = false;
            println!("{}", e);
        }
    }

    println!("Table exists {}", table_exists);

    if table_exists {
        return Ok(());
    }

    conn.execute(
        "
    CREATE TABLE workspaces (
        id      INTEGER PRIMARY KEY AUTOINCREMENT,
        name    TEXT UNIQUE NOT NULL
    )
    ",
        params![],
    )?;

    conn.execute(
        "
    CREATE TABLE dirs (
        id              INTEGER PRIMARY KEY AUTOINCREMENT,
        workspaceId     INTEGER,
        path            TEXT NOT NULL,
        
        FOREIGN KEY(workspaceId) REFERENCES workspaces(id)
    );
    ",
        params![],
    )?;

    Ok(())
}

fn connect_db() -> Result<Connection> {
    let path = home_dir().unwrap();
    let path = path.to_str().unwrap();
    let path = format!("{}{}", path, "/workspaces.db");
    let conn = Connection::open(path)?;

    Ok(conn)
}

pub fn insert_new_workspace(workspace: Workspace) -> Result<usize> {
    let conn = connect_db()?;

    conn.execute(
        "
        INSERT INTO workspaces(name) VALUES (
            ?1
        )
    ",
        params![workspace.name],
    )?;

    let mut inserted_id = 0;
    conn.query_row("SELECT last_insert_rowid()", params![], |row| {
        let id: usize = row.get(0)?;
        println!("INSERT WORKSPACE ID {}", id);
        inserted_id = 0;
        return Ok(());
    })?;

    Ok(inserted_id)
}

pub fn get_dirs_for_workspace(workspace_id: i32) -> Result<Vec<(i32, String)>> {
    let conn = connect_db()?;

    let mut stmt = conn.prepare("SELECT d.id, d.path from dirs d where workspaceId = ?1")?;

    let paths = stmt.query_map([workspace_id], |row| {
        let path: String = row.get(1).unwrap();
        let id: i32 = row.get(0).unwrap();
        return Ok((id, path));
    })?;

    let paths: Vec<(i32, String)> = paths.map(|x| x.unwrap()).collect();

    Ok(paths)
}

pub fn remove_dir_from_workspace(dir_id: i32) -> Result<()> {
    let conn = connect_db()?;

    let rows = conn.execute("DELETE from dirs where id = ?1", params![dir_id])?;

    if rows == 0 {
        return Err(rusqlite::Error::InvalidQuery);
    }

    Ok(())
}

pub fn insert_new_dir_for_workspace(workspace_id: i32, path: String) -> Result<usize, Error> {
    let conn = connect_db()?;

    conn.execute(
        "
        INSERT INTO dirs(workspaceId, path) VALUES(
            ?1, ?2
        );
    ",
        params![workspace_id, path],
    )?;

    Ok(get_last_insert_id(conn)?)
}

fn get_last_insert_id(conn: Connection) -> Result<usize> {
    let mut inserted_id = 0;
    conn.query_row("SELECT last_insert_rowid()", params![], |row| {
        let id: usize = row.get(0)?;
        println!("Last Inserted ID {}", id);
        inserted_id = id;
        return Ok(());
    })?;

    Ok(inserted_id)
}

pub fn fetch_all_workspaces() -> Result<Vec<(i32, String)>, Error> {
    let conn = connect_db().unwrap();

    let mut stmt = conn.prepare("SELECT w.name, w.id from workspaces w;")?;

    let values = stmt.query_map([], |x| {
        let name: String = x.get(0).unwrap();
        let id: i32 = x.get(1).unwrap();

        println!("{} {}", name, id);

        return Ok((id, name));
    })?;

    let values = values.map(|x| x.unwrap()).collect();

    return Ok(values);
}

#[allow(dead_code)]
pub fn fetch_all_workspaces_with_dirs() {
    let conn = connect_db().unwrap();

    let mut stmt = conn
        .prepare(
            "SELECT w.name, w.id, d.path from workspaces w
        LEFT JOIN dirs d
        ON d.workspaceId == w.id
        ",
        )
        .unwrap();

    let values = stmt
        .query_map([], |x| {
            let val: String = x.get(0).unwrap();
            let id: i32 = x.get(1).unwrap();
            let path: String = x.get(2).unwrap_or(String::from("None"));

            println!("{} {} {}", val, id, path);

            return Ok(val);
        })
        .unwrap();

    let x = values.count();

    println!("Count {}", x);
}

#[cfg(test)]
mod tests {
    use crate::db::*;

    fn connect_test_db() -> Result<Connection> {
        let path = format!("./test.db");
        let conn = Connection::open(path)?;

        conn.execute(
            "
        CREATE TABLE workspaces (
            id      INTEGER PRIMARY KEY AUTOINCREMENT,
            name    TEXT UNIQUE NOT NULL
        )
        ",
            params![],
        )?;

        conn.execute(
            "
        CREATE TABLE dirs (
            id              INTEGER PRIMARY KEY AUTOINCREMENT,
            workspaceId     INTEGER,
            path            TEXT NOT NULL,
            
            FOREIGN KEY(workspaceId) REFERENCES workspaces(id)
        );
        ",
            params![],
        )?;

        Ok(conn)
    }

    #[test]
    fn insert_a_workspace() -> Result<()> {
        let conn = connect_test_db()?;
        let w = Workspace::new(String::from("test"));
        conn.execute(
            "
            INSERT INTO workspaces(name) VALUES (
                ?1
            )
        ",
            params![w.name],
        )?;
        let mut inserted_id = 0;
        conn.query_row("SELECT last_insert_rowid()", params![], |row| {
            let id: usize = row.get(0)?;
            println!("INSERT WORKSPACE ID {}", id);
            inserted_id = id;
            return Ok(());
        })?;
        assert_eq!(inserted_id, 1);
        Ok(())
    }
}
