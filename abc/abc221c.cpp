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
  string n;
  cin >> n;
  int len = n.length();
  ll ans = 0;
  for(int i=0;i<(1<<len);i++) {
    string s, t;
    for(int j=0;j<len;j++) {
      if(i & (1<<j)) {
        s += n[j];
      } else {
        t += n[j];
      }
    }
    if(!s.length() || !t.length()) continue;
    if(s[0] == '0' || t[0] == '0') continue;
    sort(s.begin(), s.end(), greater<char>());
    sort(t.begin(), t.end(), greater<char>());
    ll a = stoll(s);
    ll b = stoll(t);
    ans = max(ans, a*b);
  }
  cout << ans << endl;
  return 0;
}
