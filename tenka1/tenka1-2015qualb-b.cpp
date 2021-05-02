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
};

using ll = long long;
using ld = long double;
using pii = pair<int, int>;
using pll = pair<ll, ll>;
using Graph = vector<vector<int>>;
using weightedGraph = vector<vector<Edge>>;
using heap = priority_queue<int, vector<int>, greater<int>>;

#define BIL ((ll)1e9)
#define MOD ((ll)1e9+7)
#define INF (1LL<<60)
#define inf (1<<29)

bool expr(string &s, int &pos) {
  int n = s.size();
  bool f = false;
  for(pos;pos<n;pos++) {
    if(s[pos] == '{') {
      if(pos+1 < n && s[pos+1] == '}') pos++;
      else expr(s, ++pos);
    } else if(s[pos] == ':') {
      f = true;
      continue;
    } else if(s[pos] == ',') {
      continue;
    } else if(s[pos] == '}') {
      return f;
    }
  }
  return f;
}

int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  string s;
  cin >> s;
  if(s == "{}") {
    cout << "dict" << endl;
    return 0;
  }
  int pos = 1;
  bool res = expr(s, pos);
  if(res) cout << "dict" << endl;
  else cout << "set" << endl;
  return 0;
}
