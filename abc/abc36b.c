#include<stdio.h>

#define MAX_N 55

int main() {
    int n;
    char s[MAX_N][MAX_N];
    scanf("%d\n", &n);
    for(int i = 0; i < n; i++) {
        scanf("%s", s[i]);
    }
    for(int i = 0; i < n; i++) {
        for(int j = n-1; j >= 0; j--) {
            printf("%c", s[j][i]);
        }
        printf("\n");
    }
    return 0;
}