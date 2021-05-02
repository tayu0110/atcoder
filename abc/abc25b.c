#include<stdio.h>
#include<string.h>
#define MAX_LEN 10
#define MAX_N 105
int main() {
    int n, a, b;
    scanf("%d%d%d", &n, &a, &b);
    int d[MAX_N];
    char s[MAX_N][MAX_LEN];
    for(int i = 0; i < n; i++) {
        scanf("%s%d", s[i], d+i);
    }
    int dist = 0;
    for(int i = 0; i < n; i++) {
        char* to = s[i];
        int w = d[i];
        if(w < a) w = a;
        else if(w > b) w = b;
        if(strcmp(to, "West") == 0) w = -w;
        dist += w;
    }
    if(dist > 0) printf("East %d\n", dist);
    else if(dist < 0) printf("West %d\n", -dist);
    else printf("0\n");
    return 0;
}