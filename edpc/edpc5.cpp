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

    int n;
    ll W;
    cin >> n >> W;

    vector<ll> w(n+1),v(n+1);
    for(int i=0;i<n;i++){
        cin >> w[i+1] >> v[i+1];
    }

    vector<vector<ll>> dp(n+1, vector<ll>(100010,INF));
    dp[0][0]=0;
    for(int i=1;i<n+1;i++){
        for(int j=0;j<100010;j++){
            if(j-v[i]>=0)dp[i][j] = min(dp[i-1].at(j-v[i])+w[i], dp[i-1][j]);
            else dp[i][j]=dp[i-1][j];
            // cout << "i " << i << " j "<< j << " dp[i][j] " << dp[i][j] << endl;
        }
    }
    ll ans=100000;
    while(dp[n][ans]>W)ans--;

    cout << ans << endl;

    return 0;
}
