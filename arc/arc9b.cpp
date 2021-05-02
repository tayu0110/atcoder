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
    vector<int> b(10);
    for(int i=0;i<10;i++){
        int m;
        cin >> m;
        b[m]=i;
    }
    int n;
    cin >> n;
    vector<string> a(n);
    for(int i=0;i<n;i++){
        cin >> a[i];
    }
    vector<pair<string, string>> c(n);
    for(int i=0;i<n;i++){
        string s;
        string d=a[i];
        for(int j=0;j<d.size();j++){
            s+=to_string(b[d[j]-'0']);
        }
        while(s.size()<9) s="0"+s;
        c[i].first=s;
        c[i].second=d;
    }
    sort(c.begin(),c.end());
    for(int i=0;i<n;i++){
        cout << c[i].second << endl;
    }
    return 0;
}
