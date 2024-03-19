use regex::Regex;
use std::io;

fn main() {
    let mut guess = String::new();

    println!("Enter a number followed by C or F (example: 37C or -40F)");
    io::stdin()
        .read_line(&mut guess)
        .expect("Could not read line");

    let re = Regex::new(r"(-?\d+)([CF])").unwrap();
    match re.captures(&guess) {
        Some(caps) => {
            let number: f32 = caps.get(1).unwrap().as_str().parse().unwrap();
            let scale = caps.get(2).unwrap().as_str();
            let new_number;
            if scale == "C" {
                new_number = number * 9.0 / 5.0 + 32.0
            } else {
                new_number = (number - 32.0) * 5.0 / 9.0
            }
            let new_scale = if scale == "C" { "F " } else { "C" };

            println!("{new_number:.2}{new_scale}");
        }
        None => println!("Wrong format, must be a number followed by C or F"),
    }
}
