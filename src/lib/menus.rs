use std::{vec, thread::sleep, time::Duration};

use crate::{
    either,
    manager::Manager,
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

            let expenses = self.manager.get_expenses();
            let total_amount = expenses_total(&expenses);
            println!("\x1b[43m VIEW EXPENSES \x1b[0m\t\tTOTAL: {total_amount}\n");

            show_expenses(&expenses, 0, true);
            printil!("\nPress \x1b[33mANY KEY\x1b[0m to go back");

            let key = self.term.read_key();

            if let Ok(_) = key {
                break
            }
        }
    }

    pub fn open_edit(&mut self) {
        let mut highlight_idx = 0;
        
        loop {
            let expenses = self.manager.get_expenses();
            let expense_ids = get_expense_ids(&expenses);
            let expenses_len = expenses.len();
            clear_term();
            println!("\x1b[43m EDIT EXPENSE \x1b[0m\n");
            
            show_expenses(&expenses, highlight_idx, false);
            printil!("\nPress \x1b[33mENTER\x1b[0m to edit the selected expense or \x1b[33mESC\x1b[0m to exit");
            
            let key = self.term.read_key().unwrap();
            
            let to_edit = select!(key, highlight_idx, expenses_len);
            if key == Key::Escape {
                return;
            }

            
            if let Some(item_idx) = to_edit {
                let expense_id = expense_ids[item_idx as usize].clone();
                if expenses_len > 0 {
                    self.edit_expense(&expense_id);
                    highlight_idx = 0;
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

            let expenses = self.manager.get_expenses();
            expense_ids = get_expense_ids(&expenses);
            let expenses_len = expense_ids.len();

            show_expenses(&expenses, highlight_idx, false);

            printil!("\nPress \x1b[31mENTER\x1b[0m to delete the selected expense or \x1b[33mESC\x1b[0m to exit");


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

    fn edit_expense(&mut self, expense_id: &str) {
        let mut highlight = 0;
        let expense = get_name_amount(&self.manager.get_single_expense(expense_id));
        loop {
            clear_term();
            let mut expense_cp = expense.clone();
            expense_cp[highlight as usize] = format!("\x1b[47m{}\x1b[0m",expense_cp[highlight as usize]); 
            println!(" {} \n {}", expense_cp[0], expense_cp[1]);
            let key = self.term.read_key().unwrap();

            let edit_field = select!(key, highlight, 2);
            
            if let Some(field) = edit_field {
                let new_field_value = self.term.read_line_initial_text(&expense[highlight as usize]).unwrap();
                self.manager.edit_expense(&expense_id, &[if field == 0 {"name"} else {"amount"}, new_field_value.as_str()]);
                break;
            }
        }
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

fn show_expenses(expenses: &[sqlite::Row], highlight_idx: i32, no_highlight: bool) {
    let mut rows = get_expenses_vec(expenses);
    
    if !no_highlight && rows.len() > 0 {
        rows[highlight_idx as usize] =
            format!("\x1b[43m\x1b[30m {} \x1b[0m", rows[highlight_idx as usize]);
    }

    println!(
        "\x1b[4mName{}Amount\x1b[0m\n",
        (0..50).map(|_| " ").collect::<String>()
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
        let row_amount = row.try_get::<i64, &str>("amount");

        let mut row = "".to_owned();

        if let Ok(name) = row_name {
            row.push_str(&format!("{name}"));
        }

        if let Ok(amount) = row_amount {
            row.push_str(
                &(0..60 - (row.len() + amount.to_string().len()))
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

fn get_name_amount(expense: &sqlite::Row) -> [String; 2] {
    let expense_name = expense.try_get::<String, &str>("name").unwrap();
    let expense_amount = expense.try_get::<i64, &str>("amount").unwrap();

    [expense_name, expense_amount.to_string()]
}