use rand::{seq::SliceRandom, thread_rng};
use std::io;

fn main() {
    let mut names = Vec::<String>::new();

    println!("Draw random names from a hat!");
    println!("Please type in one name at a time, then hit enter to add the next name.");
    println!("Enter \"DONE\" when complete!");

    loop {
        let mut name = String::new();

        io::stdin()
            .read_line(&mut name)
            .expect("Failed to read line");

        match name.as_str().trim() {
            "DONE" => break,
            _ => {
                names.push(String::from(name.trim()));
            }
        }
    }

    let mut winner_count_str = String::new();
    let mut winner_count_num: usize = 0;
    while winner_count_num == 0 || winner_count_num > names.len() {
        println!("How many names would you like to draw?");

        io::stdin()
            .read_line(&mut winner_count_str)
            .expect("Failed to read line");
        winner_count_num = match winner_count_str.trim().parse() {
            Ok(num) => {
                if num > names.len() {
                    winner_count_str.clear();
                    println!("Cant have more winners than names!");
                }
                num
            }
            Err(_) => {
                winner_count_str.clear();
                println!("Please input a non-zero positive number");
                0
            }
        };
    }

    let mut rng = thread_rng();
    let mut choices: Vec<usize> = (0..names.len()).collect();
    choices.shuffle(&mut rng);
    for num in 0..winner_count_num {
        println!("{}", names[choices[num]]);
    }
}
