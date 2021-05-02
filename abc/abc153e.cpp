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

ll dp[20005];

int main(int argc,char* argv[]){
    cin.tie(0);
    ios::sync_with_stdio(0);
    cout << fixed << setprecision(20);
    int h,n;
    cin >> h >> n;
    vector<pll> ab(n);
    ll maxval=0;
    for(int i=0;i<n;i++){
        cin >> ab[i].first >> ab[i].second;
        maxval=max(maxval, ab[i].first);
    }
    dp[0]=0;
    for(int i=1;i<=h+maxval;i++){
        dp[i]=INF;
    }
    for(int i=1;i<=h+maxval;i++){
        for(int j=0;j<n;j++){
            ll a=ab[j].first, b=ab[j].second;
            if(i-a<0)continue;
            dp[i]=min(dp[i], dp[i-a]+b);
        }
    }
    // for(int i=0;i<=h+maxval;i++){
    //     cout << "i: " << i << " dp[i]: " << dp[i] << endl;
    // }
    ll ans=INF;
    for(int i=h;i<h+maxval;i++){
        ans=min(ans, dp[i]);
    }
    cout << ans << endl;
    return 0;
}
