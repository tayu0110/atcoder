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
    string s,t;
    cin >> s >> t;
    int slen=s.length(),tlen=t.length();
    vector<vector<int>> dp(slen+1, vector<int>(tlen+1,0));
    for(int i=1;i<slen+1;i++){
        for(int j=1;j<tlen+1;j++){
            if(s[i-1]==t[j-1]){
                dp[i][j]=dp[i-1][j-1]+1;
            }else{
                dp[i][j]=max(dp[i][j-1], dp[i-1][j]);
            }
        }
    }
    string ans;
    int i=slen;
    int j=tlen;
    int len=dp[slen][tlen];
    while(len>0) {
        if(s[i-1]==t[j-1]){
            ans=s[i-1]+ans;
            i--;j--;
            len--;
        }else if(dp[i-1][j]==dp[i][j]) i--;
        else j--;
    }
    cout << ans << endl;
    return 0;
}
