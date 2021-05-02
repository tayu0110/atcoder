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
    if(n==1){
        cout << 1 << endl;
        return 0;
    }
    vector<int> b(n+1);
    b[1]=1;
    for(int i=2;i<n+1;i++){
        cin >> b[i];
    }
    vector<pii> p(n+1,make_pair(0,1000000));
    p[n].first=1,p[n].second=1;
    set<int> ck;
    for(int i=n;i>1;i--){
        int par=b[i];
        ck.insert(par);
        int pay;
        if(ck.find(i)==ck.end()) pay=1;
        else pay=p[i].first+p[i].second+1;
        if(p[par].first<pay) p[par].first=pay;
        if(p[par].second>pay) p[par].second=pay;
    }
    cout << p[1].first+p[1].second+1 << endl;
    return 0;
}
