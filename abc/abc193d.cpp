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

vector<int> ap(10, -1);
vector<int> tp(10, -1);

ll pt(string s){
  vector<int> n(10);
  for(int i=0;i<s.size();i++){
    int j=s[i]-'0';
    n[j]++;
  }
  ll res=0;
  for(int i=1;i<10;i++){
    res += (i)*pow(10, n[i]);
  }
  return res;
}

int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  ll k;
  cin >> k;
  ld n = 9*k-8;
  string s, t;
  cin >> s >> t;
  vector<int> rem(10, k);
  for(char a: s) rem[a-'0']--;
  for(char a: t) rem[a-'0']--;
  ld ans = 0;
  for(ll i=1;i<10;i++){
    if(rem[i] == 0) continue;
    ll tak;
    if(tp[i] == -1) {
      s[4] = '0'+i;
      tak = pt(s);
      tp[i] = tak;
    } else {
      tak = tp[i];
    }
    for(ll j=1;j<10;j++){
      if(rem[j] == 0) continue;
      if(i==j && rem[j] <= 1) continue;
      ll aok;
      if(ap[j] == -1) {
        t[4] = '0'+j;
        aok = pt(t);
        ap[j] = aok;
      } else {
        aok = ap[j];
      }
      if(tak <= aok) continue;
      vector<int> r = rem;
      ll u = r[i];
      r[i]--;
      u *= r[j];
      ans += u;
    }
  }
  ans /= n*(n-1);
  cout << ans << endl;
  return 0;
}
