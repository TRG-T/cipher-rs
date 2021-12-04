use array2d::Array2D;

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

    println!("Latin: ");
    for row in LATIN {
        for element in row {
            print!("{} ", element);
        }
        println!();
    }

    println!("\nGalactic: ");
    for row in GALACTIC {
        for element in row {
            print!("{} ", element);
        }
        println!();
    }

    
}
