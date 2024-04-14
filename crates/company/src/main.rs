mod company;
mod console_ui;

use company::Database;

fn main() {
    let mut db = Database::new();
    console_ui::print_greeting();

    loop {
        match console_ui::read_action() {
            Ok(action_opt) => {
                if console_ui::execute_action(action_opt, &mut db).is_continue() {
                    console_ui::print_enter_action_prompt();
                } else {
                    break;
                }
            }
            Err(e) => console_ui::print_io_error(&e),
        }
    }
}
