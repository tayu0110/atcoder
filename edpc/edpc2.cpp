#include<iostream>
#include<string>
#include<vector>
#include<algorithm>
#include<utility>
#include<tuple>
#include<map>
#include<queue>
#include<set>
#include<stack>
#include<cstdio>
#include<cstdlib>
#include<cstring>
#include<cmath>

using namespace std;

using ll = long long;
using pii = pair<int, int>;
using pll = pair<ll, ll>;

#define bil ((ll)1e9)
#define mod ((ll)1e9+7)
#define inf (1<<29)             //1<<29でオーバーフロー
#define INF (1LL<<60)           //1LL<<63でオーバーフロー

int main(int argc,char* argv[]){
    int n,k;
    cin >> n >> k;
    vector<int> h(n);
    for(auto &x:h){
        cin >> x;
    }

    vector<ll> dp(n,INF);
    dp[0]=0;
    for(int i=1;i<n;i++){
        for(int j=1;j<=k;j++){
            if(i-j>=0)dp[i]=min(dp[i],dp[i-j]+abs(h[i]-h[i-j]));
        }
        // cout << "dp["<<i<<"]="<<dp[i]<<endl;
    }

    cout << dp[n-1] << endl;

    return 0;
}
