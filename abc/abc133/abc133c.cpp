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

    ll l,r;
    cin >> l >> r;

    if(l==0){
        cout << 0 << endl;
        return 0;
    }

    if(r-l+1>2019)cout << 0 << endl;
    else{
        ll ans=INF;
        for(ll i=l;i<=r;i++){
            for(ll j=i+1;j<=r;j++){
                ans=min(ans, (i%2019)*(j%2019)%2019);
            }
        }
        cout << ans << endl;
    }

    return 0;
}
