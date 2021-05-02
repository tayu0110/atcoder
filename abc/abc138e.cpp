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
  string s, t;
  cin >> s >> t;
  vector<vector<int>> c(26);
  for(int i=0;i<s.size();i++) {
    c[s[i]-'a'].push_back(i);
  }
  vector<ll> k(t.size());
  ll g = 0;
  for(int i=0;i<t.size();i++) {
    int d = t[i]-'a';
    if(c[d].empty()) {
      cout << -1 << endl;
      return 0;
    }
    if(i==0) {
      k[i] = c[d][0];
    } else {
      auto it = upper_bound(c[d].begin(), c[d].end(), k[i-1]);
      if(it == c[d].end()) {
        g += s.size();
        k[i] = c[d][0];
      } else {
        k[i] = *it;
      }
    }
  }
  cout << g + k[t.size()-1] + 1 << endl;
  // vector<vector<int>> c(26);
  // for(int i=0;i<s.size();i++) {
  //   int j = s[i]-'a';
  //   c[j].push_back(i);
  // }
  // int prev = -1;
  // ll ans = 0;
  // for(int i=0;i<t.size();i++) {
  //   int j = t[i]-'a';
  //   if(c[j].empty()) {
  //     cout << -1 << endl;
  //     return 0;
  //   }
  //   if(prev == -1) {
  //     ans += c[j][0];
  //     prev = c[j][0];
  //   } else {
  //     auto it = upper_bound(c[j].begin(), c[j].end(), prev);
  //     if(it == c[j].end()) {
  //       ans += (s.size() - prev) + c[j][0];
  //       prev = c[j][0];
  //     } else {
  //       ans += *it - prev;
  //       prev = *it;
  //     }
  //   }
  // }
  // cout << ans + 1 << endl;
  return 0;
}
