#![allow(unused)]

use std::{collections::HashMap, thread::sleep, time::Duration};

use crate::{
    either, manager, printil, select,
    utils::{clear_term, get_input, hide_cursor, show_cursor},
};
use console::{Key, Term};

pub struct Menu {
    pub selected: i32,
    pub options: Vec<String>,
    term: Term,
}

impl Menu {
    fn show_menu(menu: Vec<String>) {
        println!("{}\r", menu.join("\n"));
    }

    pub fn new() -> Self {
        let menu = Self {
            options: vec![
                "Add expense".to_owned(),
                "View expenses".to_owned(),
                "Edit expense".to_owned(),
                "Remove expense".to_owned(),
                "Quit".to_owned(),
            ],
            selected: 1,
            term: Term::stdout(),
        };

        menu
    }

    pub fn highlight(&mut self, option_number: i32) {
        clear_term();
        let mut menu_copy = self.options.clone();
        menu_copy[option_number as usize - 1] = format!(
            "\x1b[43m\x1b[30m{}\x1b[0m",
            menu_copy[option_number as usize - 1].trim()
        );
        self.selected = option_number as i32;
        Self::show_menu(menu_copy);
    }

    pub fn select_menu_option(&mut self) -> String {
        let mut idx = self.selected;

        hide_cursor();
        loop {
            let key = self.term.read_key().unwrap();
            let option_len = self.options.len();
            select!(key, idx, option_len, self.highlight);
        }

        self.selected.to_string()
    }

    pub fn open_add(&mut self, manager: &mut manager::Manager) {
        let mut expense_name = "".to_string();
        let mut expense_amount = 0;
        clear_term();
        show_cursor();
        printil!("Press \x1b[32ma\x1b[0m to add an expense or any key to quit ");
        if let Ok(cmd) = get_input() {
            if cmd.as_str() != "a" {
                return;
            }
        }

        loop {
            clear_term();
            println!("\x1b[43m ADD EXPENSE \x1b[0m\n");

            if expense_name.is_empty() {
                printil!("Expense name ");
                let res = get_input();

                if let Ok(cmd) = res {
                    expense_name = either!(cmd.trim().len() == 0, String::from(""); cmd);
                    continue;
                }
            }

            if expense_amount == 0 {
                printil!("Expense amount ");
                let res = get_input();

                if let Ok(cmd) = res {
                    expense_amount = cmd.parse::<i32>().unwrap_or_default();
                    continue;
                }
            }

            manager.add_expense(&expense_name, expense_amount);
            break;
        }
    }

    pub fn open_view(&mut self, manager: &mut manager::Manager) {
        loop {
            clear_term();

            let expenses = manager.view_expenses();
            let total_amount = expenses_total(&expenses);
            println!("\x1b[43m VIEW EXPENSES \x1b[0m\t\tTOTAL: {total_amount}\n");

            show_expenses(&expenses, 0);
            printil!("\nPress \x1b[33mENTER\x1b[0m to go back");

            let input = get_input();

            if let Ok(cmd) = input {
                if cmd == "" {
                    break;
                }
            }
        }
    }

    pub fn open_remove(&mut self, manager: &mut manager::Manager) {
        let mut highlight_idx = 0;
        let mut expense_ids: HashMap<usize, String>;

        loop {
            clear_term();
            println!("\x1b[43m REMOVE EXPENSE \x1b[0m\n");

            let expenses = manager.view_expenses();
            expense_ids = show_expenses(&expenses, highlight_idx);
            printil!("\nPress \x1b[31mENTER\x1b[0m to delete the selected expense or \x1b[33mESC\x1b[0m to exit");

            let expenses_len = expense_ids.len();

            let key = self.term.read_key().unwrap();
            if key == Key::Escape {
                return;
            }

            let to_delete = select!(key, highlight_idx, expenses_len);

            if let Some(item_id) = to_delete {
                let expense_id = expense_ids.get(&(item_id as usize));

                if let Some(id) = expense_id {
                    manager.remove_expense(&id);
                    highlight_idx = 0;
                } 
            }
        }
    }
}

pub fn expenses_total(expenses: &Vec<sqlite::Row>) -> i32 {
    expenses
        .iter()
        .map(|row| row.try_get::<i64, usize>(2).unwrap_or_default() as i32)
        .collect::<Vec<i32>>()
        .iter()
        .sum()
}

fn show_expenses(expenses: &[sqlite::Row], highlight_idx: i32) -> HashMap<usize, String> {
    // println!("ID\tName\t\tAmount\n");
    let mut expense_ids: HashMap<usize, String> = HashMap::new();
    let mut current_idx: usize = 0;

    let gather_row = |row: &sqlite::Row| {
        let row_id = row.try_get::<i64, &str>("id");
        let row_name = row.try_get::<String, &str>("name");
        let row_amount = row.try_get::<i64, usize>(2);

        let mut row = "".to_owned();

        if let Ok(id) = row_id {
            expense_ids.insert(current_idx, id.to_string());
            current_idx += 1;
        }


        if let Ok(name) = row_name {
            row.push_str(&format!("{name}"));
        }

        if let Ok(amount) = row_amount {
            row.push_str(
                &(0..30 - (row.len() + amount.to_string().len()))
                    .map(|_| " ")
                    .collect::<String>(),
            );
            row.push_str(&format!("{}", amount.to_string()));
        }
        row
    };

    let mut rows = expenses.iter().map(gather_row).collect::<Vec<String>>();

    if rows.len() > 0 {
        rows[highlight_idx as usize] =
            format!("\x1b[43m\x1b[30m {} \x1b[0m", rows[highlight_idx as usize]);
    }

    println!(
        "\x1b[4mName{}Amount\x1b[0m\n",
        (0..20).map(|_| " ").collect::<String>()
    );

    if rows.len() == 0 {
        println!("\x1b[31mNo expenses\x1b[0m");
    } else {
        println!("{}", rows.join("\n"));
    }

    expense_ids
}
