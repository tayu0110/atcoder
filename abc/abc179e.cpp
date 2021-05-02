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
  ll n, x, m;
  cin >> n >> x >> m;
  set<ll> ck;
  ll cnt = 0;
  vector<ll> lt(0);
  ll sum = 0;
  while(ck.find(x) == ck.end()) {
    ck.insert(x);
    lt.push_back(x);
    sum += x;
    x = (x*x) % m;
    cnt++;
  }
  ll syclesum = 0;
  ll sycle = 0;
  vector<ll> sc(0);
  for(int i = cnt-1; i >= 0; i--) {
    sycle++;
    syclesum += lt[i];
    sum -= lt[i];
    cnt--;
    sc.push_back(lt[i]);
    if(lt[i] == x) break;
  }
  ll ans = 0;
  if(n >= cnt) {
    ans += sum;
    n -= cnt;
    ans += syclesum * (n/sycle);
    if(n%sycle != 0) {
      for(int i = 1; i <= n%sycle; i++) {
        ans += sc[sycle-i];
      }
    }
  } else {
    for(int i = 0; i < n; i++) {
      ans += lt[i];
    }
  }
  cout << ans << endl;
  return 0;
}
