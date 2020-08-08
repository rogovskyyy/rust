
const PLN: f32 = 1.00;
const EUR: f32 = 4.41;
const USD: f32 = 3.74;
const GBP: f32 = 4.88;

fn main() {
    println!("------------------------------------------------");
    println!("Welcome to currency converter. \nPlease select your currency: \n 1.PLN \n 2.EUR \n 3.USD \n 4.GBP");
    let mut option: String = get_input("");
    let currency_from: Currency = match option.as_str() {
        "1" => Currency::PLN,
        "2" => Currency::EUR,
        "3" => Currency::USD,
        "4" => Currency::GBP,
        _ => Currency::NONE
    };

    println!("Select currency you want convert to: \n 1.PLN \n 2.EUR \n 3.USD \n 4.GBP");
    option = get_input("");
    let currency_to: Currency = match option.as_str() {
        "1" => Currency::PLN,
        "2" => Currency::EUR,
        "3" => Currency::USD,
        "4" => Currency::GBP,
        _ => Currency::NONE
    };

    let amount: String = get_input("Amount of money");
    let amount_f32: f32 = match amount.parse::<f32>() {
        Ok(ok) => ok,
        Err(er) => panic!("ERROR: {}", er)
    };

    println!("{}", calculate(amount_f32, currency_from, currency_to));

}

enum Currency {
    PLN,
    EUR,
    USD,
    GBP,
    NONE
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

fn calculate(amount: f32, from: Currency, to: Currency) -> f32 {
    let mut _from: f32 = match from {
        Currency::PLN => PLN,
        Currency::EUR => EUR,
        Currency::USD => USD,
        Currency::GBP => GBP,
        _ => PLN
    };

    let mut _to: f32 = match to {
        Currency::PLN => PLN,
        Currency::EUR => EUR,
        Currency::USD => USD,
        Currency::GBP => GBP,
        _ => PLN
    };

    (amount * _from) / _to
} 