#include<stdio.h>

int main() {
    int x,y;
    scanf("%d%d", &x, &y);
    if(x > y) {
        printf("Worse\n");
    } else {
        printf("Better\n");
    }
    return 0;
}