fn dump(s: String) {
    println!("{}", s);
}

fn main() {
    let s1 = "hello dolly".to_string();
    dump(s1);
    println!("s1 {}", s1); // <---error: 'value used here after move'
}