fn main() {
    let vec = Vec::new();
    vec.push(1);
    vec.push(2);

    take(vec);
}

fn take(vec: Vec) {
    // when this function returns, vec is deallocated
}
