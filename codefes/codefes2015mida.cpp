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
    int n,k;
    ll m,r;
    cin >> n >> k >> m >> r;
    vector<ll> s(n-1);
    for(int i=0;i<n-1;i++){
        cin >> s[i];
    }
    if(k==n){
        ll sum=0;
        for(int i=0;i<n-1;i++){
            sum+=s[i];
        }
        ll ans=r*k-sum;
        if(ans>m) cout << -1 << endl;
        else if(ans<0) cout << 0 << endl;
        else cout << ans << endl;
        return 0;
    }
    sort(s.begin(),s.end());
    ll sum=0;
    ll ave=0;
    for(int i=n-2;i>n-2-k;i--){
        sum+=s[i];
    }
    ave=sum/k;
    if(ave>=r){
        cout << 0 << endl;
    }else{
        ll ans=r*k-(sum-s[n-1-k]);
        if(ans>m) cout << -1 << endl;
        else cout << r*k-(sum-s[n-1-k]) << endl;
    }
    return 0;
}
