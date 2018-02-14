fn main() {
    let orig = Box::new(5);
    println!("{}", *orig);
    let _stolen = orig;
    println!("{}", *orig);
}
