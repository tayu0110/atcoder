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
    ll n,k;
    cin >> n >> k;
    ll ans=0;
    for(ll b=1;b<=n;b++){
        ll p=n/b,r=n%b;
        ans+=max(0LL,b-k)*p+max(0LL,r-k+1);
        if(k==0)ans--;
    }
    // for(ll b=k+1;b<=n;b++){
    //     if(b+k>n){
    //         ans+=b-k;
    //         continue;
    //     }
    //     for(ll i=k;i<b;i++){
    //         if(n-i<b){
    //             ans+=b-i;
    //             break;
    //         }
    //         if(i)ans++;
    //         ans+=(n-i)/b;
    //     }
    // }
    cout << ans << endl;
    return 0;
}
