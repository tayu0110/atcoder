#include<stdio.h>
#include<string.h>
#define MAX_LEN 15
int main() {
    char s[12][MAX_LEN];
    for(int i=0;i<12;i++){
        scanf("%s", s[i]);
    }
    int ans=0;
    for(int i=0;i<12;i++){
        int len=strlen(s[i]);
        for(int j=0;j<len;j++){
            if(s[i][j]=='r'){
                ans++;
                break;
            }
        }
    }
    printf("%d\n", ans);
    return 0;
}