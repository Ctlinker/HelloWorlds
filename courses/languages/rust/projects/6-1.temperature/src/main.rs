use std::io;

fn main() {
    'mainloop: loop {
        let (temp, unit): (f32, i8) = loop {
            println!("Choose the unit of your number:\n 0 - Exit\n 1 - Celsius\n 2 - Fahrenheit\n");

            let mut choices = String::new();
            read_and_mutate(&mut choices);

            let choices: i8 = match choices.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please enter a valid number.");
                    continue;
                }
            };

            if choices == 0 {
                println!("You have chosen to exit. Bye!");
                break 'mainloop;
            }

            println!("Enter the temperature:");

            let mut temp = String::new();
            read_and_mutate(&mut temp);

            let temp: f32 = match temp.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please enter a valid number.");
                    continue;
                }
            };

            if choices == 1 || choices == 2 {
                break (temp, choices); // Exit and return the tuple
            }

            println!(
                "Either you chose an invalid option or something went wrong. Please try again."
            );
        };

        if unit == 1 {
            println!("{}°C is {}°F", temp, celsius_to_fahrenheit(temp));
        } else {
            println!("{}°F is {}°C", temp, fahrenheit_to_celsius(temp));
        }
    }
}

fn read_and_mutate(v: &mut String) {
    io::stdin().read_line(v).expect("Error while reading line");
}

fn celsius_to_fahrenheit(c: f32) -> f32 {
    (c * 9.0 / 5.0) + 32.0
}

fn fahrenheit_to_celsius(f: f32) -> f32 {
    (f - 32.0) * 5.0 / 9.0
}
