#include<stdio.h>
#include<ctype.h>
int main() {
  char c, d;
  scanf("%c %c", &c, &d);
  char t = tolower(c);
  if(t == d) printf("Yes\n");
  else printf("No\n");
  return 0;
}