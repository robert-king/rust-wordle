use std::io;

mod word_list;


fn get_line_from_stdin() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    let lower = buffer.to_lowercase();
    let trimmed = lower.trim();
    Ok(trimmed.to_string())
}

fn print_with_tabs(s: &str) {
    print!("{}", s.chars().next().unwrap());
    for c in s.chars().skip(1) {
        print!("\t{}", c);
    }
    println!("");
}

fn main() {
    println!("Welcome to Wordle");
    println!("Please visit https://www.powerlanguage.co.uk/wordle/ to play officially");
    let words = word_list::WordList::new();
    let solution = words.get_random_common_word();
    assert!(words.is_valid(&solution));
    let mut attempts = 0;
    println!("You have six guesses, please enter a five letter word:");
    while attempts < 6 {
        let guess = get_line_from_stdin().expect("Couldn't read from Stdin");
        if !words.is_valid(&guess) {
            println!("invalid guess: {}", guess);
            continue;
        }
        attempts += 1;
        let hint = get_hint(&guess, &solution);
        print_with_tabs(&guess);
        print_with_tabs(&hint);
        if guess == solution {
            println!("Congrats, you win!");
            return;
        }
    }
    println!("game over, the answer was {}", solution);
}

fn get_hint(guess: &str, sol: &str) -> String {
    let green_square = '🟩';
    let black_square = '⬛';
    let yellow_square = '🟨';
    let sol_bytes = sol.as_bytes();
    let mut hint = String::new();
    for (i, b) in guess.bytes().enumerate() {
        let c = if b == sol_bytes[i] {
            green_square
        } else if sol_bytes.contains(&b) {
            yellow_square
        } else {
            black_square
        };
        hint.push(c);
    }
    hint
}