const DATA: [char; 26] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];

fn main() {
    let code: String = get_input("Enter your key to ROT13: ").to_ascii_lowercase();
    let mut secred_code: String = "".to_owned();
    let mut character: char;

    for iter in code.chars() {
        character = mix_char(iter);
        secred_code.push_str(&character.to_string());
    }
    
    println!("{}", secred_code);
}

fn mix_char(item: char) -> char {
    let index: usize = DATA.iter().position(|&r| r == item).unwrap();
    let length: usize = DATA.len() - 1;

    if (index + 13) <= length {
        DATA[index + 13 as usize]
    } else if index + 13 > length {
        DATA[calc_equation(index, length)]
    } else {
        item
    }
}

fn calc_equation(index: usize, length: usize) -> usize {
    let function: usize = (index + 13) - length - 1;
    function
}


fn get_input(prompt: &str) -> String {
    use std::io;
    println!("{}",prompt);
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_goes_into_input_above) => {},
        Err(_no_updates_is_fine) => {},
    }
    input.trim().to_string()
}