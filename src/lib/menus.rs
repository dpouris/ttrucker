#![allow(unused)]

use crate::{either, printil, select, utils::{clear_term, get_input}, manager};
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
        menu_copy[option_number as usize - 1] =
            format!("\x1b[47m{}\x1b[0m", menu_copy[option_number as usize - 1].trim());
        self.selected = option_number as i32;
        Self::show_menu(menu_copy);
    }

    pub fn select_menu_option(&mut self) -> String {
        let mut idx = self.selected;

        self.term.hide_cursor();

        loop {
            let key = self.term.read_key().unwrap();
            select!(key, idx, self.options.len(), self.highlight);
        }

        self.term.show_cursor();
        self.selected.to_string()
    }

    pub fn open_add(&mut self, manager: &mut manager::Manager) {
        let mut expense_name = "".to_string();
        let mut expense_amount = 0;
        clear_term();
        printil!("Press \"a\" to add an expense or any key to quit ");
        if let Ok(cmd) = get_input() {
            if cmd.as_str() != "a" {
                return;
            }
        }

        loop {
            clear_term();
            println!("ADD EXPENSE\n");

            if expense_name.is_empty() {
                printil!("Expense name ");
                let res = get_input();

                if let Ok(cmd) = res {
                    expense_name = either!(cmd.trim().len() == 0, String::from(""); cmd);
                    // expense_name = if cmd.len() == 0 { "".to_string() } else { cmd };
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
            println!("Added expense {expense_name} with amount {expense_amount}");
            std::thread::sleep(std::time::Duration::new(3, 0));
            break;
        }
    }

    pub fn open_view(&mut self, manager: &mut manager::Manager) {
        loop {
            clear_term();

            let expenses = manager.view_expenses();
            let total_amount = expenses_total(&expenses);
            println!("VIEW EXPENSES\t\tTOTAL: {total_amount}\n");

            view_expenses(&expenses);

            printil!("Press \"q\" to quit ");
            let input = get_input();

            if let Ok(cmd) = input {
                if cmd == "q" {
                    break;
                }
            }
        }
    }

    pub fn open_remove(&mut self, manager: &mut manager::Manager) {
        loop {
            clear_term();
            println!("REMOVE EXPENSE\n");

            let expenses = manager.view_expenses();
            view_expenses(&expenses);

            printil!("Choose an expense by id to remove or press \"q\" to quit ");
            let input = get_input();

            if let Ok(cmd) = input {
                if cmd == "q" {
                    break;
                }
                manager.remove_expense(&cmd);
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

fn view_expenses(expenses: &[sqlite::Row]) {
    println!("ID\tName\t\tAmount\n");
    for row in expenses {
        let row_id = row.try_get::<i64, usize>(0);
        let row_name = row.try_get::<String, &str>("name");
        let row_amount = row.try_get::<i64, usize>(2);

        let mut row = "".to_owned();
        if let Ok(id) = row_id {
            row.push_str(&format!("{}\t", id.to_string()));
        }

        if let Ok(name) = row_name {
            row.push_str(&format!("{name}\t"));
        }
        if let Ok(amount) = row_amount {
            row.push_str(&format!("\t{}", amount.to_string()));
        }

        println!("{row}")
    }
    println!();

}