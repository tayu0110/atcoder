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
    string s;
    int t;
    cin >> s >> t;
    int x=0,y=0;
    int q=0;
    for(int i=0;i<s.size();i++){
        if(s[i]=='L') x--;
        else if(s[i]=='R') x++;
        else if(s[i]=='U') y++;
        else if(s[i]=='D') y--;
        else q++;
    }
    int dist = abs(x) + abs(y);
    if(t==1) {
        cout << dist + q << endl;
    } else {
        if(dist >= q) cout << dist - q << endl;
        else {
            q-=dist;
            cout << q%2 << endl;
        }
    }
    return 0;
}
