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
  int n, m;
  cin >> n >> m;
  vector<string> a(2*n);
  for(int i=0;i<2*n;i++) cin >> a[i];
  vector<pii> p(2*n);
  for(int i=0;i<2*n;i++) p[i] = {0, i};
  for(int i=0;i<m;i++) {
    for(int j=0;j<n;j++) {
      int f = p[j*2].second, s = p[j*2+1].second;
      if(a[f][i] == a[s][i]) continue;
      else if(a[f][i] == 'G' && a[s][i] == 'C') p[j*2].first--;
      else if(a[f][i] == 'C' && a[s][i] == 'P') p[j*2].first--;
      else if(a[f][i] == 'P' && a[s][i] == 'G') p[j*2].first--;
      else p[j*2+1].first--;
    }
    sort(p.begin(), p.end());
  }
  for(int i=0;i<2*n;i++) cout << p[i].second+1 << endl;
  return 0;
}
