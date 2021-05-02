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
  bool operator>(const Edge &e) const { return weight > e.weight; }
  bool operator<(const Edge &e) const { return weight < e.weight; }
  bool operator==(const Edge &e) const { return weight == e.weight; }
  bool operator<=(const Edge &e) const { return weight <= e.weight; }
  bool operator>=(const Edge &e) const { return weight >= e.weight; }
};

using ll = long long;
using ld = long double;
using pii = pair<int, int>;
using pll = pair<ll, ll>;
using Graph = vector<vector<int>>;
using weightedGraph = vector<vector<Edge>>;
using heap = priority_queue<int, vector<int>, greater<int>>;

const ll BIL = 1e9;
const ll MOD = 1e9 + 7;
const ll INF = 1LL << 60;
const int inf = 1 << 29;
const ld PI = 3.141592653589793238462643383;

// vector<vector<ll>> d;
// ll dfs(string &s, string &n) {
//   if(s.length() > n.length()) return 0;
//   int len = s.length();
//   string t = n.substr(0, len);
//   cout << "len: " << len << " s: " << s << " t: " << t << endl;
//   if(s.length() == n.length()) {
//     int len = s.length();
//     int res;
//     if(s[len-1] == '1') res = d[len][1] = 1;
//     else res = d[len][s[len-1]-'0'] = 0;
//     return res;
//   }
//   if(s > t) {
//     cout << "res: " << 0 << endl;
//     return 0;
//   } else if(s == t) {
//     ll res = 0;
//     string l = t + n[len];
//     for(int i=0;i<10;i++) {
//       char c = '0' + i;
//       string k = s + c;
//       if(k > l) break;
//       ll m = dfs(k, n);
//       if(s[len-1] == '1') m++;
//       res += m;
//       cout << "k: " << k << " dfs(k, n): " << m << endl;
//     }
//     cout << "res: " << res << endl;
//     return res;
//   } else {
//     if(d[len][s[len-1] - '0'] >= 0) {
//       cout << "res: " << d[len][s[len-1]-'0'] << endl;
//       return d[len][s[len-1] - '0'];
//     }
//     ll res = 0;
//     for(int i=0;i<10;i++) {
//       char c = '0' + i;
//       string k = s + c;
//       ll m = dfs(k, n);
//       if(s[len-1] == '1') m++;
//       res += m;
//       cout << "k: " << k << " dfs(k, n): " << m << endl;
//     }
//     cout << "res: " << res << endl;
//     return d[len][s[len-1]-'0'] = res;
//   }
// }

int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  ll n;
  cin >> n;
  ll res = 0;
  ll prev = 0;
  for(ll i=1;i<10000000000;i *= 10) {
    res += n / (10 * i) * i;
    if(n % (10 * i) > i - 1) {
      res += min(n % (10 * i) - (i - 1), i);
    }
    // cout << "res: " << res - prev << endl;
    prev = res;
  }
  cout << res << endl;
  return 0;
}
