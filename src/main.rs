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

fn main() {
    let text: Vec<char> = "whaat".chars().collect();
    let mut reversed_text: Vec<&char> = text.iter().rev().collect();
    encrypt(reversed_text.as_mut_slice());
} 

fn index_of(letter: char) -> (usize, usize) {
    for row in 0..5 {
        let a= LATIN[row].iter().position(|&s| s == letter);
        match a {
            Some(column) => {
                print!("{} {} | ", row, column);
                return(row, column);
            }
            None => {}
        }
    }
    return(0, 0);
}

fn encrypt(text: &mut [&char]) {
    for x in 0..text.len() {
        let (row, column) = index_of(*text[x]);
        text[x] = &GALACTIC[row][(column+KEY)%6]
    }

    println!("\nEncrypted text:");
    for letter in text {
        print!("{}", letter)
    }
    println!()
}
