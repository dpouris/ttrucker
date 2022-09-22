use std::collections::HashMap;

use sqlite::{self, Connection};

pub struct DB {
    pub connection: Connection,
}

impl DB {
    fn init(path: &str) -> Result<Connection, sqlite::Error> {
        let conn = conn(path)?;
        Ok(conn)
    }

    pub fn create_table(
        &mut self,
        table_name: &str,
        columns: HashMap<&str, String>,
    ) -> sqlite::Result<()> {
        let mut statement = format!("CREATE TABLE {table_name}(");
        for (column_name, options) in columns.iter() {
            statement.push_str(format!("{column_name} {options}, ").as_str());
        }
        statement.pop();
        statement.pop();
        statement.push_str(");");

        let res = self.connection.execute(statement)?;

        Ok(res)
    }

    pub fn select_from(&mut self, table_name: &str) -> Vec<sqlite::Row> {
        let statement = format!("SELECT * FROM {table_name};");
        let cursor = self.connection.prepare(statement).unwrap().into_cursor();

        let mut rows = vec![];

        for row in cursor {
            _ = row.and_then(|v| Ok(rows.push(v)));
        }

        rows
    }

    pub fn select_from_limit(
        &mut self,
        table_name: &str,
        id: &str,
        limit: i32,
    ) -> sqlite::Result<sqlite::Row> {
        let statement = format!(
            "SELECT * FROM {} WHERE id = {} LIMIT {}",
            table_name, id, limit
        );
        let cursor = self.connection.prepare(statement).unwrap().into_cursor();

        cursor.last().unwrap()
    }

    pub fn insert_into(
        &mut self,
        table_name: &str,
        columns: Vec<&str>,
        values: Vec<String>,
    ) -> sqlite::Result<()> {
        let mut statement = format!("INSERT INTO {table_name} (");

        let mut columns_string = columns.join(", ");
        columns_string.push_str(")\nVALUES (");
        statement.push_str(&columns_string);

        let mut values_string = values.join(", ");
        values_string.push_str(");");
        statement.push_str(&values_string);

        self.connection.execute(statement)?;

        Ok(())
    }

    pub fn update_where(
        &mut self,
        table_name: &str,
        id: &str,
        to_set: &[&str; 2],
    ) -> sqlite::Result<()> {
        let statement = format!(
            "UPDATE {} SET {} = {} WHERE id = {}",
            table_name, to_set[0], to_set[1], id
        );

        self.connection.execute(statement)?;

        Ok(())
    }

    pub fn delete_from(&mut self, table_name: &str, filter: &str) {
        let statement = format!("DELETE FROM {table_name} WHERE {filter}");

        let res = self.connection.execute(statement);

        if let Err(msg) = res {
            eprintln!("ERROR: {}", msg)
        }
    }
}

pub fn new_db_conn(path: &str) -> Result<DB, sqlite::Error> {
    let conn = DB::init(path)?;
    Ok(DB { connection: conn })
}

fn conn(path: &str) -> sqlite::Result<sqlite::Connection> {
    let connect = sqlite::open(path)?;
    Ok(connect)
}
