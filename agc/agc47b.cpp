#include <iostream>
#include <iomanip>
#include <string>
#include <vector>
#include <algorithm>
#include <utility>
#include <tuple>
#include <map>
#include <queue>
#include <deque>
#include <set>
#include <stack>
#include <numeric>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <cmath>
#include <cassert>

#include <atcoder/all>

using namespace std;
using namespace atcoder;

#define DEBUG(var) cerr << #var << ": " << var << " "
#define DEBUG_EN(var) cerr << #var << ": " << var << endl

using ll = long long;
using ld = long double;
using pii = pair<int, int>;
using pll = pair<ll, ll>;
using Graph = vector<vector<int>>;
template<class T> void print_with_space(T p) { for(auto e : p) cerr << e << " "; cerr << endl; }

const ll MOD = 1e9 + 7;
const ll INF = 1LL << 60;
const int inf = 1 << 29;
const ld PI = acos(-1);
struct Trie {
  struct Node {
    vector<int> next;
    vector<ll> suf;
    char c;
    int shared;
    int str_end;
    Node(char _c, int _shared) : c(_c), shared(_shared), str_end(0) {
      next.assign(26, -1);
      suf.assign(26, 0);
    }
  };
  vector<Node> nodes;
  int root;
  Trie() : root(0) {
    nodes.push_back(Node(0, 0));
  }
  void insert(const string &str) {
    int now = 0;
    nodes[now].shared++;
    for(int i=0;i<str.length();i++) {
      int ntc = str[i] - 'a';
      int nt = nodes[now].next[ntc];
      if(nt < 0) {
        nodes[now].next[ntc] = nt = nodes.size();
        nodes.push_back(Node(str[i], 1));
      } else {
        nodes[nt].shared++;
      }
      now = nt;
    }
    nodes[now].str_end++;
  }
  bool find(const string &str, bool prefix) {
    int now = 0;
    for(int i=0;i<str.length();i++) {
      int ntc = str[i] - 'a';
      int next = nodes[now].next[ntc];
      if(next < 0) return false;
      now = next;
    }
    return prefix ? true : nodes[now].str_end > 0;
  }
  bool find(const string &str) {
    return find(str, false);
  }
  bool find_prefix(const string &prefix) {
    return find(prefix, true);
  }
  ll solve(const string &str) {
    int now = 0;
    vector<int> path;
    for(int i=0;i<str.length();i++) {
      path.push_back(now);
      int ntc = str[i] - 'a';
      now = nodes[now].next[ntc];
    }
    set<char> suf;
    ll res = 0;
    for(int i=str.length()-1;i>=0;i--) {
      suf.insert(str[i]);
      int node = path[i];
      for(auto c : suf) {
        int nt = nodes[node].next[c - 'a'];
        if(nt < 0) continue;
        res += nodes[nt].str_end;
      }
    }
    return res-1;
  }
};
int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n;
  cin >> n;
  vector<string> s(n);
  Trie trie;
  for(int i=0;i<n;i++) {
    cin >> s[i];
    reverse(s[i].begin(), s[i].end());
    trie.insert(s[i]);
  }
  ll ans = 0;
  for(int i=0;i<n;i++) {
    ans += trie.solve(s[i]);
  }
  cout << ans << endl;
  return 0;
}
