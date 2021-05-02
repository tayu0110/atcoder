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
    int n;
    cin >> n;
    vector<int> p(n);
    int w=0;
    for(int i=0;i<n;i++){
        cin >> p[i];
        w+=p[i];
    }
    vector<bool> dp(w+1,false);
    dp[0]=true;
    for(int i=0;i<n;i++){
        for(int j=w;j>=0;j--){
            // if(dp[j]) continue;
            if(j-p[i]>=0 && dp[j-p[i]]) dp[j]=dp[j-p[i]];
            // cout << "j: " << j << " dp[j]: " << dp[j] << endl;
        }
    }
    int ans=0;
    // for(int i=0;i<w+1;i++) cout << dp[i] << endl;
    for(int i=0;i<w+1;i++) if(dp[i]) ans++;
    cout << ans << endl;
    return 0;
}
