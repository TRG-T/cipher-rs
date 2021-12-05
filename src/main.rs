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
    let  reversed_text = text.iter().rev().collect::<Vec<_>>();
    encrypt(reversed_text.as_slice());

    print!("Reversed text: ");
    for letter in reversed_text {
        print!("{}", letter)
    }
    println!();

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

fn encrypt(text: &[&&str]) {
    for x in 0..4 {
        let (row, column) = index_of(text[x]);
    }
    println!()
}
