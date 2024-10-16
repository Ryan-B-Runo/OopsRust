extern crate core;

use std::io::stdin;

fn prompt_only(prompt_text: String) -> String {
    let mut response = String::new();
    println!("{}", prompt_text);
    stdin().read_line(&mut response).expect("Error");
    match response.to_lowercase().trim().parse::<i32>() {
        Ok(_) => { response }
        Err(_) => { "[ERROR]".to_string() }
    }
}

fn prompt_all_bounds(prompt_text: String, error_message: String, lower_bound: i32, upper_bound: i32)-> String {
    let mut response = String::new();
    println!("{}", prompt_text);
    stdin().read_line(&mut response).expect("Error");
    let trimmed_response = response.to_lowercase().trim().parse::<i32>().expect("Error");

    if trimmed_response < upper_bound && trimmed_response > lower_bound{
        response
    } else {
        error_message
    }
}

    fn prompt_one_bound(prompt_text: String, error_message: String, bound: i32, is_upper_bound: bool) -> String {
        let mut response = String::new();
        println!("{}", prompt_text);
        stdin().read_line(&mut response).expect("Error");
        let trimmed_response = response.to_lowercase().trim().parse::<i32>().expect("Error");
        if is_upper_bound{
            if trimmed_response < bound{
                response
            }else{
                error_message
            }
        }else{
            if trimmed_response > bound{
                response
            }else{
                error_message
            }
        }
    }

fn main() {
    let mut value = prompt_only("Enter an int:".parse().unwrap());
    println!("The integer is {}.", value.trim_end());

    value = "Error".to_string();
    while value == "Error"{
        value = prompt_all_bounds("Enter an int 0-100".parse().unwrap(), "Error".parse().unwrap(), 0, 100);
        if value != "Error"{
            println!("The value chosen by the user is: {}.", value.trim_end());
        }
        else{
            println!("{}", value);
        }
    }

    value = "Error".to_string();
    while value == "Error"{
        value = prompt_one_bound("Enter an int >100".parse().unwrap(), "Error".to_string(), 100 , false);
        if value != "Error"{
            println!("The value chosen by the user is: {}.", value.trim_end());
        }
        else{
            println!("{}", value);
        }
    }

    value = "Error".to_string();
    while value == "Error"{
        value = prompt_one_bound("Enter an int <-20".parse().unwrap(), "Error".to_string(), -20 , true);
        if value != "Error"{
            println!("The value chosen by the user is: {}.", value.trim_end());
        }
        else{
            println!("{}", value);
        }
    }
}