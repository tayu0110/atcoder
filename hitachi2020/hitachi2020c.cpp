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
using Graph = vector<vector<int>>;
template<class T> void print_with_space(T p) { for(auto e : p) cerr << e << " "; cerr << endl; }

const ll MOD = 1e9 + 7;
const ll INF = 1LL << 60;
const int inf = 1 << 29;
const ld PI = acos(-1);

int main(int argc, char* argv[]){
  cout << fixed << setprecision(20);
  int n;
  cin >> n;
  vector<vector<int>> t(n);
  for(int i=0;i<n-1;i++) {
    int a, b;
    cin >> a >> b;
    a--; b--;
    t[a].push_back(b);
    t[b].push_back(a);
  }
  queue<int> nt;
  vector<int> ck(n, -1);
  nt.push(0);
  while(!nt.empty()) {
    int now = nt.front(); nt.pop();
    for(auto to : t[now]) {
      if(ck[to] >= 0) continue;
      ck[to] = (ck[now] + 1) % 2;
      nt.push(to);
    }
  }
  set<int> r, b;
  for(int i=0;i<n;i++) {
    if(ck[i]) r.insert(i+1);
    else b.insert(i+1);
  }
  if(r.size() > n/3 && b.size() > n/3) {
    int a = 1, b = 2, c = 3;
    for(int i=1;i<=n;i++) {
      if(i > 1) cout << " ";
      if(r.find(i) != r.end()) {
        if(a > n) {
          cout << c;
          c += 3;
        } else {
          cout << a;
          a += 3;
        }
      } else {
        if(b > n) {
          cout << c;
          c += 3;
        } else {
          cout << b;
          b += 3;
        }
      }
    }
    cout << endl;
  } else {
    if(b.size() <= n/3) r.swap(b);
    int a = 1, b = 2, c = 3;
    for(int i=1;i<=n;i++) {
      if(i > 1) cout << " ";
      if(r.find(i) != r.end()) {
        cout << c;
        c += 3;
      } else {
        if(a <= n) {
          cout << a;
          a += 3;
        } else if(b <= n) {
          cout << b;
          b += 3;
        } else {
          cout << c;
          c += 3;
        }
      }
    }
    cout << endl;
  }
  return 0;
}
