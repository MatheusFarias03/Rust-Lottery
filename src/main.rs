#![allow(unused_imports)]
use rand::prelude::*;
use std::io::{self, Read, Write};

/// Clears the screen and sets the cursor at first row and column.
fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

/// Generates the 6 drawn numbers of the lottery.
fn generate_numbers() -> Vec<u16> {
    // Enables random number generation.
    let mut rng = rand::thread_rng();

    // Vector to store the numbers. They cannot repeat.
    let mut drawn_numbers:Vec<u16> = Vec::new();

    // Generate numbers and add them to our vector.
    let mut i = 0;
    while i < 6 {
        let generated_num:u16 = rng.gen_range(1..=60);
        match drawn_numbers.contains(&generated_num) {
            false => {
                drawn_numbers.push(generated_num);
                i += 1;
            },
            true => continue
        }
    }
    drawn_numbers.sort();
    return drawn_numbers
}

/// Print all numbers from the passed vector.
fn print_vector(vec: &mut Vec<u16>) {
    for i in vec.iter() {
        print!(" {} ", i);
    }
    println!("");
}

/// Allows the user to select the amount of numbers that he/she wants to choose.
fn number_quantity() -> u8 {

    // Get user input and convert it to integer.
    let mut got_num:bool = false;
    let mut num_selected:u8 = 0;
    let mut input = String::new();

    // Error variables.
    let mut got_error:bool = false;
    let mut error_msg = String::new();

    while got_num == false {

        clear_screen();

        if got_error == true {
            println!("{}", error_msg);
        }
        println!("Select the amount of numbers you want to chose (between 6 and 15) : ");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");

        let number_selected:u8 = match input.trim().parse() {
            Ok(num) => {
                if num <= 15 && num >= 6 {
                    got_num = true;
                    num
                } else {
                    got_error = true;
                    error_msg = ("Selected number is invalid. Try selecting a number between 6 and 15.").to_string();
                    input.clear();
                    continue
                }
            },
            Err(_) => {
                got_error = true;
                error_msg = ("Not an integer.").to_string();
                input.clear();
                continue
            },
        };

        num_selected = number_selected;
    }

    return num_selected
    
}

/// Returns the cost of the selected quantity of numbers.
fn qtd_cost(qtd: u8) -> f64 {

    match qtd {
        6 => 4.5,
        7 => 31.5,
        8 => 126.0,
        9 => 378.0,
        10 => 945.0,
        11 => 2_079.0,
        12 => 4_158.0,
        13 => 7_722.0,
        14 => 13_513.0,
        15 => 22_522.5,
        _ => 0.0
    }

}

/// Allows the user to select the numbers according to a passed quantity.
fn chosen_numbers(qtd: u8) -> Vec<u16> {

    // Error variables.
    let mut got_error:bool = false;
    let mut error_msg:String = String::new();

    // Vector to store the user's chosen numbers.
    let mut chosen_vec:Vec<u16> = Vec::new();

    // Add user's input numbers into the vector.
    let mut i:u8 = 0;
    
    while i < qtd {
        
        clear_screen();

        println!("Your chosen numbers are:");
        print_vector(&mut chosen_vec);
        println!("\nYou chose : {} numbers.\n", i);

        if got_error == true {
            println!("{}", error_msg);
        }
        println!("Select a number between 1 and 60 to add to your list of numbers : ");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");
        
        match input.trim().parse::<u16>() {
            Ok(num) => {
                match chosen_vec.contains(&num) {
                    false => {
                        if num >= 1 && num <= 60 {
                            chosen_vec.push(num);
                            i += 1;
                            got_error = false;
                        } else {
                            got_error = true;
                            error_msg = ("Number out of the list. Try selecting a number between 1 and 60.").to_string();
                        }
                    },
                    true => {
                        got_error = true;
                        error_msg = ("You have already selected this number.").to_string();
                        continue;
                    }
                }
            },
            Err(_) => {
                got_error = true;
                error_msg = ("Not an integer.").to_string();
                continue;
            }
        };
    }

    chosen_vec.sort();
    return chosen_vec
}

/// Checks if the player guessed the right numbers of the lottery.
fn check_result(user_vec: &mut Vec<u16>, drawn_vec: &mut Vec<u16>, money_spent: f64, attempts: u16) {

    clear_screen();

    println!("You guessed:");
    print_vector(user_vec);

    println!("The right numbers are:");
    print_vector(drawn_vec);

    let mut matching:u8 = 0;

    for i in drawn_vec {
        for j in &mut *user_vec {
            if i == j {
                matching += 1;
            }
        }
    }

    if matching < 6 {
        println!("Too bad! You guessed only {} numbers correctly.\nYou have spent R$ {} already.", matching, money_spent);
        println!("You have tried {} times.", attempts);
    } else {
        println!("Hooray! You won R$ 500.000.000,00 ! Your profit is R$ {} .", 500_000_000.00 - money_spent);
        println!("It took you {} attempts.", attempts);
    }

}

/// Function for asking the player if he/she wants to play again.
fn play_again() -> bool {

    println!("\nDo you want to play again? (Y/N) ");

    let mut option: char = '\0';
    let stdin = io::stdin();
    let input = &mut String::new();

    input.clear();  
    io::stdout().flush().expect("Failed to flush old data.");
    stdin.read_line(input).expect("Failed to read line.");
    option = input.chars().nth(0).unwrap();
    option = option.to_ascii_uppercase();

    match option {
        'Y' => true,
        'N' => false,
        _ => false
    }

}

fn main() {

    let mut drawn_numbers:Vec<u16> = Vec::new();
    let mut player_numbers:Vec<u16> = Vec::new();
    let mut qtd:u8 = 0;
    let mut total_spent:f64 = 0.0;
    let mut attempts:u16 = 0;

    let mut is_playing:bool = true;

    while is_playing == true {

        attempts += 1;

        drawn_numbers = generate_numbers();

        qtd = number_quantity();
        total_spent += qtd_cost(qtd);

        player_numbers = chosen_numbers(qtd);

        check_result(&mut player_numbers, &mut drawn_numbers, total_spent, attempts);

        is_playing = play_again();
    }
}
