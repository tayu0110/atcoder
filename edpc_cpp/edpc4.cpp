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

    int n,w;
    cin >> n >> w;
    vector<pll> wv(n);
    for(auto &x:wv){
        cin >> x.first >> x.second;
    }

    vector<vector<ll>> dp(n, vector<ll>(w+1,0));
    for(int i=0;i<n;i++){
        for(int j=0;j<w+1;j++){
            if(i==0){
                if(j-wv[i].first>=0)dp[i][j] = wv[i].second;
                else continue;
            }else{
                if(j-wv[i].first>=0)dp[i][j] = max(dp[i-1][j-wv[i].first]+wv[i].second, dp[i-1][j]);
                else dp[i][j] = dp[i-1][j];
            }
            // cout << "i " << i << endl;
            // cout << "j " << j << endl;
            // cout << "dp[i][j] " << dp[i][j] << endl;
        }
    }
    ll ans = dp[n-1][w];
    cout << ans << endl;

    return 0;
}
