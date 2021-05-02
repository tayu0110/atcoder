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

ll modPow(long long a, long long n, long long p) {
  if (n == 0) return 1;
  if (n == 1) return a % p;
  if (n % 2 == 1) return (a * modPow(a, n - 1, p)) % p;
  long long t = modPow(a, n / 2, p);
  return (t * t) % p;
}

int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  ll a, b, c;
  cin >> a >> b >> c;
  ll a1 = a%10;
  if(a1 == 0 || a1 == 1 || a1 == 5 || a1 == 6) {
    cout << a1 << endl;
    return 0;
  }
  if(b == 0 || c == 0) {
    cout << 1 << endl;
  }
  vector<int> ck(1, 0);
  int k=a1;
  while(true) {
    ck.push_back(k);
    k *= a1;
    k %= 10;
    if(k == a1) break;
  }
  ll flag;
  if(a1 == 4 || a1 == 9) {
    flag = modPow(b, c, 2);
    if(flag == 0) flag = 2;
  }
  else {
    flag = modPow(b, c, 4);
    if(flag == 0) flag = 4;
  }
  cout << ck[flag] << endl;
  return 0;
}
