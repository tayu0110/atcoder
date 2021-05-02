#include<stdio.h>
#include<string.h>
#include<stdbool.h>
#define MAX_N 105
int main() {
    int n, a, b, k;
    scanf("%d%d%d%d", &n, &a, &b, &k);
    int p[MAX_N];
    bool ck[MAX_N];
    memset(p, 0, sizeof(p));
    memset(ck, false, sizeof(ck));
    ck[a]=true;
    ck[b]=true;
    for(int i = 0; i < k; i++) {
        scanf("%d", p + i);
    }
    for(int i = 0; i < k; i++) {
        int q = p[i];
        if(ck[q]) {
            printf("NO\n");
            return 0;
        }
        ck[q] = true;
    }
    printf("YES\n");
    return 0;
}