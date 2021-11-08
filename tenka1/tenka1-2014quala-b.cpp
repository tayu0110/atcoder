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
int check(vector<int> &t) {
  int res = 0;
  for(int i=0;i<t.size();i++) res += (t[i] == 0);
  return res;
}
void inc(vector<int> &t, int k, int d) {
  for(int i=0;i<t.size() && k; i++)
    if(t[i] == 0) {
      t[i] += d;
      k--;
    }
}
int dec(vector<int> &t) {
  int res = 0;
  for(int i=0;i<t.size();i++) {
    if(t[i] == 1) res++;
    if(t[i]) t[i]--;
  }
  return res;
}
int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  string s;
  cin >> s;
  ll ans = 0;
  vector<int> k(5), c(5);
  ll sc = 0;
  int tame = 0;
  for(int i=0;i<s.length();i++) {
    int t = check(k);
    if(!tame) {
      if(s[i] == 'N') {
        if(t > 0) {
          ans += 10 + sc / 10;
          inc(k, 1, 7);
          inc(c, 1, 2);
        }
      } else if(s[i] == 'C') {
        if(t > 2) {
          ans += 50 + sc / 10 * 5;
          inc(k, 3, 9);
          inc(c, 1, 4);
          tame = 3;
        }
      }
    }
    tame = max(tame-1, 0);
    dec(k);
    sc += dec(c);
  }
  cout << (ll)ans << endl;
  return 0;
}
