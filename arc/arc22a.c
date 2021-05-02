#include<stdio.h>
#include<ctype.h>
#include<string.h>
#define MAX_LEN 105
int main() {
  char s[MAX_LEN];
  scanf("%s", s);
  int n = strlen(s);
  char c = 'I';
  for(int i = 0; i < n; i++) {
    s[i] = toupper(s[i]);
    if(s[i] == c) {
      if(c == 'I') c = 'C';
      else if(c == 'C') c = 'T';
      else if(c == 'T') c = 'A';
    }
  }
  if(c == 'A') printf("YES\n");
  else printf("NO\n");
  return 0;
}