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

struct Edge {
    int to;
    long long weight;
    Edge(int to, long long weight) : to(to), weight(weight) {}
};

using ll = long long;
using ld = long double;
using pii = pair<int, int>;
using pll = pair<ll, ll>;
using Graph = vector<vector<int>>;
using weightedGraph = vector<vector<Edge>>;

#define BIL ((ll)1e9)
#define MOD ((ll)1e9+7)
#define INF (1LL<<60)           //1LL<<63でオーバーフロー
#define inf (1<<29)             //1<<29でオーバーフロー

int main(int argc,char* argv[]){
    cin.tie(0);
    ios::sync_with_stdio(0);
    cout << fixed << setprecision(20);
    ld x, y, r;
    cin >> x >> y >> r;
    ld r2=r*r;
    ll top=floorl(y+r), bottom=ceill(y-r);
    ll pos=floorl(x+r), neg=ceill(x-r);
    ll ans=0;
    for(ll i=bottom;i<=top;i++){
        // cout << "i: " << i << endl;
        ll l=ceill(x),r=pos;
        // cout << "l: " << l << " r: " << r << endl;
        ll la, ra;
        ll m=r;
        while(r-l>1){
            ll m=(l+r)/2;
            ld lhs=((ld)m-x)*((ld)m-x)+((ld)i-y)*((ld)i-y);
            if(lhs==r2){
                break;
            }else if(lhs<r2){
                r=m;
            }else if(lhs>r2){
                l=m;
            }
        }
        ra=m;
        if(((ld)ra-x)*((ld)ra-x)+((ld)i-y)*((ld)i-y) > r2) ra=0;
        l=neg, r=ceill(x);
        m=l;
        // cout << "l: " << l << " r: " << r << endl;
        while(r-l>1){
            ll m=(l+r)/2;
            ld lhs=((ld)m-x)*((ld)m-x)+((ld)i-y)*((ld)i-y);
            if(lhs==r2){
                break;
            }else if(lhs<r2){
                r=m;
            }else if(lhs>r2){
                l=m;
            }
        }
        la=m;
        if(((ld)la-x)*((ld)la-x)+((ld)i-y)*((ld)i-y) > r2) la=0;
        // cout << "la: " << la << " ra: " << ra << endl;
        ans+=(ra-la+1);
    }
    cout << ans << endl;
    return 0;
}
