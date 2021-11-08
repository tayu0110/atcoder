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
#include<cassert>

using namespace std;

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
const ld PI = 3.141592653589793238462643383;

int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  ll n;
  int m;
  cin >> n >> m;
  vector<pll> p(m);
  for(int i=0;i<m;i++) {
    ll x, y;
    cin >> x >> y;
    p[i] = {x, y};
  }
  sort(p.begin(), p.end());
  set<ll> ck;
  ck.insert(n);
  int t = 0;
  while(t < m) {
    vector<ll> a;
    vector<ll> b;
    ll x = p[t].first;
    while(t < m && p[t].first == x) {
      ll y = p[t].second;
      if(ck.find(y-1) != ck.end()) a.push_back(y);
      else if(ck.find(y+1) != ck.end()) a.push_back(y);
      else b.push_back(y);
      t++;
    }
    for(auto e : a) ck.insert(e);
    for(auto e : b) ck.erase(e);
  }
  cout << ck.size() << endl;
  return 0;
}
