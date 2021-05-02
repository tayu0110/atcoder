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

    ll n;
    cin >> n;

    if(n==1){
        cout << 0 << endl;
        return 0;
    }

    ll ans=0,all=1,no0or9=1,no09=1;
    for(ll i=0;i<n;i++){
        all=all*10%MOD;
        no0or9=no0or9*9%MOD;
        no09=no09*8%MOD;
    }

    ans=(all-2*no0or9+no09)%MOD;

    if(ans<0)ans+=MOD;

    cout << ans << endl;

    return 0;
}
