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
    vector<vector<int>> f(n,vector<int>(10,0));
    for(int i=0;i<n;i++){
        for(int j=0;j<10;j++){
            cin >> f[i][j];
        }
    }
    vector<vector<ll>> p(n,vector<ll>(11));
    for(int i=0;i<n;i++){
        for(int j=0;j<11;j++){
            cin >> p[i][j];
        }
    }
    ll ans=-INF;
    for(int i=1;i<(1<<10);i++){
        ll now=0;
        for(int k=0;k<n;k++){
            int c=0;
            for(int j=0;j<10;j++){
                if((i & (1<<j)) && f[k][j]) c++;
            }
            // cout << "c: " << c << " ";
            now+=p[k][c];
        }
        // cout << "now: " << now << endl;
        ans=max(ans, now);
    }
    cout << ans << endl;
    return 0;
}
