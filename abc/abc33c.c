#include<stdio.h>
#include<string.h>
#include<stdbool.h>
#define MAX_LEN 100005
typedef struct {
  int vals[MAX_LEN];
  int size;
  bool zero;
} stack;
void init(stack *s) {
  s->size = 0;
  s->zero = false;
}
void push(stack *s, int p) {
  if(p == 0) s->zero = true;
  s->vals[s->size] = p;
  s->size++;
}
int main() {
  char s[MAX_LEN];
  memset(s, '\0', sizeof(s));
  scanf("%s", s);
  stack st;
  init(&st);
  int len = strlen(s);
  int ans = 0;
  for(int i = 0; i < len; i++) {
    if(s[i] == '+') {
      if(!st.zero) ans++;
      init(&st);
    } else if(s[i] == '*') {
      continue;
    } else {
      push(&st, s[i]-'0');
    }
  }
  if(!st.zero) ans++;
  printf("%d\n", ans);
  return 0;
}