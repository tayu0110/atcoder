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

struct Edge {
    int to;
    long long weight;
    Edge(int to, long long weight) : to(to), weight(weight) {}
};

using ll = long long;
using ld = long double;
using pii = pair<int, int>;
using pll = pair<ll, ll>;
using Graph = vector<vector<int>>;
using weightedGraph = vector<vector<Edge>>;

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
    vector<int> r(n);
    for(int i=0;i<n;i++){
        cin >> r[i];
    }
    vector<vector<int>> dp(2, vector<int>(n,0));
    for(int i=0;i<n;i++){
        dp[0][i]=1;
        dp[1][i]=1;
        for(int j=0;j<i;j++){
            if(r[i]<r[j]) dp[0][i]=max(dp[0][i], dp[1][j]+1);
            if(r[i]>r[j]) dp[1][i]=max(dp[1][i], dp[0][j]+1);
        }
    }
    int ans = max(dp[0][n-1], dp[1][n-1]);
    if(ans < 3) cout << 0 << endl;
    else cout << ans << endl;
    return 0;
}
