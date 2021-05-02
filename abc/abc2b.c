#include<stdio.h>
#include<string.h>
#define MAX_LEN 35
int main() {
  char w[MAX_LEN];
  scanf("%s", w);
  int len = strlen(w);
  for(int i = 0; i < len; i++) {
    if(w[i] == 'a' || w[i] == 'i' || w[i] == 'u' || w[i] == 'e' || w[i] == 'o') continue;
    printf("%c", w[i]);
  }
  printf("\n");
  return 0;
}