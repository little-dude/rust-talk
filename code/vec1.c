typedef struct {
  int* data;
  int  length;
  int  capacity;
} Vec;

Vec* vec_new() {
  Vec vec;
  vec.data = NULL;
  vec.length = 0;
  vec.capacity = 1;
  // pointer to stack
  // allocated memory
  return &vec;
}

void vec_push(Vec* vec, int n) {
  if (vec->length == vec->capacity) {
    vec_resize(vec);
  }

  vec->data[vec->length] = n;
  ++vec->length;
}



void vec_free(Vec* vec) {
  free(vec);
  // freeing invalid memory
  free(vec->data);
}
