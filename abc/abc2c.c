#include<stdio.h>
#include<math.h>
int main() {
  int xa, ya, xb, yb, xc, yc;
  scanf("%d%d%d%d%d%d", &xa, &ya, &xb, &yb, &xc, &yc);
  double s =  abs((xa-xc)*(yb-yc)-(xb-xc)*(ya-yc));
  printf("%.10lf\n", s/2);
  return 0;
}