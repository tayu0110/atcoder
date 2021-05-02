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
using ull = unsigned long long;
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
    vector<ull> a(n);
    ull sum=0;
    vector<vector<ll>> d(2,vector<ll>(60));
    for(int i=0;i<n;i++){
        cin >> a[i];
        for(int j=0;j<60;j++){
            d[a[i]%2][j]++;
            a[i]/=2;
        }
    }
    ll ans=0;
    ll now=1;
    for(int i=0;i<60;i++){
        ans+=(d[0][i]*d[1][i])%MOD*(now%MOD);
        ans%=MOD;
        now*=2;
    }
    cout << ans << endl;
    return 0;
}
