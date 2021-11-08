#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <stdbool.h>

typedef long long ll;
typedef long double ld;
int num[20];
int top;
void up(int d) {
  if(d < top) top--;
  num[d]++;
  if(d > top && num[d] - num[d-1] > 1) up(d-1);
  else if(num[d] > 9) up(d-1);
  else {
    for(int i=d+1;i<20;i++) {
      num[i] = (num[i-1]-1 >= 0 ? num[i-1]-1 : 0);
    }
  }
}
void add(void) {
  num[19]++;
  if(num[19] - num[18] > 1) up(18);
  else if(num[19] > 9) up(18);
}
int main(int argc, char **argv) {
  int k;
  scanf("%d", &k);
  if(k < 10) {
    printf("%d\n", k);
    return 0;
  }
  memset(num, 0, sizeof(num));
  top = 18;
  k -= 10;
  num[18] = 1;
  for(int i=0;i<k;i++) add();
  int cnt = 0;
  while(!num[cnt]) cnt++;
  for(int i=cnt;i<20;i++) printf("%d", num[i]);
  printf("\n");
  return 0;
}