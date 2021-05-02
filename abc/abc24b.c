#include<stdio.h>
#define MAX_N 100005
int main() {
    int n, t;
    scanf("%d%d", &n, &t);
    int a[MAX_N];
    for(int i = 0; i < n; i++) scanf("%d", a + i);
    int ans = 0;
    for(int i = 0; i < n-1; i++) {
        if(a[i+1] - a[i] <= t) ans += a[i+1] - a[i];
        else ans += t;
    }
    printf("%d\n", ans + t);
    return 0;
}