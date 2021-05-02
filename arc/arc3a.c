#include<stdio.h>
#define MAX_N 105
int main() {
  int n;
  scanf("%d", &n);
  char r[MAX_N];
  scanf("%s", r);
  double gpa = 0;
  for(int i = 0; i < n; i++) {
    char c = r[i];
    if(c == 'A') gpa += 4;
    else if(c == 'B') gpa += 3;
    else if(c == 'C') gpa += 2;
    else if(c == 'D') gpa += 1;
  }
  printf("%.15lf\n", gpa / n);
  return 0;
}