// Expenses app
// ------------
// You'll have 4 options:
// 1. Add expense
// 2. View expense
// 3. Edit expense
// 4. Remove expense
// (Bill total included by default)
//
// ---- SAVE / LOAD ----
// The program should save the expenses into a sqlite db in the users home dir under the expenses/ dir
// If the user already has an expenses/db.sqlite file then load it or else create it. (Give the user visual status)
//
// ---- FUNCTIONALITY ----
// 1. Give them a prompt to enter the name of an expese followed by the amount (save to db after)
// 2. Return the name and amount of the expenses to the user (query from db)
// 3. Enter a menu that lists all the expenses along with indicative numbers to choose in order to edit the name or amount of the expense.
//    The prompt will persist until the user presses q
// 4. Enter a menu that lists all the expenses along with indicative numbers to choose in order to delete it (remove from db after)
// NOTE: When the user is in the main menu show the total of all expenses

use lib::{manager, menus, utils::*};
use std::{fs, process};

enum Function {
    Add,
    View,
    Edit,
    Remove,
    Quit,
}

fn parse_input(input: &str) -> Option<Function> {
    match input {
        "1" => Some(Function::Add),
        "2" => Some(Function::View),
        "3" => Some(Function::Edit),
        "4" => Some(Function::Remove),
        "5" => Some(Function::Quit),
        _ => None,
    }
}

fn prepare_manager(path: &str, home: &str) -> Result<manager::Manager, ()> {
    let manager = manager::init_manager(path);

    if let Err(_) = manager {
        if let Ok(_) = fs::create_dir_all(format!("{home}/expenses")) {
            if let Err(msg) = fs::File::create(path) {
                println!("{msg}")
            }
            let mut man = manager::init_manager(path).expect("Cannot initialize manager");
            man.create_expenses_table();
            return Ok(man);
        }
        return Err(());
    }

    return Ok(manager.unwrap());
}

fn main() {
    let home = option_env!("HOME");

    if let Some(home) = home {
        let path_to_sqlite = format!("{}/expenses/db.sqlite", &home);
        // check if user has a expenses/db.sqlite if he doesn't create it
        let manager_res = prepare_manager(&path_to_sqlite, &home);
        if let Ok(man) = manager_res {
            let mut manager = man;
            let mut err = "";

            loop {
                clear();
                println!(
                    "1. Add expense\n2. View expenses\n3. Edit expense\n4. Remove expense\n5. Quit"
                );

                if !err.is_empty() {
                    println!("{}", err);
                }

                err = "";
                let res = get_input();
                if res.is_err() {
                    continue;
                }

                let opt = Some(1);
                let _ = opt.filter(|x| *x > 0);
                let func = parse_input(res.unwrap_or_else(|_| String::new()).as_str());
                if let None = func {
                    err = "Invalid input";
                    continue;
                }

                match func.unwrap() {
                    Function::Add => menus::open_add_menu(&mut manager),
                    Function::View => menus::open_view_menu(&mut manager),
                    Function::Edit => (),
                    Function::Remove => menus::open_remove_menu(&mut manager),
                    Function::Quit => {
                        clear();
                        break;
                    }
                }
            }
        }
    } else {
        println!("Cannot find HOME dir");
        process::exit(1)
    }
}
