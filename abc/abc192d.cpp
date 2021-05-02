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
  Edge(int to, long long weight) : to(to), weight(weight) {}
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

bool check(ll d, ll m, string x) {
  ll len = x.length();
  ll rem = m;
  for(int j = 0; j < len; j++) {
    ll a = x[j]-'0';
    ll now = 1;
    if(a == 0) continue;
    for(int i=0;i<len-1-j;i++){
      now *= (d+1);
      if(a * now > rem) {
        return false;
      }
    }
    rem -= a*now;
    if(rem < 0) return false;
  }
  return true;
}

int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  string x;
  ll m;
  cin >> x >> m;
  if(x.length() == 1) {
    ll n = stoll(x);
    if(n <= m) cout << 1 << endl;
    else cout << 0 << endl;
    return 0;
  }
  ll len = x.length();
  ll d=0;
  for(int i=0;i<x.size();i++){
    ll a = x[i]-'0';
    d = max(d, a);
  }
  if(!check(d, m, x)) {
    cout << 0 << endl;
    return 0;
  }
  ll ans = powl(m/(ll)(x[0]-'0'), (ld)1/(ld)(len-1));
  // while(true) {
  //   ll now = 1;
  //   bool flag = false;
  //   for(int i=0;i<len-1;i++){
  //     now *= ans;
  //     if(now > m) {
  //       flag = true;
  //       break;
  //     }
  //   }
  //   if(flag) break;
  //   ans++;
  // }
  ans++;
  while(true) {
    ll n = ans;
    ll rem = m;
    bool flag = true;
    for(int i=0;i<x.size();i++){
      ll a = x[i]-'0';
      if(a == 0) continue;
      ll k = 1;
      ll now = 0;
      for(int j=0;j<len-1-i;j++){
        k *= n;
        if(a * k > rem) {
          flag = false;
          break;
        }
      }
      for(int j=0;j<a;j++){
        rem-=k;
        if(rem < 0) {
          flag= false;
          break;
        }
      }
      if(!flag) break;
    }
    if(flag) {
      // cout << "rem: " << rem << endl;
      break;
    }
    ans--;
    if(ans <= d) {
      cout << 0 << endl;
      return 0;
    }
  }
  cout << ans-d << endl;
  return 0;
}
