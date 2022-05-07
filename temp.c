#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <stdbool.h>

typedef long long ll;
typedef long double ld;

int smaller(const void *a, const void *b) { return *(int *)a - *(int *)b; }
int greater(const void *a, const void *b) { return *(int *)b - *(int *)a; }
void sort(int *base, size_t size, int (*comp)(const void*, const void*)) { qsort(base, size, sizeof(*base), comp); }

int main(int argc, char **argv) {
  
  return 0;
}