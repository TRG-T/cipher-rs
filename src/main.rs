use console::Term;
use dialoguer::{theme::ColorfulTheme, Select};
use std::io;

const LATIN: [[char; 6]; 5] = [
    ['a', 'b', 'c', 'd', 'e', 'f'],
    ['g', 'h', 'i', 'j', 'k', 'l'],
    ['m', 'n', 'o', 'p', 'q', 'r'],
    ['s', 't', 'u', 'v', 'w', 'x'],
    ['y', 'z', '!', '?', ':', ' '],
];

const GALACTIC: [[char; 6]; 5] = [
    ['ᔑ', 'ʖ', 'ᓵ', '↸', 'Ŀ', '⎓'],
    ['ㅓ', '〒', '╎', '፧', 'ꖌ', 'ꖎ'],
    ['ᒲ', 'リ', 'フ', '¡', 'ᑑ', '።'],
    ['ነ', 'ﬧ', '⚍', '⍊', '∴', '/'],
    ['॥', 'Λ', 'ʗ', '˨', 'ᚴ', 'ᚌ'],
];
const KEY: usize = 3;

fn main() -> std::io::Result<()> {
    let selection = Select::with_theme(&ColorfulTheme::default())
        .item("Encrypt")
        .item("Decrypt")
        .default(0)
        .interact_on(&Term::stderr())?;

    println!("Enter the text");
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read input");
    let text = user_input.trim();

    match selection {
        0 => {
            let encrypted_text = encrypt(text);
            println!("\nEncrypted text: {}", encrypted_text);
        }
        1 => {
            let decrypted_text = decrypt(text);
            println!("\nDecrypted text: {}", decrypted_text);
        }
        _ => {
            println!("Something went wrong")
        }
    }
    Ok(())
}

fn index_of(letter: char, choice: usize) -> (usize, usize) {
    [LATIN, GALACTIC]
        .get(choice)
        .and_then(|map| {
            map.iter().enumerate().find_map(|(row, row_slice)| {
                let col = row_slice.iter().position(|&s| s == letter)?;
                print!("{} {} | ", row, col);
                Some((row, col))
            })
        })
        .unwrap_or((0, 0))
}

fn encrypt(text: &str) -> String {
    text.chars()
        .rev()
        .map(|letter| {
            let (row, col) = index_of(letter, 0);
            GALACTIC[row][(col + KEY) % 6]
        })
        .collect()
}

fn decrypt(text: &str) -> String {
    text.chars()
        .rev()
        .map(|letter| {
            let (row, col) = index_of(letter, 1);
            LATIN[row][(col + KEY) % 6]
        })
        .collect()
}
