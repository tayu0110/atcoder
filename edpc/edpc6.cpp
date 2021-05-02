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

int dp[3010][3010];

int main(int argc,char* argv[]){
    cin.tie(0);
    ios::sync_with_stdio(0);
    cout << fixed << setprecision(20);

    string s,t;
    cin >> s >> t;
    int slen = s.size();
    int tlen = t.size();

    for(int i=1;i<slen+1;i++){
        for(int j=1;j<tlen+1;j++){
            if(s[i-1] == t[j-1]) dp[i][j] = dp[i-1][j-1]+1;
            else dp[i][j] = max(dp[i-1][j], dp[i][j-1]);
        }
    }

    int len = dp[slen][tlen];
    int i = slen;
    int j = tlen;
    char ans[3010];

    while(len > 0) {
        if(s[i-1] == t[j-1]){
            ans[len] = s[i-1];
            i--;
            j--;
            len--;
        }else if(dp[i][j] == dp[i-1][j]){
            i--;
        }else{
            j--;
        }
    }

    for(int i=1;i<dp[slen][tlen]+1;i++){
        cout << ans[i];
    }
    cout << endl;

    return 0;
}
