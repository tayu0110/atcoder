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
    // Give up
    // int ans=n;
    // for(int i=0; i<n+1;i++){
    //     int cc=0;
    //     int t=i;
    //     while(t>0)cc+=t%6,t/=6;
    //     t=n-i;
    //     while(t>0)cc+=t%9,t/=9;
    //     if (ans>cc)ans=cc;
    // }
    // cout << ans << endl;
    vector<ll> dp(n+1, INF);
    dp[0]=0;
    for(int i=1;i<n+1;i++){
        int pow=1;
        while(pow<=i){
            dp[i]=min(dp[i], dp[i-pow]+1);
            pow*=6;
        }
        int pow9=1;
        while(pow9<=i){
            dp[i]=min(dp[i], dp[i-pow9]+1);
            pow9*=9;
        }
    }
    cout << dp[n] << endl;
    return 0;
}
