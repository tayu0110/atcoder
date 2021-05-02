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
    int n,m;
    cin >> n >> m;
    multiset<ll> ck;
    for(int i=0;i<n;i++){
        ll a;
        cin >> a;
        ck.insert(a);
    }
    // cout << ck.size() << endl;
    for(int i=0;i<m;i++){
        auto it=ck.end();
        it--;
        ll last=*it;
        ck.erase(it);
        ck.insert(last/2);
    }
    ll ans=0;
    for(auto it=ck.begin();it!=ck.end();it++){
        ll a=*it;
        ans+=a;
        // cout << "a: " << a << endl;
    }
    cout << ans << endl;
    return 0;
}
