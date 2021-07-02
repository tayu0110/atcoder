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
  cout << fixed << setprecision(20);
  int a, b;
  cin >> a >> b;
  vector<char> p(10);
  for(int i=0;i<a+b;i++) {
    int q;
    cin >> q;
    q %= 10;
    p[q] = (i < a ? '.' : 'o');
  }
  for(int i=0;i<10;i++) if(p[i] != '.' && p[i] != 'o') p[i] = 'x';
  printf("%c %c %c %c\n", p[7], p[8], p[9], p[0]);
  printf(" %c %c %c\n", p[4], p[5], p[6]);
  printf("  %c %c\n", p[2], p[3]);
  printf("   %c\n", p[1]);
  return 0;
}
