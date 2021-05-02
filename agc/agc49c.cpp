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
    vector<ll> a(n+1), b(n+1);
    for(int i=1;i<n+1;i++) cin >> a[i];
    for(int i=1;i<n+1;i++) cin >> b[i];
    vector<vector<ll>> dp(n+1, vector<ll>(2));
    for(int i=1;i<n+1;i++)for(int j=0;j<2;j++){
        if(j==0) dp[i][j]=min(dp[i-1][0]+1,dp[i-1][1]+1);
        else{
            if(a[i]-b[i]<=0) dp[i][j]=b[i]-a[i]+1;
            else{
                ll bg=1,en=n,num=a[i]-b[i],pt;
                if(num<a[bg]) pt=0;
                else if(num>=a[en]) pt=en;
                else{
                    while(bg!=en){
                        int p=bg+(en-bg)/2;
                        if(num>a[p]) bg=p+1;
                        else en=p;
                        if(bg>=en){
                            pt=min(bg,en);
                            if(num==a[pt])pt--;
                            break;
                        }
                    }
                }
                dp[i][j]=min(dp[pt][0], dp[pt][1]);
            }
        }
    }

    // for(int i=1;i<n+1;i++){
    //     for(int j=0;j<2;j++){
    //         cout << dp[i][j] << " ";
    //     }
    //     cout << endl;
    // }

    cout << min(dp[n][0],dp[n][1]) << endl;

    return 0;
}
