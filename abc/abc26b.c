#include<stdio.h>
#define MAX_N 1005
#define PI 3.14159265358979

void sort(int *r, int lhs, int rhs) {
    // printf("lhs: %d, rhs: %d\n", lhs, rhs);
    if(rhs-lhs <= 1) return;
    int m = (lhs + rhs) / 2;
    int s = lhs, l = rhs-1;
    while(l-s > 0) {
        if(r[s] >= r[m] && r[l] <= r[m]) {
            int temp = r[s];
            r[s] = r[l];
            r[l] = temp;
            if(s == m){
                m = l;
                s++;
            }else if(l == m){
                m = s;
                l--;
            }else{
                s++;
                l--;
            }
        } else if(r[s] >= r[m]) {
            l--;
        } else if(r[l] <= r[m]) {
            s++;
        } else {
            s++;
            l--;
        }
    }
    // for(int i = lhs; i < rhs; i++) printf("%d ", r[i]);
    // printf("\n");
    sort(r, lhs, m);
    sort(r, m+1, rhs);
}

int main() {
    int n;
    scanf("%d", &n);
    int r[MAX_N];
    for(int i = 0; i < n; i++) {
        scanf("%d", r+i);
    }
    sort(r, 0, n);
    int red = (n-1)%2;
    int ans = 0;
    for(int i = 0; i < n; i++) {
        int k = r[i];
        // printf("i: %d, k: %d\n", i, k);
        if(i%2 == red) ans += k*k;
        else ans -= k*k;
    }
    printf("%.10lf\n", (double)ans*PI);
    return 0;
}
