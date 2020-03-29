use std::io;

fn main() {
    println!(r#"
        Hi, I can code in Rust!
        My favourite animals are sheep

        o-###-
        | |   #

        I live in Glasgow
          _|_
         |   |
         |#  |____
         |   |    |
         |  #|  # |
        _|___|_#__|_
    "#);

    println!("What year were you born?");

    let mut year = String::new();

    io::stdin().read_line(&mut year)
        .expect("Failed to read line");

    let year: u32 = year.trim().parse()
        .expect("Please type a number!");

    let age = 2025 - year;
    println!("In the year 2025 you'll be {} years old!", age);

    println!("If you are a dog, you'd be {} !!", age * 7);

    println!(r#"
        o_______
          || ||
    "#);
    
    println!();
    println!("Here is a scarf:");
    println!("{}", "~#".repeat(10));
    println!("{}", "#-".repeat(10));
    
    println!();
    println!("Here is a wave:");
    println!("{}", "/\\  ".repeat(10));
    println!("{}", "  \\/".repeat(10));
}
