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

set<int> ck;
int dfs(int node, vector<int> &dp, vector<vector<int>> &t) {
    if(ck.find(node)!=ck.end()) return dp[node];
    ck.insert(node);
    int ans=0;
    for(int i=0;i<t[node].size();i++){
        int j=t[node][i];
        ans=max(ans, dfs(j, dp, t)+1);
    }
    return dp[node]=ans;
}

int main(int argc,char* argv[]){
    cin.tie(0);
    ios::sync_with_stdio(0);
    cout << fixed << setprecision(20);
    int n,m;
    cin >> n >> m;
    vector<vector<int>> t(n+1, vector<int>(0));
    for(int i=0;i<m;i++){
        int x, y;
        cin >> x >> y;
        t[x].push_back(y);
    }
    vector<int> dp(n+1, 0);
    for(int i=1;i<n+1;i++){
        dfs(i, dp, t);
    }
    int ans=0;
    for(int i=1;i<n+1;i++) ans=max(ans, dp[i]);
    cout << ans << endl;
    return 0;
}
