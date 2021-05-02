#include<stdio.h>
#include<string.h>
#define MAX_LEN 105
int main() {
    char s[MAX_LEN];
    scanf("%s", s);
    int a = 0, b = 0, c = 0, d = 0, e = 0, f = 0;
    int len = strlen(s);
    for(int i = 0; i < len; i++) {
        if(s[i] == 'A') a++;
        else if(s[i] == 'B') b++;
        else if(s[i] == 'C') c++;
        else if(s[i] == 'D') d++;
        else if(s[i] == 'E') e++;
        else if(s[i] == 'F') f++;
    }
    printf("%d %d %d %d %d %d\n", a, b, c, d, e, f);
    return 0;
}