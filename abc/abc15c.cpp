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
    cin >> n >> k;
    vector<vector<int>> t(n, vector<int>(k));
    for(int i=0;i<n;i++){
        for(int j=0;j<k;j++){
            cin >> t[i][j];
        }
    }
    set<int> ck;
    ck.insert(0);
    for(int i=0;i<n;i++){
        set<int> a;
        for(auto it=ck.begin();it!=ck.end();it++){
            int k=*it;
            for(int j=0;j<t[i].size();j++){
                a.insert(k^t[i][j]);
            }
        }
        ck=a;
    }
    if(ck.find(0)!=ck.end()) cout << "Found" << endl;
    else cout << "Nothing" << endl;
    return 0;
}
