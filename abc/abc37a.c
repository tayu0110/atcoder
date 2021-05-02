#include<stdio.h>

int main() {
    int a, b, c;
    scanf("%d%d%d", &a, &b, &c);
    if(a < b) {
        printf("%d\n", c / a);
    } else {
        printf("%d\n", c / b);
    }
    return 0;
}