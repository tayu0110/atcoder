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
  int t;
  cin >> t;
  vector<ll> res;
  while(t--) {
    int n;
    cin >> n;
    vector<ll> b(n);
    for(int i=0;i<n;i++) cin >> b[i];
    set<ll> ck;
    for(int i=0;i<n;i++) ck.insert(b[i]);
    vector<ll> a;
    for(auto e : ck) a.push_back(e);
    reverse(a.begin(), a.end());
    n = a.size();
    int z = 0, o = 0, t = 0;
    for(int i=0;i<n;i++) {
      if(a[i] % 3 == 0) z++;
      else if(a[i] % 3 == 1) o++;
      else t++;
    }
    if(!o && !t) res.push_back(a[0] / 3);
    else if(o && !t) res.push_back(a[0] / 3 + 1);
    else if(!o && t) res.push_back(a[0] / 3 + 1);
    else {
      if(a[0] % 3 == 0) {
        res.push_back(a[0] / 3 + 1);
      } else if(a[0] % 3 == 1) {
        if(a[n-1] < 2) res.push_back(a[0] / 3 + 2);
        else if(n > 1 && a[0] - a[1] == 1) res.push_back(a[0] / 3 + 2);
        else res.push_back(a[0] / 3 + 1);
      } else {
        res.push_back(a[0] / 3 + 2);
      }
    }
  }
  for(auto e : res) cout << e << endl;
  return 0;
}
