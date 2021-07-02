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

#define BIL ((ll)1e9)
#define MOD ((ll)1e9+7)
#define INF (1LL<<60)           //1LL<<63でオーバーフロー
#define inf (1<<29)             //1<<29でオーバーフロー

int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n;
  cin >> n;
  vector<int> s(0);
  for(int i=0;i<n;i++) {
    int a;
    cin >> a;
    if(a != 0) s.push_back(a);
  }
  n = s.size();
  int q;
  cin >> q;
  vector<int> k(q);
  for(int i=0;i<q;i++) {
    cin >> k[i];
  }
  sort(s.begin(), s.end(), greater<int>());
  for(int i=0;i<q;i++) {
    int j = k[i];
    if(k[i] >= n) {
      cout << 0 << endl;
    } else {
      cout << s[j] + 1 << endl;
    }
  }
  return 0;
}
