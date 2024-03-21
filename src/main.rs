use std::io;

fn main() {
    println!("This program computes the n-th fibonacci number!");
    println!("Let the first number be the 0-th number.\n");

    let desired_position: u32 = read_integer();
    let result: u32 = iterative_fib(desired_position);
    println!("The {desired_position}-th fibonacci number is {result}");
}

fn read_integer() -> u32 {
    loop {
        println!("Enter the position of the number you wanna generate: ");
        let mut input = String::new();
    
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse() {
            Ok(user_integer) => return user_integer,
            Err(_) => {
                println!("Input must be an unsigned integer!");
                continue;
            }
        }
    };
}

fn iterative_fib(position: u32) -> u32 {
    if position == 0 {
        0
    } else if position == 1 {
        1
    } else {
        let mut previous: u32 = 0;
        let mut current: u32 = 1;

        let mut index: u32 = 2;
        while index <= position {
            let temp: u32 = current;
            current += previous;
            previous = temp;
            
            index += 1;
        }

        current
    }
}

fn recursive_fib(position: u32) -> u32 {
    if position == 0 {
        0
    } else if position == 1 {
        1
    } else {
        recursive_fib(position - 1) + recursive_fib(position - 2)
    }
}
