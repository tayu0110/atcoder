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
    ll n;
    cin >> n;
    vector<ll> a(n+1);
    a[0]=-INF;
    for(int i=1;i<n+1;i++){
        cin >> a[i];
    }
    sort(a.begin(),a.end());
    ll ans=0;
    for(ll i=n;i>0;i--){
        ans+=(i-1)*a[i];
        ans-=(n-i)*a[i];
        // cout << "a[i]: " << a[i] << " ans: " << ans << endl;
    }
    cout << ans << endl;
    return 0;
}
