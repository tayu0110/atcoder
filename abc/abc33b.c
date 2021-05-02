#include<stdio.h>
#include<stdbool.h>
#define MAX_N 1005
int main() {
    int n;
    scanf("%d", &n);
    char s[MAX_N][25];
    int p[MAX_N];
    long long sum = 0;
    for(int i = 0; i < n; i++) {
        scanf("%s%d", s[i], p+i);
        sum += p[i];
    }
    char* ans;
    bool flag = false;
    for(int i = 0; i < n; i++) {
        int citizen = p[i];
        if(sum < 2*citizen) {
            flag = true;
            ans = s[i];
            break;
        }
    }
    if(flag) printf("%s\n", ans);
    else printf("atcoder\n");
    return 0;
}