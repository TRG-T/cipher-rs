use std::io;
use dialoguer::{ Select, theme::ColorfulTheme };
use console::Term;

const LATIN: [[char; 6]; 5] = [
    ['a', 'b', 'c', 'd', 'e', 'f'],
    ['g', 'h', 'i', 'j', 'k', 'l'],
    ['m', 'n', 'o', 'p', 'q', 'r'],
    ['s', 't', 'u', 'v', 'w', 'x'],
    ['y', 'z', '!', '?', ':', ' ']
];

const GALACTIC: [[char; 6]; 5] = [
    ['ᔑ', 'ʖ', 'ᓵ', '↸', 'Ŀ', '⎓'],
    ['ㅓ', '〒', '╎', '፧', 'ꖌ', 'ꖎ'],
    ['ᒲ', 'リ', 'フ', '¡', 'ᑑ', '።'],
    ['ነ', 'ﬧ', '⚍', '⍊', '∴', '/'],
    ['॥', 'Λ', 'ʗ', '˨', 'ᚴ', 'ᚌ']
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
    let mut text = user_input.trim().chars().collect::<Vec<_>>();

    match selection {
        0 => {
            let encrypted_text = encrypt(text.as_mut_slice());
            println!("\nEncrypted text:");
            for letter in encrypted_text {
                print!("{}", letter)
            }
            println!();
        }
        1 => {
            let decrypted_text = decrypt(text.as_mut_slice());
            println!("\nDecrypted text:");
            for letter in decrypted_text {
                print!("{}", letter)
            }
            println!();
        }
        _ => {
            println!("Something went wrong")
        }
    }
    Ok(())
} 

fn index_of(letter: &char, choice: u8) -> (usize, usize) {
    for row in 0..5 {
        let col: Option<usize>;
        match choice {
            0 => {
                col = LATIN[row].iter().position(|s| s == letter);
            }
            1 => {
                col = GALACTIC[row].iter().position(|s| s == letter);
            }
            _ => { col = None }
        }
        match col {
            Some(column) => {
                print!("{} {} | ", row, column);
                return(row, column);
            }
            None => {}
        }
    }
    return(0, 0);
}

fn encrypt(text: &mut [char]) -> Vec<&char> {
    let mut reversed_text: Vec<&char> = text.iter().rev().collect();
        for x in 0..text.len() {
        let (row, column) = index_of(reversed_text[x], 0);
        reversed_text[x] = &GALACTIC[row][(column+KEY)%6]
    }
    return reversed_text;
}

fn decrypt(text: &mut [char]) -> Vec<&char> {
    let mut reversed_text: Vec<&char> = text.iter().rev().collect();
    for x in 0..text.len() {
        let (row, column) = index_of(reversed_text[x], 1);
        reversed_text[x] = &LATIN[row][(column-KEY)%6]
    }
    return reversed_text;
}
