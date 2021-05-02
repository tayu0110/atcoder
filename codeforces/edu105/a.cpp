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
  for(int k = 0; k < n; k++) {
    string s;
    cin >> s;
    int a = 0, b = 0, c = 0;
    for(int i=0;i<s.size();i++){
      if(s[i] == 'A') a++;
      if(s[i] == 'B') b++;
      if(s[i] == 'C') c++;
    }
    if(a+b == c) {
      for(int i=0;i<s.size();i++) if(s[i] == 'B') s[i] = 'A';
    }else if(b+c == a) {
      for(int i=0;i<s.size();i++) if(s[i] == 'C') s[i] = 'B';
    }else if(c+a == b) {
      for(int i=0;i<s.size();i++) if(s[i] == 'A') s[i] = 'C';
    }else {
      cout << "NO" << endl;
      continue;
    }
    string t = "";
    string u = "";
    for(int i=0;i<s.size();i++){
      if(t.size() == 0) {
        t += s[i];
        u += s[i];
      } else if(t.size() == 1) {
        u += s[i];
        if(t[0] != s[i]) {
          t += s[i];
          u.pop_back();
          u.pop_back();
        }
      } else {
        u += s[i];
        if(u.size() >= 2) {
          int m = u.size();
          while(u.size() >= 2 && u[m-1] == t[1] && u[m-2] == t[0]) {
            u.pop_back();
            u.pop_back();
          }
        }
      }
    }
    if(u.size() > 0) cout << "NO" << endl;
    else cout << "YES" << endl;
  }
  return 0;
}
