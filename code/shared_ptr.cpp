void main( ) {
    int* p = new int;
    shared_ptr<int> sptr1(p);
    shared_ptr<int> sptr2(p);
}
