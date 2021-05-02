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
  ll b, c;
  cin >> b >> c;
  if(b > 0) {
    if(b-c/2 <= -(b-(c-1)/2)) cout << 2*b+(c-2)/2+(c-1)/2+1;
    else cout << b+(c-2)/2-b+c/2-(b-(c-1)/2)+b+(c-1)/2+2 << endl;
    // cout << (b-c/2 <= -(b-(c-1)/2)) << endl;
  } else if(b < 0) {
    if(-b-(c-1)/2 < b+(c-2)/2) cout << -2*b+(c-1)/2+c/2+1;
    else cout << -b+(c-1)/2+b+(c-1)/2+1+b+(c-2)/2-b+c/2+1 << endl;
  } else {
    cout << c << endl;
  }
  return 0;
}
