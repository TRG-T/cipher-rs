use array2d::Array2D;

fn main() {
    let latin_rows = vec![
        vec!["a", "b", "c", "d", "e", "f"],
        vec!["g", "h", "i", "j", "k", "l"],
        vec!["m", "n", "o", "p", "q", "r"],
        vec!["s", "t", "u", "v", "w", "x"],
        vec!["y", "z", "!", "?", ":", " "]
    ];
    let latin = Array2D::from_rows(&latin_rows);

    let galactic_rows = vec![
        vec!["ᔑ", "ʖ", "ᓵ", "↸", "Ŀ", "⎓"],
        vec!["ㅓ", "〒", "╎", "፧", "ꖌ", "ꖎ"],
        vec!["ᒲ", "リ", "フ", "¡", "ᑑ", "።"],
        vec!["ነ", "ﬧ", "⚍", "⍊", "∴", "/"],
        vec!["॥", "Λ", "ʗ", "˨", "ᚴ", "ᚌ"]
    ];
    let galactic = Array2D::from_rows(&galactic_rows);


    
}
