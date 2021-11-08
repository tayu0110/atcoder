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
  int n, k, c;
  cin >> n >> k >> c;
  string s;
  cin >> s;
  vector<int> l(k), r(k);
  int cnt = 0;
  int i = 0;
  while(i < n && cnt < k) {
    if(s[i] == 'o') {
      l[cnt] = i;
      cnt++;
      i += c;
    }
    i++;
  }
  i = n-1;
  cnt = k-1;
  while(i >= 0 && cnt >= 0) {
    if(s[i] == 'o') {
      r[cnt] = i;
      cnt--;
      i -= c;
    }
    i--;
  }
  vector<int> ans;
  for(int i=0;i<k;i++) {
    if(l[i] == r[i]) ans.push_back(l[i]);
  }
  if(ans.size()) for(auto e : ans) cout << e+1 << endl;
  return 0;
}
