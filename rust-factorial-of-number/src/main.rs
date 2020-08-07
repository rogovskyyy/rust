fn main() {
    let input: String = get_input("Enter factorial of a number: ");
    let is_string: Option<i32> = is_not_string(input);
    match is_string {
        Some(x) => match is_greater_than_zero(x) {
            Some(x) => println!("{}", x),
            None => user_error()
        },
        None => user_error()
    }
}

fn user_error() {
    println!("An error occured, please make sure you entered correct format (i > 0) & (not string) ");
}

fn is_not_string(item: String) -> Option<i32> {
    let result: Option<i32> = match item.parse::<i32>() {
        Ok(ok) => Some(ok),
        Err(_) => None
    };
    result
}

fn is_greater_than_zero(item: i32) -> Option<i32> {
    if item > 0 {
        Some(calculate(item))
    } else {
        None
    }
}

fn calculate(item: i32) -> i32 {
    let mut res: i32 = 1;
    for iter in 1..(item + 1) {
        res = res * iter;
    }
    res
}

fn get_input(prompt: &str) -> String{
    use std::io;
    println!("{}",prompt);
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_goes_into_input_above) => {},
        Err(_no_updates_is_fine) => {},
    }
    input.trim().to_string()
}
