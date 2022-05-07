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
#include<cassert>

using namespace std;

#define DEBUG(var) cerr << #var << ": " << var << " "
#define DEBUG_EN(var) cerr << #var << ": " << var << endl

using ll = long long;
using ld = long double;
using pii = pair<int, int>;
using pll = pair<ll, ll>;
using Graph = vector<vector<int>>;
template<class T> void print_with_space(T p) { for(auto e : p) cerr << e << " "; cerr << endl; }

const ll MOD = 1e9 + 7;
const ll INF = 1LL << 60;
const int inf = 1 << 29;
const ld PI = 3.141592653589793238462643383;

int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int t;
  cin >> t;
  vector<vector<ll>> res(t);
  for(int idx=0;idx<t;idx++) {
    int n;
    cin >> n;
    vector<ll> a(n);
    for(int i=0;i<n;i++) cin >> a[i];
    vector<ll> b(n+1);
    for(int i=0;i<n;i++) b[a[i]]++;
    stack<ll> rem;
    ll g = 0;
    bool f = true;
    for(int i=0;i<n+1;i++) {
      if(!f) {
        res[idx].push_back(-1);
        continue;
      }
      if(b[i] == 0) {
        res[idx].push_back(g);
        if(rem.empty()) f = false;
        else {
          b[i] = 1;
          b[rem.top()]--;
          g += i - rem.top();
          if(b[rem.top()] == 1) rem.pop();
        }
      } else {
        res[idx].push_back(b[i] + g);
        if(b[i] > 1) rem.push(i);
      }
    }
  }
  for(auto e : res) for(int i=0;i<e.size();i++) {
    cout << e[i];
    if(i == e.size()-1) cout << endl;
    else cout << " ";
  }
  return 0;
}
