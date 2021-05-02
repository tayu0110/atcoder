#include<stdio.h>

int main() {
    int l, h, n;
    scanf("%d%d", &l, &h);
    scanf("%d", &n);
    for(int i = 0; i < n; i++) {
        int a;
        scanf("%d", &a);
        if(a >= l && a <= h) {
            printf("0\n");
        } else if(a < l) {
            printf("%d\n", l-a);
        } else if(a > h) {
            printf("-1\n");
        }
    }
    return 0;
}