#[allow(dead_code)]

fn print_bytes_and_chars(s: &String) {
    println!("All bytes are:");
    for i in s.as_bytes() {
        print!("{} ", i);
    }
    print!("\n");
    println!("All chars are:");
    for i in s.chars() {
        print!("{} ", i);
    }
    print!("\n");
}

pub fn main6() {
    let s = "Hello".to_string();
    println!("<sync 1> {}", s);
    let s = s + " " + "World!";
    println!("<sync 2> {}", s);
    let s = format!("{}, {}!", "Hello", "World");
    println!("<sync 3> {}", s);
    println!("<slice 0..3> {}", &s[0..3]);

    println!("Now test for the unicode string {}", "你好世界");
    let s = "你好世界".to_string();
    print_bytes_and_chars(&s);
}
