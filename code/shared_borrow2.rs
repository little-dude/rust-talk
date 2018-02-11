fn main() {
    let vec = Vec::new();
    vec.push(1);
    vec.push(2);

    borrow_and_mutate(&mut vec);

    // we can still use the vec
    println!("vec = {}", vec);
}

fn borrow_and_mutate(vec: &mut Vec) {
    // mutate vec it
}
