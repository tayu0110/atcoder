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

string trans(string s) {
  string t;
  for(int i=0;i<5;i++) {
    if(s[i] == ':') t = ":" + t;
    if(s[i] == '1' || s[i] == '0' || s[i]=='8') t = s[i] + t;
    if(s[i] == '2') t = "5" + t;
    if(s[i] == '5') t = "2" + t;
  }
  return t;
}

bool check(string s, int h, int m) {
  for(int i=0;i<5;i++) {
    if(s[i] == '3' || s[i] == '4' || s[i] == '6' || s[i] == '7' || s[i] == '9') return false;
  }
  string t = trans(s);
  int nh = stoi(t.substr(0, 2));
  int nm = stoi(t.substr(3, 2));
  if(nh >= h) return false;
  if(nm >= m) return false;
  return true;
}

int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int t;
  cin >> t;
  for(int i=0;i<t;i++) {
    int h, m;
    cin >> h >> m;
    string s;
    cin >> s;
    if(check(s, h, m)) {
      cout << s << endl;
      continue;
    }
    string ans = "";
    int nh = stoi(s.substr(0, 2));
    int nm = stoi(s.substr(3, 2));
    for(int i=nh; i < h; i++) {
      bool flag = false;
      for(int j=0; j < m; j++) {
        if(i == nh && j <= nm) continue;
        string sh = to_string(i);
        string sm = to_string(j);
        while(sh.length() < 2) sh = "0" + sh;
        while(sm.length() < 2) sm = "0" + sm;
        string t = sh + ":" + sm;
        if(check(t, h, m)) {
          ans = t;
          flag = true;
          break;
        }
      }
      if(flag) break;
    }
    if(ans == "") ans = "00:00";
    cout << ans << endl;
  }
  return 0;
}
