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

    ll n,a,b,c,d,e;
    cin >> n >> a >> b >> c >> d >> e;
    ll bn=min(a, min(b, min(c, min(d, e))));
    ll block=ceil((ld)n/(ld)bn);
    // cout << "block=" << block << endl;
    ll ans=5+block-1;
    cout << ans << endl;
    // vector<ll> city(6,0);
    // city[0]=n;
    // ll tm=0;
    // while(city[5]!=n){
    //     city[5]+=min(city[4],e);
    //     city[4]=max(0LL,city[4]-e);
    //     city[4]+=min(city[3],d);
    //     city[3]=max(0LL,city[3]-d);
    //     city[3]+=min(city[2],c);
    //     city[2]=max(0LL,city[2]-c);
    //     city[2]+=min(city[1],b);
    //     city[1]=max(0LL,city[1]-b);
    //     city[1]+=min(city[0],a);
    //     city[0]=max(0LL,city[0]-a);

    //     tm++;
    //     cout << tm << ": ";
    //     for(int i=0;i<6;i++){
    //         cout << city[i] << " ";
    //     }
    //     cout << endl;
    // }

    // cout << tm << endl;

    return 0;
}
