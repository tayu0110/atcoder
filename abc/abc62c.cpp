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
    ll h,w;
    cin >> h >> w;
    ll ans=INF;
    for(ll i=1;i<h;i++){
        ll s1=w*i;
        ll s2,s3;
        if((h-i)%2==0)s2=s3=w*((h-i)/2);
        else {
            s2=w*((h-i)/2);
            s3=w*((h-i)/2+1);
        }
        ll sd=max(s1, max(s2,s3))-min(s1, min(s2, s3));
        ans=min(ans, sd);
        if(w%2==0)s2=s3=(h-i)*(w/2);
        else {
            s2=(h-i)*(w/2);
            s3=(h-i)*(w/2+1);
        }
        sd=max(s1, max(s2,s3))-min(s1, min(s2, s3));
        ans=min(ans, sd);
    }
    for(ll i=1;i<w;i++){
        ll s1=h*i;
        ll s2,s3;
        if((w-i)%2==0)s2=s3=h*((w-i)/2);
        else {
            s2=h*((w-i)/2);
            s3=h*((w-i)/2+1);
        }
        ll sd=max(s1, max(s2,s3))-min(s1, min(s2, s3));
        ans=min(ans, sd);
        if(h%2==0)s2=s3=(w-i)*(h/2);
        else {
            s2=(w-i)*(h/2);
            s3=(w-i)*(h/2+1);
        }
        sd=max(s1, max(s2,s3))-min(s1, min(s2, s3));
        ans=min(ans, sd);
    }
    cout << ans << endl;
    return 0;
}
