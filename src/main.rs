use owo_colors::OwoColorize;
use rand;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path, env,
};

enum LetterState {
    NotInWord(u8),
    Misplaced(u8),
    Correct(u8),
}

struct Guess {
    letters: Vec<LetterState>,
}

impl Guess {
    fn new(text: &String, correct: &String) -> Self {
        let mut letters = Vec::new();
        let correct = correct.as_bytes();
        for (idx, c) in text.as_bytes().iter().enumerate() {
            if correct[idx] == *c {
                letters.push(LetterState::Correct(*c));
            } else if correct.contains(c) {
                letters.push(LetterState::Misplaced(*c));
            } else {
                letters.push(LetterState::NotInWord(*c));
            }
        }

        Guess { letters }
    }
}

fn read_words(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);

    buf.lines()
        .map(|l| l.expect("could not parse line"))
        .collect()
}

fn choose_random_word(words: &Vec<String>) -> &String {
    let x = rand::random::<usize>();
    return &words[x % words.len()];
}

fn print_board(board: &Vec<Guess>, round: &i32) {
    println!("{} -----", round);
    for line in board {
        for letter_guess in &line.letters {
            match letter_guess {
                LetterState::Correct(c) => {
                    let c = String::from(*c as char);
                    print!("{}", c.black().on_green());
                }
                LetterState::NotInWord(c) => {
                    let c = String::from(*c as char);
                    print!("{}", c);
                }
                LetterState::Misplaced(c) => {
                    let c = String::from(*c as char);
                    print!("{}", c.black().on_yellow());
                }
            }
        }
        println!();
    }
    println!("-----");
}

fn read_guess() -> String {
    let mut guess = String::new();
    std::io::stdin()
        .read_line(&mut guess)
        .expect("failed to read input (ㆆ_ㆆ)");
    return String::from(guess.trim());
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let words = read_words(filename);

    let word = choose_random_word(&words);

    let mut board = Vec::new();

    for round in 1..=6 {
        let guess = read_guess();

        if guess.eq(word) {
            println!("You won!");
            break;
        } else if guess.len() == 5 {
            board.push(Guess::new(&guess, &word));

            print_board(&board, &round);
        } else {
            println!("Words of size 5!");
        }
    }
    println!("You are a big looser - word was {}", word.red());
}
