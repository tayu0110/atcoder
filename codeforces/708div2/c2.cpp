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
  Edge() : to(0), weight(0) {}
  Edge(int to, long long weight) : to(to), weight(weight) {}
  Edge(const Edge& e) {
    to = e.to;
    weight = e.weight;
  }
};

using ll = long long;
using ld = long double;
using pii = pair<int, int>;
using pll = pair<ll, ll>;
using Graph = vector<vector<int>>;
using weightedGraph = vector<vector<Edge>>;
using heap = priority_queue<int, vector<int>, greater<int>>;

#define BIL ((ll)1e9)
#define MOD ((ll)1e9+7)
#define INF (1LL<<60)           //1LL<<63でオーバーフロー
#define inf (1<<29)             //1<<29でオーバーフロー

int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int t;
  cin >> t;
  while(t--) {
    ll n, k;
    cin >> n >> k;
    if(n % k == 0) {
      for(int i=0;i<k;i++) cout << n/k << " ";
      cout << endl;
      continue;
    }
    if(n%2 == k%2) {
      for(int i=0;i<k-2;i++) cout << 1 << " ";
      cout << (n-(k-2))/2 << " " << (n-(k-2))/2 << endl;
    } else if(n%2 == 1 && k%2 == 0) {
      cout << 1 << " ";
      k--;
      n--;
      if(k == 3) {
        if(n % 4 == 0) cout << n/2 << " " << n/4 << " " << n/4 << endl;
        else cout << 2 << " " << (n-2)/2 << " " << (n-2)/2 << endl;
      } else {
        do {
          cout << 2 << " ";
          k--;
          n -= 2;
        } while((n-(k-2))/2 % 2 != 0);
        for(int i=0;i<k-2;i++) cout << 1 << " ";
        cout << (n-(k-2))/2 << " " << (n-(k-2))/2 << endl;
      }
    } else {
      if(k == 3) {
        if(n % 4 == 0) cout << n/2 << " " << n/4 << " " << n/4 << endl;
        else cout << 2 << " " << (n-2)/2 << " " << (n-2)/2 << endl;
      } else {
        do {
          cout << 2 << " ";
          k--;
          n -= 2;
        } while((n-(k-2))/2 % 2 != 0);
        for(int i=0;i<k-2;i++) cout << 1 << " ";
        cout << (n-(k-2))/2 << " " << (n-(k-2))/2 << endl;
      }
    }
  }
  return 0;
}
