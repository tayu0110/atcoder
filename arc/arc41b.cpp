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
  int n,m;
  cin >> n >> m;
  vector<string> bstr(n);
  for(int i=0;i<n;i++) {
    cin >> bstr[i];
  }
  vector<vector<int>> b(n, vector<int>(m, 0));
  for(int i=0;i<n;i++)for(int j=0;j<m;j++) b[i][j] = bstr[i][j] - '0';
  vector<vector<int>> a(n, vector<int>(m, 0));
  for(int i=0;i<n;i++){
    if(i==0 || i==n-1) continue;
    for(int j=0;j<m;j++){
      if(j==0 || j==m-1) continue;
      if(b[i-1][j]==0 || b[i][j-1]==0 || b[i+1][j]==0 || b[i][j+1]==0) continue;
      int minval = min(b[i-1][j], min(b[i][j-1], min(b[i+1][j], b[i][j+1])));
      a[i][j]=minval;
      b[i-1][j]-=minval;
      b[i][j-1]-=minval;
      b[i+1][j]-=minval;
      b[i][j+1]-=minval;
    }
  }
  for(int i=0;i<n;i++){
    for(int j=0;j<m;j++) cout << a[i][j];
    cout << endl;
  }
  return 0;
}
