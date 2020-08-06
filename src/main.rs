fn main() -> std::io::Result<()> {
    use std::net::{TcpStream, TcpListener};
    use std::io::prelude::*;

    let address: String = String::from("127.0.0.1:1407");
    let username: String = simple_user_input::get_input("Enter your nickname: ");
    let mut stream: TcpStream = TcpStream::connect(address).expect("Couldn't connect to the server...");
    let mut listener: TcpListener = TcpListener::bind(stream).expect("Couldn't connect to the server...");
    
    loop {
        let formated_text: String = String::from(format!("{} >", username));
        let formated_text_str: &str = &formated_text;

        let message: String = simple_user_input::get_input(formated_text_str);
        let data_bytes = format!("{user} > {data}", data = message, user = username);
        let data = data_bytes.as_bytes();
    }
}

mod simple_user_input {
    use std::io;
    pub fn get_input(prompt: &str) -> String{
        println!("{}", prompt);
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_goes_into_input_above) => {},
            Err(_no_updates_is_fine) => {},
        }
        input.trim().to_string()
    }
}