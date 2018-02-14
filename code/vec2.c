void vec_resize(Vec* vec) {
  int new_capacity = vec->capacity * 2;
  // invalid malloc
  int* new_data = (int*) malloc(new_capacity);
  assert(new_data != NULL);

  for (int i = 0; i < vec->length; ++i) {
    new_data[i] = vec->data[i];
  }

  // forgot to free previous data memory
  vec->data = new_data;
  vec->capacity = new_capacity;
}
