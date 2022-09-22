// #![allow(unused)]

use std::{vec};

use crate::{
    either,
    manager::{Manager},
    printil, select,
    utils::{clear_term, get_input, hide_cursor, show_cursor},
};
use console::{Key, Term};

pub struct Menu {
    pub selected: i32,
    pub options: Vec<String>,
    manager: Manager,
    term: Term,
}

impl Menu {
    pub fn new(manager: Manager) -> Self {
        let menu = Self {
            options: vec![
                "Add expense".to_owned(),
                "View expenses".to_owned(),
                "Edit expense".to_owned(),
                "Remove expense".to_owned(),
                "Quit".to_owned(),
            ],
            selected: 1,
            manager: manager,
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

    pub fn open_add(&mut self) {
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

            self.manager.add_expense(&expense_name, expense_amount);
            break;
        }
    }

    pub fn open_view(&mut self) {
        loop {
            clear_term();

            let expenses = self.manager.view_expenses();
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

    pub fn open_remove(&mut self) {
        let mut highlight_idx = 0;
        let mut expense_ids: Vec<String>;

        loop {
            clear_term();
            println!("\x1b[43m REMOVE EXPENSE \x1b[0m\n");

            let expenses = self.manager.view_expenses();
            show_expenses(&expenses, highlight_idx);

            expense_ids = get_expense_ids(&expenses);
            printil!("\nPress \x1b[31mENTER\x1b[0m to delete the selected expense or \x1b[33mESC\x1b[0m to exit");

            let expenses_len = expense_ids.len();

            let key = self.term.read_key().unwrap();
            let to_delete = select!(key, highlight_idx, expenses_len);
            if key == Key::Escape {
                return;
            }

            if let Some(item_idx) = to_delete {
                if expenses_len > 0 {
                    self.manager.remove_expense(&expense_ids[item_idx as usize]);
                    highlight_idx = 0;
                }
            }
        }
    }

    fn show_menu(menu: Vec<String>) {
        println!("{}\r", menu.join("\n"));
    }
}

fn expenses_total(expenses: &Vec<sqlite::Row>) -> i32 {
    expenses
        .iter()
        .map(|row| row.try_get::<i64, usize>(2).unwrap_or_default() as i32)
        .collect::<Vec<i32>>()
        .iter()
        .sum()
}

fn show_expenses(expenses: &[sqlite::Row], highlight_idx: i32) {
    let mut rows = get_expenses_vec(expenses);

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
}

fn get_expenses_vec(expenses: &[sqlite::Row]) -> Vec<String> {
    let gather_row = |row: &sqlite::Row| {
        let row_name = row.try_get::<String, &str>("name");
        let row_amount = row.try_get::<i64, usize>(2);

        let mut row = "".to_owned();

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

    let rows = expenses.iter().map(gather_row).collect::<Vec<String>>();

    rows
}

fn get_expense_ids(expenses: &[sqlite::Row]) -> Vec<String> {
    let mut expense_ids = vec![];
    for expense_row in expenses.iter() {
        let row_id = expense_row.try_get::<i64, &str>("id");

        if let Ok(id) = row_id {
            expense_ids.push(id.to_string());
        }
    }

    expense_ids
}
