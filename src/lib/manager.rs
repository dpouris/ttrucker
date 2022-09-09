use crate::db;
use std::collections::HashMap;

pub struct Manager {
    db: db::DB,
}

#[allow(unused_must_use)]
impl Manager {
    pub fn create_expenses_table(&mut self) {
        let mut columns = HashMap::new();

        columns.insert("id", "INTEGER PRIMARY KEY".to_owned());
        columns.insert("name", "TEXT".to_owned());
        columns.insert("amount", "INTEGER".to_owned());

        self.db.create_table("expenses", columns);
    }

    pub fn add_expense(&mut self, name: &str, amount: i32) {
        let columns = vec!["name", "amount"];
        let values = vec![format!("'{name}'"), format!("{amount}")];
        self.db.insert_into("expenses", columns, values);
    }

    pub fn view_expenses(&mut self) -> Vec<sqlite::Row> {
        self.db.select_from("expenses")
    }

    pub fn remove_expense(&mut self, expense_id: &str) {
        self.db
            .delete_from("expenses", format!("id = {expense_id}").as_str());
    }

    pub fn reconnect(&mut self, path: &str) {
        let res = db::new_db_conn(&path);

        if let Err(msg) = res {
            println!("{msg}");
            return;
        }

        self.db = res.unwrap();
    }
}

pub fn init_manager(path: &str) -> Result<Manager, sqlite::Error> {
    let db = db::new_db_conn(&path)?;

    Ok(Manager { db })
}
