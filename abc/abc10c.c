#include<stdio.h>
#include<math.h>
#include<stdbool.h>
int main() {
  int a[7];
  for(int i = 0; i < 7; i++) scanf("%d", a + i);
  double txa = a[0];
  double tya = a[1];
  double txb = a[2];
  double tyb = a[3];
  int t = a[4];
  int v = a[5];
  int n = a[6];
  bool flag = false;
  for(int i = 0; i < n; i++) {
    double x, y;
    scanf("%lf%lf", &x, &y);
    double a = (txa-x)*(txa-x)+(tya-y)*(tya-y);
    double b = (x-txb)*(x-txb)+(y-tyb)*(y-tyb);
    double l = sqrt(a)+sqrt(b);
    l = ceil(l);
    // printf("%lf %lf %lf %lf\n", txa, tya, txb, tyb);
    // printf("%lf %lf\n", x, y);
    // printf("%lf %lf %lf\n", a, b, l);
    if(t*v < l) continue;
    flag = true;
  }
  if(flag) printf("YES\n");
  else printf("NO\n");
  return 0;
}