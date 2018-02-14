void example() {
    vector<string> vector;
    // ...

    auto& elem = vector[0];

    // mutation + aliasing
    vector.push_back(some_string);

    cout << elem;
}
