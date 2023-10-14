use std::io::Write;

#[allow(dead_code)]
pub fn main4() {
    use std::io;

    let mut s = String::new();
    print!("Please enter a string: ");
    io::stdout().flush().expect("");
    let r = io::stdin().read_line(&mut s);
    if let Result::Ok(size) = r {
        assert_eq!(size, s.len());
        println!("Test succeeded!");
    }
}
