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
            println!("\nEncrypted text: {}", encrypted_text);
        }
        1 => {
            let decrypted_text = decrypt(text.as_mut_slice());
            println!("\nDecrypted text: {}", decrypted_text);
        }
        _ => {
            println!("Something went wrong")
        }
    }
    Ok(())
} 

fn index_of(letter: char, choice: u8) -> (usize, usize) {
    for row in 0..5 {
        let col: Option<usize>;
        match choice {
            0 => {
                col = LATIN[row].iter().position(|s| s == &letter);
            }
            1 => {
                col = GALACTIC[row].iter().position(|s| s == &letter);
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

fn encrypt(text: &mut [char]) -> String {
    let reversed_text: String = text.iter().rev().collect();
    let mut encrypted_text = String::from("");
    for letter in reversed_text.chars() {
        let (row, column) = index_of(letter, 0);
         encrypted_text.push(GALACTIC[row][(column+KEY)%6])
    }
    return encrypted_text;
}

fn decrypt(text: &mut [char]) -> String {
    let reversed_text: String = text.iter().rev().collect();
    let mut decrypted_text = String::from("");
    for letter in reversed_text.chars() {
        let (row, column) = index_of(letter, 1);
        decrypted_text.push(LATIN[row][(column+KEY)%6])
    }
    return decrypted_text;
}
