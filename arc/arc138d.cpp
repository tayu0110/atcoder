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
  int n, k;
  cin >> n >> k;
  vector<int> t;
  for(int i=0;i<(1<<n);i++) {
    int p = __builtin_popcount(i);
    if(p == k) t.push_back(i);
  }
  set<int> ck;
  ck.insert(0);
  vector<int> a(1, 0);
  int cnt = 0;
  for(int i=0;i<(1<<n)-1;i++) {
    int b = a.back();
    int v = b ^ t[cnt];
    if(i == (1<<n)-2) {
      if(ck.find(v) != ck.end() || __builtin_popcount((v ^ a[0])) != k) {
        bool f = false;
        for(int j=0;j<t.size();j++) {
          cnt = (cnt+1) % t.size();
          v = b ^ t[cnt];
          if(ck.find(v) == ck.end() && __builtin_popcount((v ^ a[0])) == k) {
            f = true;
            a.push_back(v);
            break;
          }
        }
        if(!f) {
          cout << "No" << endl;
          return 0;
        }
        break;
      } else {
        a.push_back(v);
        break;
      }
    }
    if(ck.find(v) != ck.end()) {
      bool f = false;
      for(int j=0;j<t.size();j++) {
        cnt = (cnt+1) % t.size();
        v = b ^ t[cnt];
        if(ck.find(v) == ck.end()) {
          f = true;
          a.push_back(v);
          ck.insert(v);
          break;
        }
      }
      if(!f) {
        cout << "No" << endl;
        return 0;
      }
    } else {
      a.push_back(v);
      ck.insert(v);
    }
    cnt = (cnt+1) % t.size();
  }
  if(__builtin_popcount((a[0] ^ a[a.size()-1])) == k) {
    cout << "Yes" << endl;
    for(int i=0;i<a.size();i++) {
      if(i) cout << " "; cout << a[i];
    }
    cout << endl;
  } else {
    cout << "No" << endl;
  }
  return 0;
}
