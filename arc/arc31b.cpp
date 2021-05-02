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
  vector<vector<char>> a(10, vector<char>(10));
  for(int i=0;i<10;i++){
    for(int j=0;j<10;j++) cin >> a[i][j];
  }
  bool flag;
  for(int i=0;i<10;i++){
    for(int j=0;j<10;j++){
      flag = true;
      vector<vector<bool>> ck(10, vector<bool>(10, false));
      queue<pii> nt;
      nt.push(make_pair(i, j));
      while(!nt.empty()) {
        int y = nt.front().first;
        int x = nt.front().second;
        nt.pop();
        if(ck[y][x]) continue;
        ck[y][x] = true;
        if(y-1 >= 0 && a[y-1][x] == 'o') {
          nt.push(make_pair(y-1, x));
        }
        if(y+1 < 10 && a[y+1][x] == 'o') {
          nt.push(make_pair(y+1, x));
        }
        if(x-1 >= 0 && a[y][x-1] == 'o') {
          nt.push(make_pair(y, x-1));
        }
        if(x+1 < 10 && a[y][x+1] == 'o') {
          nt.push(make_pair(y, x+1));
        }
      }
      for(int k=0;k<10;k++){
        for(int l=0;l<10;l++){
          if(a[k][l] == 'o') if(!ck[k][l]) flag = false;
        }
      }
      if(flag) break;
    }
    if(flag) break;
  }
  if(flag) cout << "YES" << endl;
  else cout << "NO" << endl;
  return 0;
}
