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

int dp[2][101];

int main(int argc,char* argv[]){
    cin.tie(0);
    ios::sync_with_stdio(0);
    cout << fixed << setprecision(20);
    int a,b,x,y;
    cin >> a >> b >> x >> y;
    if(a==b){
        cout << x << endl;
    }else if(a<b){
        dp[0][a]=0;
        dp[1][a]=x;
        for(int i=a+1;i<=b;i++){
            dp[0][i]=min(dp[0][i-1]+y,dp[1][i-1]+x);
            dp[1][i]=min(dp[0][i]+x, dp[1][i-1]+y);
        }
        cout << dp[1][b] << endl;
    }else{
        dp[0][a]=0;
        dp[1][a]=x;
        for(int i=a-1;i>=b;i--){
            dp[1][i]=min(dp[0][i+1]+x, dp[1][i+1]+y);
            dp[0][i]=min(dp[0][i+1]+y, dp[1][i]+x);
        }
        cout << dp[1][b] << endl;

        // for(int i=0;i<2;i++){
        //     for(int j=a;j>=b;j--){
        //         cout << dp[i][j] << " ";
        //     }
        //     cout << endl;
        // }
    }
    return 0;
}
