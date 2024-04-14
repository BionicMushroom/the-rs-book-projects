use crate::company::Database;

use std::io;
use std::ops::ControlFlow;

pub enum Action {
    AddEmployee { name: String, department: String },
    RemoveEmployee { name: String, department: String },
    ListEmployees { department: String },
    ListAllEmployees,
    Exit,
}

pub fn read_action() -> io::Result<Option<Action>> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;

    Ok(parse_action(&buffer))
}

pub fn execute_action(action_opt: Option<Action>, db: &mut Database) -> ControlFlow<()> {
    if let Some(action) = action_opt {
        match action {
            Action::AddEmployee { name, department } => {
                add_employee(name.as_str(), department.as_str(), db);
                ControlFlow::Continue(())
            }
            Action::RemoveEmployee { name, department } => {
                remove_employee(name.as_str(), department.as_str(), db);
                ControlFlow::Continue(())
            }
            Action::ListEmployees { department } => {
                list_employees(department.as_str(), db);
                ControlFlow::Continue(())
            }
            Action::ListAllEmployees => {
                list_all_employees(db);
                ControlFlow::Continue(())
            }
            Action::Exit => ControlFlow::Break(()),
        }
    } else {
        print_unrecognized_action();
        ControlFlow::Continue(())
    }
}

pub fn print_greeting() {
    println!("Welcome! Here is the list of available actions:");
    print_allowed_actions();
    print_enter_action_prompt();
}

pub fn print_enter_action_prompt() {
    println!();
    println!("Enter your desired action:");
}

pub fn print_io_error(e: &io::Error) {
    println!();
    println!("Could not read the desired action: '{e}'.");
    println!("Please re-enter the desired action:");
}

fn parse_action(raw_action: &str) -> Option<Action> {
    let args = raw_action.split_whitespace().collect::<Vec<&str>>();
    match args.len() {
        1 => parse_one_argument_action(args[0]),
        2 => parse_two_arguments_action(args[0], args[1]),
        4 => parse_four_argument_action(args[0], args[1], args[2], args[3]),
        _ => None,
    }
}

fn parse_one_argument_action(arg: &str) -> Option<Action> {
    if arg.to_lowercase() == "exit" {
        Some(Action::Exit)
    } else {
        None
    }
}

fn parse_two_arguments_action(arg1: &str, arg2: &str) -> Option<Action> {
    if arg1.to_lowercase() == "list" && arg2.to_lowercase() == "employees" {
        Some(Action::ListAllEmployees)
    } else {
        None
    }
}

fn parse_four_argument_action(arg1: &str, arg2: &str, arg3: &str, arg4: &str) -> Option<Action> {
    match arg1.to_lowercase().as_str() {
        "add" => parse_add_action(arg2, arg3, arg4),
        "remove" => parse_remove_action(arg2, arg3, arg4),
        "list" => parse_list_employees_in_department_action(arg2, arg3, arg4),
        _ => None,
    }
}

fn parse_add_action(arg2: &str, arg3: &str, arg4: &str) -> Option<Action> {
    if arg3.to_lowercase() == "to" {
        Some(Action::AddEmployee {
            name: arg2.into(),
            department: arg4.into(),
        })
    } else {
        None
    }
}

fn parse_remove_action(arg2: &str, arg3: &str, arg4: &str) -> Option<Action> {
    if arg3.to_lowercase() == "from" {
        Some(Action::RemoveEmployee {
            name: arg2.into(),
            department: arg4.into(),
        })
    } else {
        None
    }
}

fn parse_list_employees_in_department_action(arg2: &str, arg3: &str, arg4: &str) -> Option<Action> {
    if arg2.to_lowercase() == "employees" && arg3.to_lowercase() == "in" {
        Some(Action::ListEmployees {
            department: arg4.into(),
        })
    } else {
        None
    }
}

fn add_employee(name: &str, department: &str, db: &mut Database) {
    if db.add_employee(name, department) {
        println!("Employee {name} has been added to department {department}.");
    } else {
        println!("Employee {name} already exists in department {department}.");
    }
}

fn remove_employee(name: &str, department: &str, db: &mut Database) {
    if db.remove_employee(name, department) {
        println!("Employee {name} has been removed from department {department}.");
    } else {
        println!("Employee {name} does not exist in department {department}.");
    }
}

fn list_employees(department: &str, db: &Database) {
    let employees_iter = db.employees(department);

    if employees_iter.len() == 0 {
        println!("There are no employees in department {department}.");
    } else {
        for employee in employees_iter {
            println!("{employee}");
        }
    }
}

fn list_all_employees(db: &Database) {
    let departments_iter = db.departments();

    if departments_iter.len() == 0 {
        println!("There are no employees.");
    } else {
        for (department, employees_iter) in departments_iter {
            println!("Employees in department {department}:");
            for employee in employees_iter {
                println!("{employee}");
            }
            println!("-----------------------------------------------");
        }
    }
}

fn print_allowed_actions() {
    println!();
    println!("add <employee_name> to <department_name>");
    println!("remove <employee_name> from <department_name>");
    println!("list employees in <department_name>");
    println!("list employees");
    println!("exit");
}

fn print_unrecognized_action() {
    println!();
    println!("The desired action was not recognized.");
    println!("Here is the list of available actions:");
    print_allowed_actions();
}
