use std::io;

macro_rules! maximum_allowed_fibonacci_number_index {
    () => {
        186u8
    };
}

const MAXIMUM_ALLOWED_FIBONACCI_NUMBER_INDEX: u8 = maximum_allowed_fibonacci_number_index!();
const ALLOWED_FIBONACCI_NUMBER_INDICES: &str =
    concat!("0-", maximum_allowed_fibonacci_number_index!());

fn main() {
    println!("Enter the index of the Fibonacci number you want to generate ({ALLOWED_FIBONACCI_NUMBER_INDICES}):");
    let mut console_input = String::new();

    loop {
        if let Err(e) = read_console_input(&mut console_input) {
            println!("Could not read the index of the Fibonacci number to generate: '{e}'");
            ask_to_re_enter_fibonacci_number_index();
            continue;
        }

        let Ok(fibonacci_number_index) = console_input.trim().parse() else {
            print_invalid_fibonacci_number_index_error();
            continue;
        };

        let Some(fibonacci_number) = generate_fibonacci_number(fibonacci_number_index) else {
            print_invalid_fibonacci_number_index_error();
            continue;
        };

        println!("The Fibonacci number at index {fibonacci_number_index} is {fibonacci_number}.");
        break;
    }
}

fn read_console_input(buffer: &mut String) -> io::Result<usize> {
    buffer.clear();
    io::stdin().read_line(buffer)
}

fn ask_to_re_enter_fibonacci_number_index() {
    println!("Please re-enter the index of the Fibonacci number you want to generate ({ALLOWED_FIBONACCI_NUMBER_INDICES}):");
}

fn print_invalid_fibonacci_number_index_error() {
    println!("The given input is not a valid Fibonacci number index.");
    ask_to_re_enter_fibonacci_number_index();
}

fn generate_fibonacci_number(index: u8) -> Option<u128> {
    match index {
        0u8 => Some(0u128),
        1u8 => Some(1u128),
        index if index <= MAXIMUM_ALLOWED_FIBONACCI_NUMBER_INDEX => {
            let mut fn_1 = 0u128;
            let mut fn_2 = 1u128;

            for _ in 2u8..=index {
                let temp = fn_2;
                fn_2 = fn_1 + temp;
                fn_1 = temp;
            }

            Some(fn_2)
        }
        _ => None,
    }
}
