fn main() {
    let vec = Vec::new();
    vec.push(1);
    vec.push(2);

    let elem = &vec[0];
    borrow(&vec); // ok

    println!("elem = {}", elem); // &elem is guaranteed to be valid
}

fn borrow(vec: &Vec) {
    // use vec but cannot mutate it
}
