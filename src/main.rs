use rand::{seq::SliceRandom, thread_rng};
use std::io;
use to_int_and_back::to;

fn main() {
    let mut names = Vec::<String>::new();

    println!("\nDraws one or more random names from a list!");
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
    let names_len = names.len() as isize;

    let mut winner_count_str = String::new();
    let mut winner_count_num: isize = 0;
    while winner_count_num <= 0 || winner_count_num > names_len {
        println!("\nHow many names would you like to draw?");

        io::stdin()
            .read_line(&mut winner_count_str)
            .expect("Failed to read line");
        if winner_count_str.trim().to_lowercase().eq("and") {
            colour::red_ln!("\nInvalid Input");
            winner_count_str.clear();
            continue;
        }
        winner_count_num = match winner_count_str.trim().parse() {
            Ok(num) => num,
            Err(_) => match to::int(&winner_count_str.trim()) {
                Ok(num) => num,
                Err(e) => {
                    if e.trim().contains("and") {
                        colour::red_ln!("\nInvalid Input");
                    } else {
                        colour::red_ln!("{}", e);
                    }
                    winner_count_str.clear();
                    continue;
                }
            },
        };
        if winner_count_num <= 0 || winner_count_num > names_len {
            winner_count_str.clear();
            colour::red_ln!("\nInvalid Input");
            colour::red_ln!(
                "{}",
                format!(
                    "Can not draw {} names from {} names",
                    winner_count_num, names_len
                )
            );
            winner_count_str.clear();
        }
    }

    let mut rng = thread_rng();
    let mut choices: Vec<usize> = (0..names_len as usize).collect();
    choices.shuffle(&mut rng);
    println!("");
    for num in 0..winner_count_num {
        colour::cyan_ln!("{}", names[choices[num as usize]]);
    }
    println!("");
}
