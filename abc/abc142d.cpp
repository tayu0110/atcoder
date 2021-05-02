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
    ll a,b;
    cin >> a >> b;
    ll g=gcd(a,b);
    map<ll,int> d;
    d.insert(make_pair(1,1));
    ll now=2;
    ll ng=g;
    while(ng>1 && now*now<=g){
        if(ng%now==0){
            d.insert(make_pair(now,1));
            while(ng%now==0)ng/=now;
        }
        now++;
    }
    if(ng!=1)d.insert(make_pair(g,1));
    cout << d.size() << endl;
    return 0;
}
