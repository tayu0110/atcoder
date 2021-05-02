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
    ll a,b;
    cin >> n >> a >> b;
    vector<ll> x(n);
    for(int i=0;i<n;i++){
        cin >> x[i];
    }
    ll ans=0;
    for(int i=1;i<n;i++){
        ll diff=x[i]-x[i-1];
        if(diff*a<=b){
            ans+=diff*a;
        }else{
            ans+=b;
        }
    }
    cout << ans << endl;
    return 0;
}
