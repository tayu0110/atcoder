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
  int na, nb;
  cin >> na >> nb;
  vector<ll> a(na), b(nb);
  set<ll> ck;
  int dob=0, sig=0;
  for(int i=0;i<na;i++) {
    cin >> a[i];
    ck.insert(a[i]);
  }
  sig = ck.size();
  for(int i=0;i<nb;i++) {
    cin >> b[i];
    if(ck.find(b[i]) != ck.end()) {
      dob++;
      ck.erase(b[i]);
    } else {
      sig++;
    }
  }
  cout << (double)dob/(double)sig << endl;
  return 0;
}
