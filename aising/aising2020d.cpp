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

ll modPow(long long a, long long n, long long p) {
  if (n == 0) return 1;
  if (n == 1) return a % p;
  if (n % 2 == 1) return (a * modPow(a, n - 1, p)) % p;
  long long t = modPow(a, n / 2, p);
  return (t * t) % p;
}
int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n;
  cin >> n;
  string x;
  cin >> x;
  int b = 0;
  for(int i=0;i<n;i++) if(x[i] == '1') b++;
  if(b == 0) {
    for(int i=0;i<n;i++) cout << 1 << endl;
    return 0;
  }
  vector<ll> l(n), m(n);
  ll dl = 0, dm = 0;
  for(int i=0;i<n;i++) {
    if(b > 1) l[i] = modPow(2, n-1-i, b-1);
    if(b != n) m[i] = modPow(2, n-1-i, b+1);
    if(x[i] == '1') {
      dl += l[i];
      dm += m[i];
    }
  }
  vector<int> res(n+1);
  for(int i=1;i<n+1;i++) {
    int pc = 0;
    for(int j=0;j<20;j++) {
      if(i & (1<<j)) pc++;
    }
    if(pc == 0) continue;
    int k = i % pc;
    res[i] = res[k] + 1;
  }
  for(int i=0;i<n;i++) {
    if(x[i] == '0') {
      int k = b + 1;
      ll t = dm + m[i];
      cout << res[t % k] + 1 << endl;
    } else {
      int k = b - 1;
      if(k == 0) {
        cout << 0 << endl;
        continue;
      }
      ll t = dl - l[i];
      cout << res[t % k] + 1 << endl;
    }
  }
  return 0;
}
