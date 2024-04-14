use std::io;

const TEMPERATURE_FORMAT: &str = "<64-bit floating point number><F|f|C|c>";

fn main() {
    let mut console_input = String::new();

    println!("Enter the temperature to convert, in the format {TEMPERATURE_FORMAT}:");

    loop {
        if let Err(e) = read_line_from_console(&mut console_input) {
            println!("Could not read the temperature to convert: '{e}'.");
            println!("Please re-enter the temperature to convert:");

            continue;
        }

        let Some((degrees, unit)) = get_raw_degrees_and_unit_from_console_input(&console_input)
        else {
            print_invalid_temp_format_err();
            continue;
        };

        let Ok(degrees) = degrees.parse() else {
            print_invalid_temp_format_err();
            continue;
        };

        match unit {
            "F" | "f" => {
                println!(
                    "{degrees} degrees Fahrenheit are {} degrees Celsius.",
                    convert_fahrenheit_to_celsius(degrees)
                );
                break;
            }
            "C" | "c" => {
                println!(
                    "{degrees} degrees Celsius are {} degrees Fahrenheit.",
                    convert_celsius_to_fahrenheit(degrees)
                );
                break;
            }
            _ => {
                print_invalid_temp_format_err();
                continue;
            }
        };
    }
}

fn read_line_from_console(buffer: &mut String) -> io::Result<usize> {
    buffer.clear();
    io::stdin().read_line(buffer)
}

fn get_raw_degrees_and_unit_from_console_input(console_input: &str) -> Option<(&str, &str)> {
    let trimmed_console_input = console_input.trim();
    let unit_index = trimmed_console_input.len().saturating_sub(1);

    if trimmed_console_input.is_char_boundary(unit_index) {
        Some(trimmed_console_input.split_at(unit_index))
    } else {
        None
    }
}

fn print_invalid_temp_format_err() {
    println!(
        "The given temperature to convert is not in the required format ({TEMPERATURE_FORMAT})."
    );
    println!("Please enter the temperature to convert in the required format:");
}

fn convert_fahrenheit_to_celsius(degrees: f64) -> f64 {
    (degrees - 32.) * 5. / 9.
}

fn convert_celsius_to_fahrenheit(degrees: f64) -> f64 {
    degrees * 9. / 5. + 32.
}
