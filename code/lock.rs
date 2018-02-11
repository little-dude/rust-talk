fn sync_incr(mutex: &Mutex<int>) {
    // get a mutable reference to data
    let mut data = mutex.lock();
    *data += 1;
    // Destructor runs here, releasing lock
}
