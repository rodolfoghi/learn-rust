use rand::Rng;
use std::io;

fn main() {
    let chars = "abcdefghijklmnopqrstuvxz1234567890!@#$%&*()-+=?;:.><.\\|{}[]";
    println!("Password Generator");
    println!("==================");

    println!("number of passwords?");
    let mut number_of_passwords = String::new();

    io::stdin()
        .read_line(&mut number_of_passwords)
        .expect("Failed to read number of passwords");

    let number_of_passwords: u32 = number_of_passwords
        .trim()
        .parse()
        .expect("Please type a number!");

    println!("password length?");
    let mut password_length = String::new();

    io::stdin()
        .read_line(&mut password_length)
        .expect("Failed to read number of passwords");

    let password_length: u32 = password_length
        .trim()
        .parse()
        .expect("Please type a number!");

    for _ in 0..number_of_passwords {
        let mut new_password = String::new();

        for _ in 0..password_length {
            let start: usize = rand::thread_rng().gen_range(1, chars.len()) as usize;
            let end: usize = start + 1;
            new_password.push_str(&chars[start..end]);
        }

        println!("{}", new_password);
    }
}
