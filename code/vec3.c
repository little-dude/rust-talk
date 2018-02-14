int main() {
  Vec* vec = vec_new();
  vec_push(vec, 107);

  int* n = &vec->data[0];
  vec_push(vec, 110);
  // access to invalid memory
  printf("%d\n", *n);

  // freeing memory freed by vec_free
  free(vec->data);
  vec_free(vec);
  return 0;
}
