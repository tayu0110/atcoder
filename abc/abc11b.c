#include<stdio.h>
#include<string.h>
#include<ctype.h>
#define MAX_LEN 15
int main() {
  char s[MAX_LEN];
  int len;
  scanf("%s", s);
  len = strlen(s);
  s[0] = toupper(s[0]);
  for(int i = 1; i < len; i++) s[i] = tolower(s[i]);
  printf("%s\n", s);
  return 0;
}