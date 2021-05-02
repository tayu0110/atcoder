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
    int h, w, d;
    cin >> h >> w >> d;
    map<int, pii> mp;
    vector<vector<int>> a(h, vector<int>(w, 0));
    for(int i = 0; i < h; i++) {
        for(int j = 0; j < w; j++) {
            cin >> a[i][j];
            mp[a[i][j]] = make_pair(i+1, j+1);
        }
    }
    vector<ll> s(h*w+1);
    for(int i = 1; i < h*w+1; i++) {
        if(i+d >= h*w+1) continue;
        int y = mp[i].first, x = mp[i].second;
        int ny = mp[i+d].first, nx = mp[i+d].second;
        s[i+d] = s[i] + abs(ny-y) + abs(nx-x);
    }
    int q;
    cin >> q;
    for(int i = 0; i < q; i++) {
        int l, r;
        cin >> l >> r;
        cout << s[r] - s[l] << endl;
    }
    return 0;
}
