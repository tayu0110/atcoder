#include<stdio.h>

int main() {
    int a, b, c, d;
    scanf("%d%d%d%d", &a, &b, &c, &d);
    b *= c;
    d *= a;
    if(b > d) printf("TAKAHASHI\n");
    else if(b < d) printf("AOKI\n");
    else printf("DRAW\n");
    return 0;
}