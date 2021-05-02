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

int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n;
  cin >> n;
  set<pii> ck;
  for(int i=0;i<n;i++){
    int m, d;
    string s;
    cin >> s;
    int sl = 0;
    while(s[sl] != '/') sl++;
    m = stoi(s.substr(0, sl));
    d = stoi(s.substr(sl+1));
    ck.insert(make_pair(m, d));
  }
  set<int> nck;
  vector<bool> s(366, false);
  int w = 0;
  bool h[7] = {true, false, false, false, false, false, true};
  for(int m = 1; m < 13; m++) {
    for(int d = 1; d < 32; d++) {
      if(m == 2 && d == 30) break;
      if((m == 4 || m == 6 || m == 9 || m == 11) && d == 31) break;
      if(ck.find(make_pair(m, d)) != ck.end()) nck.insert(w);
      w++;
    }
  }
  for(int i = 0; i < 366; i++) {
    if(h[i%7]) {
      s[i] = true;
      if(nck.find(i) != nck.end()) {
        int l = i;
        while(nck.find(l) != nck.end() && (l < 366)) l++;
        nck.insert(l);
      }
    } else {
      if(nck.find(i) != nck.end()) s[i] = true;
    }
  }
  int now = 0;
  int ans = 0;
  for(int i = 0; i < 366; i++) {
    if(s[i]) now++;
    else {
      ans = max(ans, now);
      now = 0;
    }
  }
  if(now != 0) ans = max(ans, now);
  cout << ans << endl;
  return 0;
}
