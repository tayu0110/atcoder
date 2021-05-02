#include<iostream>
#include<iomanip>
#include<string>
#include<vector>
#include<algorithm>
#include<utility>
#include<tuple>
#include<map>
#include<queue>
#include<set>
#include<stack>
#include<numeric>
#include<cstdio>
#include<cstdlib>
#include<cstring>
#include<cmath>

using namespace std;

using ll = long long;
using pii = pair<int, int>;
using pll = pair<ll, ll>;

#define BIL ((ll)1e9)
#define MOD ((ll)1e9+7)
#define INF (1LL<<60)           //1LL<<63でオーバーフロー
#define inf (1<<29)             //1<<29でオーバーフロー

int main(int argc,char* argv[]){
    cin.tie(0);
    ios::sync_with_stdio(0);
    cout << fixed << setprecision(20);

    double n,k;
    cin >> n >> k;

    double p;
    if(n>k){
        p=((n-k+1)/n);
        for(int i=1;i<k;i++){
            int now=i;
            double nowp=1/n;
            while(now<k){
                now*=2;
                nowp*=0.500000000;
            }
            p+=nowp;
        }
    }
    else{
        p=0;
        for(int i=1;i<n+1;i++){
            int now=i;
            double nowp=1/n;
            while(now<k){
                now*=2;
                nowp*=0.500000000;
            }
            p+=nowp;
        }
    }
    

    cout << p << endl;

    return 0;
}
