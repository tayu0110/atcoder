//DP初歩
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

int main(int argc,char* argv[]){
    int n;
    cin >> n;

    vector<int> h(n);
    for(auto &x:h)
        cin >> x;
    
    vector<int> dp(n);
    dp[0]=0;
    dp[1]=abs(h[1]-h[0]);

    for(int i=2;i<n;i++){
        dp[i]=min(dp[i-2]+abs(h[i]-h[i-2]),dp[i-1]+abs(h[i]-h[i-1]));
    }

    cout << dp[n-1] << endl;

    return 0;
}
