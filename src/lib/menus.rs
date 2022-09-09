use crate::{either, manager, printil, utils::*};

pub fn open_add_menu(manager: &mut manager::Manager) {
    let mut expense_name = "".to_string();
    let mut expense_amount = 0;

    clear();
    printil!("Press \"a\" to add an expense or any key to quit ");
    if let Ok(cmd) = get_input() {
        if cmd.as_str() != "a" {
            return;
        }
    }

    loop {
        clear();
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

pub fn open_view_menu(manager: &mut manager::Manager) {
    loop {
        clear();

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

pub fn open_remove_menu(manager: &mut manager::Manager) {
    loop {
        clear();
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
    println!()
}
