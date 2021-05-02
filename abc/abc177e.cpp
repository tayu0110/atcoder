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
  vector<int> a(n);
  int g=0;
  int maxn = 0;
  for(int i=0;i<n;i++){
    cin >> a[i];
    if(g==0) g=a[i];
    else g=gcd(g, a[i]);
    maxn = max(maxn, a[i]);
  }
  if(g != 1) {
    cout << "not coprime" << endl;
    return 0;
  }
  vector<bool> is_prime(maxn+1, true);
  vector<int> prime(0);
  is_prime[0] = false;
  is_prime[1] = false;
  for(int i=2;i<maxn+1;i++){
    if(!is_prime[i]) continue;
    prime.push_back(i);
    for(int j=2*i;j<maxn+1;j+=i) is_prime[j] = false;
  }
  set<int> ck;
  for(int i=n-1;i>=0;i--) {
    if(is_prime[a[i]]) {
      if(ck.find(a[i]) != ck.end()) {
        cout << "setwise coprime" << endl;
        return 0;
      }
      ck.insert(a[i]);
      continue;
    }
    for(int j=0;(j < prime.size() && prime[j]*prime[j]<=a[i]);j++){
      if(a[i] % prime[j] != 0) continue;
      if(ck.find(prime[j]) != ck.end()) {
        cout << "setwise coprime" << endl;
        return 0;
      }
      ck.insert(prime[j]);
      // cout << "insert: " << prime[j] << endl;
      while(a[i] % prime[j] == 0) a[i] /= prime[j];
      if(a[i] == 1) break;
      if(is_prime[a[i]]) {
        if(ck.find(a[i]) != ck.end()) {
          cout << "setwise coprime" << endl;
          return 0;
        }
        ck.insert(a[i]);
        break;
      }
    }
  }
  cout << "pairwise coprime" << endl;
  return 0;
}
