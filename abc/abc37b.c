#include<stdio.h>
#include<string.h>

#define MAX_N 105

int main() {
    int n, q;
    scanf("%d%d", &n, &q);
    long long a[MAX_N];
    memset(a, 0, sizeof(a));
    for(int i = 0; i < q; i++) {
        int l, r;
        long long t;
        scanf("%d%d%lld", &l, &r, &t);
        for(; l <= r; l++) {
            a[l] = t;
        }
    }
    for(int i = 1; i < n+1; i++) {
        printf("%d\n", a[i]);
    }
    return 0;
}