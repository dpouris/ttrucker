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

use lib::{
    manager,
    menus::*,
    utils::{clear_term, show_cursor},
};
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
            let manager = man;
            let mut menu = Menu::new(manager);

            loop {
                menu.highlight(1);

                let opt = menu.select_menu_option();
                let func = parse_input(opt.as_str());

                match func.unwrap() {
                    Function::Add => menu.open_add(),
                    Function::View => menu.open_view(),
                    Function::Edit => menu.open_edit(),
                    Function::Remove => menu.open_remove(),
                    Function::Quit => {
                        clear_term();
                        break;
                    }
                }
            }
        }
    } else {
        println!("Cannot find HOME dir");
        process::exit(1)
    }
    show_cursor()
}
