#include<stdio.h>

int main() {
    char s[6];
    scanf("%s", s);
    int n;
    scanf("%d", &n);
    int a;
    printf("%c%c\n", s[(n-1)/5], s[(n-1)%5]);
    return 0;
}