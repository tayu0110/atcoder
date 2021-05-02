#include<stdio.h>
#include<string.h>
#include<stdbool.h>
#define MAX_LEN 15
int main() {
  char s[MAX_LEN], t[MAX_LEN];
  scanf("%s%s", s, t);
  int len = strlen(s);
  bool flag = true;
  for(int i = 0; i < len; i++) {
    if(s[i] == t[i]) continue;
    else if(s[i] == '@' && (t[i] == 'a' || t[i] == 't' || t[i] == 'c' || t[i] == 'o' || t[i] == 'd' || t[i] == 'e' || t[i] == 'r')) continue;
    else if(t[i] == '@' && (s[i] == 'a' || s[i] == 't' || s[i] == 'c' || s[i] == 'o' || s[i] == 'd' || s[i] == 'e' || s[i] == 'r')) continue;
    else flag = false;
  }
  if(flag) printf("You can win\n");
  else printf("You will lose\n");
  return 0;
}