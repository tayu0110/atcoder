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
  Edge() : to(0), weight(0) {}
  Edge(int to, long long weight) : to(to), weight(weight) {}
  Edge(const Edge& e) {
    to = e.to;
    weight = e.weight;
  }
};

using ll = long long;
using ld = long double;
using pii = pair<int, int>;
using pll = pair<ll, ll>;
using Graph = vector<vector<int>>;
using weightedGraph = vector<vector<Edge>>;
using heap = priority_queue<int, vector<int>, greater<int>>;

#define BIL ((ll)1e9)
#define MOD ((ll)1e9+7)
#define INF (1LL<<60)           //1LL<<63でオーバーフロー
#define inf (1<<29)             //1<<29でオーバーフロー

int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int t;
  cin >> t;
  while(t--) {
    int n, m;
    cin >> n >> m;
    vector<ll> a(n);
    map<int, int> mp;
    for(int i=0;i<n;i++) {
      cin >> a[i];
      a[i] %= m;
      mp[a[i]]++;
    }
    int ans = 0;
    for(auto &e : mp) {
      if(e.second == 0) continue;
      if(e.first == 0) ans++;
      else {
        int j = m - e.first;
        if(mp.find(j) != mp.end()) {
          if(abs(e.second - mp[j]) <= 1) {
            ans++;
            mp[j] = 0;
            e.second = 0;
          } else {
            if(e.second < mp[j]) {
              ans++;
              mp[j] -= e.second + 1;
              ans += mp[j];
              mp[j] = 0;
              e.second = 0;
            } else {
              ans++;
              e.second -= mp[j] + 1;
              ans += e.second;
              e.second = 0;
              mp[j] = 0;
            }
          }
        } else {
          ans += e.second;
        }
      }
      // cout << "e.first: " << e.first << " ans: " << ans << endl;
    }
    cout << ans << endl;
  }
  return 0;
}
