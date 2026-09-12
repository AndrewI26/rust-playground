use std::io;

fn main() {
    println!("Find the n'th fib number!");
    println!("Please enter a value for n: ");

    let mut n_input = String::new();

    io::stdin()
        .read_line(&mut n_input)
        .expect("Failed to read line");

    let n: u32 = match n_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number!");
            return;
        }
    };

    let mut num1 = 0;
    let mut num2 = 1;

    for i in 1..n {
        if i == 1 {
            println!("{num1}");
        }
        if i == 2 {
            println!("{num2}");
        }
        let tmp = num1 + num2;
        println!("{tmp}");

        num1 = num2;
        num2 = tmp;
    }
}
