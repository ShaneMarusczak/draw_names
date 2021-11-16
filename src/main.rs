use rand::{seq::SliceRandom, thread_rng};
use std::io;

fn main() {
    let mut names = Vec::<String>::new();

    println!("Draws one or more random names from a list!");
    println!("Enter one name at a time.");
    println!("Enter \"DONE\" when complete.");

    loop {
        let mut name = String::new();

        io::stdin()
            .read_line(&mut name)
            .expect("Failed to read line");

        match name.trim() {
            "DONE" => break,
            _ => names.push(String::from(name.trim())),
        }
    }
    let names_len = names.len();

    let mut winner_count_str = String::new();
    let mut winner_count_num: usize = 0;
    while winner_count_num == 0 || winner_count_num > names_len {
        println!("How many names would you like to draw?");

        io::stdin()
            .read_line(&mut winner_count_str)
            .expect("Failed to read line");
        winner_count_num = match winner_count_str.trim().parse() {
            Ok(num) => num,
            Err(_) => 0,
        };
        if winner_count_num == 0 || winner_count_num > names_len {
            winner_count_str.clear();
            println!("Invalid Input");
        }
    }

    let mut rng = thread_rng();
    let mut choices: Vec<usize> = (0..names_len).collect();
    choices.shuffle(&mut rng);
    for num in 0..winner_count_num {
        println!("{}", names[choices[num]]);
    }
}
