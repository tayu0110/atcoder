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
    int n, c;
    cin >> n >> c;
    vector<map<int, int>> cl(2);
    for(int i = 0; i < 2; i++)for(int j = 1; j <= 10; j++) cl[i][j] = 0;
    for(int i = 0; i < n; i++) {
        int a;
        cin >> a;
        cl[i%2][a]++;
    }
    int rem = 0;
    for(auto i=cl[0].begin();i!=cl[0].end();i++){
        for(auto j=cl[1].begin();j!=cl[1].end();j++){
            if(i->first == j->first) continue;
            rem = max(i->second + j->second, rem);
        }
    }
    cout << (n-rem) * c << endl;
    return 0;
}
