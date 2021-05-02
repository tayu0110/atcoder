#include<stdio.h>

int main() {
    int a, b;
    scanf("%d%d", &a, &b);
    int e = 10;
    while(b/e != 0) e *= 10;
    printf("%d\n", 2 * (a*e + b));
    return 0;
}