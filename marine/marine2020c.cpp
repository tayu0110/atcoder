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

int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n, k;
  cin >> n >> k;
  vector<int> a(n);
  for(int i=0;i<n;i++) {
    cin >> a[i];
  }
  vector<int> p(n+1, 0);
  for(int j=0;j<k;j++) {
    for(int i=0;i<n+1;i++) p[i] = 0;
    for(int i=0;i<n;i++) {
      int t = a[i];
      int l = max(i - t, 0);
      int r = min(i + t + 1, n);
      p[l]++;
      if(l != r) p[r]--;
    }
    bool f = true;
    for(int i=1;i<n+1;i++) {
      a[i-1] = p[i-1];
      p[i] += p[i-1];
      if(a[i-1] != n) f = false;
    }
    if(f) break;
  }
  for(int i=0;i<n;i++) {
    cout << a[i];
    if(i != n-1) cout << " ";
  }
  cout << endl;
  return 0;
}
