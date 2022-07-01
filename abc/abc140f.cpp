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

using namespace std;

#define DEBUG(var) cerr << #var << ": " << (var) << " "
#define DEBUG_EN(var) cerr << #var << ": " << (var) << endl

using ll = long long;
using ld = long double;
using pii = pair<int, int>;
using pll = pair<ll, ll>;
template<class T> void print_with_space(T p) { for(auto e : p) cerr << e << " "; cerr << endl; }

const ll MOD = 1e9 + 7;
const ll INF = 1LL << 60;
const int inf = 1 << 29;

int main(int argc, char* argv[]) {
  int n;
  cout << fixed << setprecision(20);
  cin >> n;
  int m = 1 << n;
  vector<int> s(m);
  for(int i=0;i<m;i++) cin >> s[i];
  map<int, int> mp;
  for(int i=0;i<m;i++) mp[s[i]]++;
  vector<pii> v;
  for(auto e : mp) v.push_back(e);
  sort(v.begin(), v.end(), greater<pii>());
  priority_queue<int> nt;
  nt.push(m);
  for(auto now : v) {
    int cnt = now.second;
    if(nt.size() < cnt) {
      cout << "No" << endl;
      return 0;
    }
    vector<int> t;
    for(int i=0;i<cnt;i++) {
      t.push_back(nt.top());
      nt.pop();
    }
    for(auto c : t) {
      c /= 2;
      while(c) {
        nt.push(c);
        c /= 2;
      }
    }
  }
  cout << "Yes" << endl;
  return 0;
}
