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
    vector<string> s(n);
    for(int i=0;i<n;i++){
        cin >> s[i];
    }
    vector<vector<ll>> dp(2, vector<ll>(n+1,0));
    dp[0][0]=1;
    dp[1][0]=1;
    for(int i=1;i<n+1;i++){
        string t=s[i-1];
        // cout << "i: " << i << endl;
        for(int j=0;j<2;j++){
            for(int k=0;k<2;k++){
                int nj=j;
                // cout << "nj: " << nj << " k: " << k << " ";
                if(t=="AND") nj &= k;
                else nj |= k;
                dp[nj][i]+=dp[j][i-1];
                // cout << "nj: " << nj << " k: " << k << endl;
            }
        }
    }
    // for(int i=0;i<2;i++){
    //     for(int j=0;j<n+1;j++){
    //         cout << dp[i][j] << " ";
    //     }
    //     cout << endl;
    // }
    cout << dp[1][n] << endl;
    return 0;
}
