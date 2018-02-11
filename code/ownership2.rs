fn main() {
    let vec = Vec::new();
    vec.push(1);
    vec.push(2);

    take(vec); // move vec

    println!("vec = {}", vec); // err: vec does not exist anymore
}

fn take(vec: Vec) {
    // when this function returns, vec is deallocated
}
