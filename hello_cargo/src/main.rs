use std::io;
use rand::Rng;

fn main() {

    let mut guess = String::new();

    let rand = rand::thread_rng().gen_range(1..101);

    io::stdin().read_line(&mut guess)
        .expect("failed");

    let guess: u32 = guess.trim().parse()
        .expect("format error");

    println!("rand is {}", rand);
    println!("guess is {}", guess);

    if guess == rand {
        println!("yes");
    } else {
        println!("no");
    }
}
