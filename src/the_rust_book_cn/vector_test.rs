#[allow(dead_code)]
pub fn main5() {
    let mut v = vec![1, 2, 3, 4, 5];
    dbg!(&v);

    // create
    v.push(6);
    v.push(7);
    dbg!(&v);

    // read
    for i in &v {
        println!("{}", i);
    }
    if let Option::Some(i) = v.get(4) {
        println!("The 4th element is {}", i);
    }

    // update
    for i in &mut v {
        *i *= 20;
    }
    dbg!(&v);
}
