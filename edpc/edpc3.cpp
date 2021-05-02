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
    cin >> n;
    int dp[3][100010];
    for(int i=0;i<n;i++){
        for(int j=0;j<3;j++)dp[j][i]=0;
    }


    for(int i=0;i<n;i++){
        int a,b,c;
        cin >> a >> b >> c;
        if(i==0){
            dp[0][0]=a;
            dp[1][0]=b;
            dp[2][0]=c;
        }else{
            dp[0][i]=max(dp[1][i-1], dp[2][i-1]) + a;
            dp[1][i]=max(dp[0][i-1], dp[2][i-1]) + b;
            dp[2][i]=max(dp[0][i-1], dp[1][i-1]) + c;
        }
    }

    cout << max(dp[0][n-1], max(dp[1][n-1], dp[2][n-1])) << endl;

    return 0;
}
