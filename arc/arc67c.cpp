#include<iostream>
#include<iomanip>
#include<string>
#include<vector>
#include<algorithm>
#include<utility>
#include<tuple>
#include<map>
#include<queue>
#include<deque>
#include<set>
#include<stack>
#include<numeric>
#include<cstdio>
#include<cstdlib>
#include<cstring>
#include<cmath>

using namespace std;

using ll = long long;
using ld = long double;
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
    ll n;
    cin >> n;
    if(n==1){
        cout << 1 << endl;
        return 0;
    }
    vector<ll> d(n+1);
    for(int i=2;i<n+1;i++){
        int now=i;
        int div=2;
        while(now!=1){
            if(now%div!=0){
                div++;
                continue;
            }
            now/=div;
            d[div]++;
        }
    }
    ll ans=1;
    for(int i=2;i<n+1;i++){
        ans*=(d[i]+1);
        ans%=MOD;
    }
    cout << ans << endl;
    return 0;
}
