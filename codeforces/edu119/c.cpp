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
  vector<string> res;
  while(t--) {
    int n, k;
    ll x;
    string s;
    cin >> n >> k >> x >> s;
    vector<ll> v;
    int now = 0;
    int a = 0;
    for(int i=0;i<n;i++) {
      if(s[i] == '*') now++;
      else {
        v.push_back(now * k + 1);
        now = 0;
        a++;
      }
    }
    if(now != 0) v.push_back(now * k + 1);
    vector<ll> p(v.size());
    x--;
    for(int i=v.size()-1;i>=0;i--) {
      p[i] = x % v[i];
      x /= v[i];
    }
    string ans;
    for(int i=0;i<v.size();i++) {
      for(int j=0;j<p[i];j++) ans += 'b';
      ans += 'a';
    }
    if(a != v.size()) ans.pop_back();
    res.push_back(ans);
  }
  for(auto e : res) cout << e << endl;
  return 0;
}
