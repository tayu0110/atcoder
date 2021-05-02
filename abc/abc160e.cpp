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
    int x,y,a,b,c;
    cin >> x >> y >> a >> b >> c;
    vector<ll> p(a),q(b),r(c);
    for(int i=0;i<a;i++) cin >> p[i];
    for(int i=0;i<b;i++) cin >> q[i];
    for(int i=0;i<c;i++) cin >> r[i];
    sort(p.begin(),p.end(), greater<ll>());
    sort(q.begin(),q.end(), greater<ll>());
    sort(r.begin(),r.end(), greater<ll>());
    int na=0,nb=0,nc=0;
    ll ans=0;
    for(int i=0;i<x+y;i++){
        ll np,nq,nr;
        if(na<a && na<x) np=p[na];
        else np=-1;
        if(nb<b && nb<y) nq=q[nb];
        else nq=-1;
        if(nc<c) nr=r[nc];
        else nr=-1;
        if(np>=nq && np>=nr){
            ans+=np;
            na++;
        }else if(nq>=np && nq>=nr){
            ans+=nq;
            nb++;
        }else{
            ans+=nr;
            nc++;
        }
        // cout << "ans: " << ans << endl;
    }
    // cout << na << " " << nb << " " << nc << endl;
    cout << ans << endl;
    return 0;
}
