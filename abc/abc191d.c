#include <stdio.h>
#include <math.h>

const long long g = 10000;

long long absl(long long val) { return val < 0 ? -val : val; }

long long search(long long y, long long o, long long r) {
  long long c = r * r - absl(y - o) * absl(y - o);
  long long ac = 0, wa = r+1;
  while(wa-ac > 1) {
    long long wj = (wa+ac) / 2;
    if(wj * wj <= c) ac = wj;
    else wa = wj;
  }
  return ac;
}

int main() {
  double x, y, r;
  scanf("%lf %lf %lf", &x, &y, &r);
  long long a = (long long)round(x * g), b = (long long)round(y * g), p = (long long)round(r * g);
  long long top = b + p;
  long long bottom = b - p;
  if(top >= 0) top = top / g * g;
  else top = (top-g+1) / g * g;
  long long ans = 0;
  while(top >= bottom) {
    long long w = search(top, b, p);
    long long right = a + w;
    long long left = a - w;
    if(right >= 0) right = right / g;
    else right = (right-g+1) / g;
    if(left >= 0) left = (left+g-1) / g;
    else left = left / g;
    ans += right - left + 1;
    top -= g;
  }
  printf("%lld\n", ans);
  return 0;
}