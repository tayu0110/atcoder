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
  int h, w;
  cin >> h >> w;
  vector<string> s(h);
  for(int i=0;i<h;i++){
    cin >> s[i];
  }
  set<pii> mp;
  vector<vector<bool>> ck(h, vector<bool>(w, false));
  for(int i=0;i<h;i++){
    for(int j=0;j<w;j++){
      if(s[i][j] == '.') continue;
      bool flag=true;
      for(int k=-1;k<2;k++){
        if(i+k < 0 || i+k >= h) continue;
        for(int l=-1;l<2;l++){
          if(j+l < 0 || j+l >= w) continue;
          if(s[i+k][j+l] == '.') {
            flag = false;
            break;
          }
        }
        if(!flag) break;
      }
      if(flag) {
        mp.insert(make_pair(i, j));
        for(int k=-1;k<2;k++){
          if(i+k < 0 || i+k >= h) continue;
          for(int l=-1;l<2;l++){
            if(j+l < 0 || j+l >= w) continue;
            ck[i+k][j+l] = true;
          }
        }
      }
    }
  }
  for(int i=0;i<h;i++){
    for(int j=0;j<w;j++){
      if(ck[i][j]){
        if(s[i][j] == '#') continue;
        else {
          cout << "impossible" << endl;
          return 0;
        }
      } else {
        if(s[i][j] == '.') continue;
        else {
          cout << "impossible" << endl;
          return 0;
        }
      }
    }
  }
  cout << "possible" << endl;
  for(int i=0;i<h;i++){
    for(int j=0;j<w;j++){
      if(mp.find(std::make_pair(i, j)) != mp.end()) {
        cout << '#';
      } else {
        cout << '.';
      }
    }
    cout << endl;
  }
  return 0;
}
