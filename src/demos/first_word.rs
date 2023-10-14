fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    s
}

pub fn main2() {
    let a = first_word("a b c");
    let b = first_word("awdfawefawef");
    println!("{a}");
    println!("{b}");
}
