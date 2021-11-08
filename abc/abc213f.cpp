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
template<class T> void print_with_space(T p) { for(auto e : p) cout << e << " "; cout << endl; }

const ll BIL = 1e9;
const ll MOD = 1e9 + 7;
const ll INF = 1LL << 60;
const int inf = 1 << 29;
const ld PI = 3.141592653589793238462643383;
vector<int> Zalgo(string& s) {
//   int n = s.length();
//   vector<int> z(n);
//   int from = -1, to = -1;
//   for(int i=1;i<n;i++) {
//     int &j = z[i];
//     if(from != -1) j = min(z[i-from], max(0, to-i));
//     while(i+j < n && s[j] == s[i+j]) j++;
//     if(to < i+j) {
//       from = i;
//       to = i+j;
//     }
//     z[0] = n;
//   }
//   return z;
  vector<int> Z(s.size());
  Z[0] = s.size();
  int i = 1, j = 0;
  while(i < s.size()){
      while(i + j < s.size() && s[j] == s[i + j]) j++;
      Z[i] = j;

      if(j == 0){
          i++;
          continue;
      }
      int k = 1;
      while(k < j && k + Z[k] < j){
          Z[i + k] = Z[k];
          k++;
      }
      i += k;
      j -= k;
  }
  return Z;
}
int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n;
  string s;
  cin >> n >> s;
  vector<int> z = Zalgo(s);
  ll sum = 0;
  for(int i=0;i<n;i++) sum += z[i];
  cout << sum << endl;
  return 0;
}
