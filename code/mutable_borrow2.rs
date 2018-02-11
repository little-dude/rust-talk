fn main() {
    let vec = Vec::new();
    vec.push(1);
    vec.push(2);

    let elem = &vec[0];
    borrow_and_mutate(&mut vec); // err: already a shared ref to vec

    println!("elem = {}", elem); // elem might be invalid
}

fn borrow_and_mutate(vec: &mut Vec) {
    // mutate vec
}
