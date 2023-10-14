use std::io::Write;

fn guess_number() {
    use rand::Rng;
    use std::cmp::Ordering;
    use std::io;

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Guess the number!");

    loop {
        print!("Please input the number: ");
        io::stdout().flush().expect("");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("Too big"),
            Ordering::Less => println!("To small"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

pub fn main1() {
    guess_number();
}
