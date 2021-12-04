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

    let text = &mut ["d", "u", "p", "a"];
    let reversed_text = text.iter().rev();

    print!("Reversed text: ");
    for letter in reversed_text {
        print!("{}", letter)
    }
    println!();

    encrypt(reversed_text);
}

fn indexOf(letter: &str) {
    for i in 0..5 {
        let a= LATIN[i].iter().position(|&s| s == letter);
        print!("{:?}", a);
    }
}

fn encrypt(text: [&str; 4]) {
    for x in 0..4 {
        indexOf(text[x])
    }
}
