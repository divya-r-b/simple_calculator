use std::io;

fn main() {
    println!("Simple Calculator");

    loop {
        println!("Choose an operation:");
        println!("1. Add");
        println!("2. Subtract");
        println!("3. Multiply");
        println!("4. Divide");
        println!("5. Quit");

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a valid number.");
                continue;
            }
        };

        if choice == 5 {
            break;
        }

        let (_operation, operator) = match choice {
            1 => ("Addition", "+"),
            2 => ("Subtraction", "-"),
            3 => ("Multiplication", "*"),
            4 => ("Division", "/"),
            _ => {
                println!("Invalid choice. Please select a valid operation.");
                continue;
            }
        };

        let mut input1 = String::new();
        let mut input2 = String::new();

        println!("Enter the first number:");
        io::stdin()
            .read_line(&mut input1)
            .expect("Failed to read line");

        println!("Enter the second number:");
        io::stdin()
            .read_line(&mut input2)
            .expect("Failed to read line");

        let num1: f64 = match input1.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input for the first number.");
                continue;
            }
        };

        let num2: f64 = match input2.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input for the second number.");
                continue;
            }
        };

        let result = match choice {
            1 => num1 + num2,
            2 => num1 - num2,
            3 => num1 * num2,
            4 => {
                if num2 == 0.0 {
                    println!("Division by zero is not allowed.");
                    continue;
                }
                num1 / num2
            }
            _ => {
                println!("Invalid choice. Please select a valid operation.");
                continue;
            }
        };

        println!("{} {} {} = {}", num1, operator, num2, result);
    }
}
