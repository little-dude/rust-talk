void example() {
    vector<string> vector;
    // ...

    // aliasing
    auto& elem = vector[0];

    // mutation
    vector.push_back(some_string);

    // oops
    cout << elem;
}
