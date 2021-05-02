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
    int n;
    cin >> n;
    vector<int> a(n);
    set<int> ck;
    for(int i=0;i<n;i++){
        cin >> a[i];
        ck.insert(a[i]);
    }
    ll ans=0;
    for(auto it=ck.begin();it!=ck.end();it++){
        ll sum=0;
        ll k=*it;
        for(int i=0;i<n;i++){
            if(a[i]<k){
                ans=max(sum, ans);
                sum=0;
                continue;
            }
            sum+=k;
        }
        ans=max(ans, sum);
    }
    cout << ans << endl;
    return 0;
}
