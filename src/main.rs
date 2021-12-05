const LATIN: [[&str; 6]; 5] = [
    ["a", "b", "c", "d", "e", "f"],
    ["g", "h", "i", "j", "k", "l"],
    ["m", "n", "o", "p", "q", "r"],
    ["s", "t", "u", "v", "w", "x"],
    ["y", "z", "!", "?", ":", " "]
];

const GALACTIC: [[&str; 6]; 5] = [
    ["ᔑ", "ʖ", "ᓵ", "↸", "Ŀ", "⎓"],
    ["ㅓ", "〒", "╎", "፧", "ꖌ", "ꖎ"],
    ["ᒲ", "リ", "フ", "¡", "ᑑ", "።"],
    ["ነ", "ﬧ", "⚍", "⍊", "∴", "/"],
    ["॥", "Λ", "ʗ", "˨", "ᚴ", "ᚌ"]
];

fn main() {
    let text = ["d", "u", "p", "a"];
    let mut reversed_text = text.iter().rev().collect::<Vec<_>>();
    encrypt(reversed_text.as_mut_slice());

}

fn index_of(letter: &str) -> (usize, usize) {
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

fn encrypt(text: &mut [&&str]) {
    for x in 0..4 {
        let (row, column) = index_of(text[x]);

        if column+3 >= 6 {
            text[x] = &GALACTIC[row][(column+3)%6]
        } else {
            text[x] = &GALACTIC[row][column+3]
        }
    }

    println!("\nEncrypted text:");
    for letter in text {
        print!("{}", letter)
    }
    println!()
}
