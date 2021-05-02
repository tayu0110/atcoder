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
#include<fstream>
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
    vector<pii> xy(n);
    for(int i=0;i<n;i++){
        cin >> xy[i].first >> xy[i].second;
    }
    int m;
    cin >> m;
    vector<string> op(m);
    for(int i=0;i<m;i++){
        getline(cin, op[i]);
    }
    int q;
    cin >> q;
    for(int i=0;i<q;i++){
        ll a,b;
        cin >> a >> b;
        a--;b--;
        ll x=xy[b].first, y=xy[b].second;
        string p=op[a];
        if(p=="1"){            
            cout << y << " " << -x << endl;
            xy[b].first=y;
            xy[b].second=-x;
        }else if(p=="2"){
            cout << -y << " " << x << endl;
            xy[b].first=-y;
            xy[b].second=x;
        }else if(p[0]=='3'){
            ll pp=stoll(p.substr(2));
            cout << x << " " << 2*pp-y << endl;
            xy[b].second=2*pp-y;
        }else if(p[0]=='4'){
            ll pp=stoll(p.substr(2));
            cout << 2*pp-x << " " << y << endl;
            xy[b].first=2*pp-x;
        }
    }
    return 0;
}
