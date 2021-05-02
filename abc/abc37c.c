#include<stdio.h>

#define MAX_N 100005

int main() {
    int n, k;
    scanf("%d%d", &n, &k);
    long long a[MAX_N];
    long long sum[MAX_N];
    sum[0] = 0;
    for(int i = 0; i < n; i++) {
        scanf("%lld", (a + i));
        sum[i+1] = sum[i] + a[i];
    }
    long long ans = 0;
    for(int i = k; i < n+1; i++) {
        ans += sum[i] - sum[i-k];
        // printf("ans: %lld\n", ans);
    }
    printf("%lld\n", ans);
    return 0;
}