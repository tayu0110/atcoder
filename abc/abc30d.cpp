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

#define DEBUG(var) cout << #var << ": " << var << " ";
#define DEBUG_EN(var) cout << #var << ": " << var << endl;

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
int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n, a;
  cin >> n >> a;
  string k;
  cin >> k;
  int len = k.length();
  vector<int> b(n);
  for(int i=0;i<n;i++) cin >> b[i];
  vector<int> l;
  set<int> ck;
  while(ck.find(a) == ck.end()) {
    ck.insert(a);
    l.push_back(a);
    a = b[a-1];
  }
  int ls = l.size();
  l.push_back(a);
  if(len < 6 && ls > stoi(k)) {
    cout << l[stoi(k)] << endl;
    return 0;
  }
  int d = 0;
  while(l[d] != a) d++;
  int over = 0;
  string ds = to_string(d);
  int dl = ds.length();
  int i = len-1, j = dl-1;
  while((i >= 0 && j >= 0) || over) {
    int kv = k[i]-'0';
    int dv = j < 0 ? 0 : ds[j]-'0';
    kv -= over;
    bool f = false;
    if(kv < 0) {
      f = true;
      kv += 10;
    }
    if(kv >= dv) {
      k[i] = (kv-dv) + '0';
    } else {
      f = true;
      kv = kv+10 - dv;
      k[i] = kv + '0';
    }
    i--;
    j--;
    if(f) over = 1;
    else over = 0;
  }
  int loop = ls-d;
  ll kval = 0;
  for(int i=0;i<len;i++) {
    kval = kval * 10 + (k[i]-'0');
    kval %= loop;
  }
  cout << l[d+kval] << endl;
  return 0;
}
