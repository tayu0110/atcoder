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

int main(int argc, char* argv[]){
  cout << fixed << setprecision(20);
  int t;
  cin >> t;
  for(int i=1;i<=t;i++) {
    int n;
    cin >> n;
    vector<string> s(n);
    for(int j=0;j<n;j++) cin >> s[j];
    vector<string> v;
    set<int> rem;
    for(int i=0;i<n;i++) rem.insert(i);
    while(rem.size()) {
      bool cat = false;
      deque<int> nt;
      int start = *rem.begin();
      nt.push_back(start);
      rem.erase(start);
      char f = s[start].front(), b = s[start].back();
      while(true) {
        int nf = -1, nb = -1;
        for(auto e : rem) {
          string &k = s[e];
          if(f == k.back() && (nf < 0 || k.front() == k.back())) nf = e, cat = true;
          else if(b == k.front() && (nb < 0 || k.front() == k.back())) nb = e, cat = true;
        }
        if(nf >= 0) {
          nt.push_front(nf);
          rem.erase(nf);
          f = s[nf].front();
        }
        if(nb >= 0) {
          nt.push_back(nb);
          rem.erase(nb);
          b = s[nb].back();
        }
        if(nf < 0 && nb < 0) break;
      }
      string res;
      while(!nt.empty()) res += s[nt.front()], nt.pop_front();
      if(cat) {
        rem.insert(s.size());
        s.push_back(res);
      } else {
        v.push_back(res);
      }
    }
    string res;
    for(auto c : v) res += c;
    set<char> ck;
    char prev = '0';
    bool bad = false;
    for(auto c : res) {
      if(c == prev) continue;
      if(ck.find(c) != ck.end()) {
        bad = true;
        break;
      }
      ck.insert(prev);
      prev = c;
    }
    if(bad) printf("Case #%d: IMPOSSIBLE\n", i);
    else printf("Case #%d: %s\n", i, res.c_str());
  }
  return 0;
}
