#include <iostream>
#include <iomanip>
#include <string>
#include <vector>
#include <algorithm>
#include <utility>
#include <tuple>
#include <map>
#include <queue>
#include <deque>
#include <set>
#include <stack>
#include <numeric>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <cmath>
#include <cassert>

using namespace std;

#define DEBUG(var) cerr << #var << ": " << (var) << " "
#define DEBUG_EN(var) cerr << #var << ": " << (var) << endl

using ll = long long;
using ld = long double;
using pii = pair<int, int>;
using pll = pair<ll, ll>;
using Graph = vector<vector<int>>;
template<class T> void print_with_space(T p) { for(auto e : p) cerr << e << " "; cerr << endl; }

const ll MOD = 1e9 + 7;
const ll INF = 1LL << 60;
const int inf = 1 << 29;
const ld PI = acos(-1);
template<class T>
struct heap {
  priority_queue<T, vector<T>, greater<T>> pq;
  heap() : pq() {}
  heap(priority_queue<T, vector<T>, greater<T>> pq) : pq(pq) {}
  void push(T c) { pq.push(c); }
  T top() { return pq.top(); }
  void pop() { pq.pop(); }
  bool empty() { return pq.empty(); }
  int size() { return pq.size(); }
  void swap(heap<T> nt) { pq.swap(nt.pq); }
};
int main(int argc, char* argv[]){
  cout << fixed << setprecision(20);
  int q;
  cin >> q;
  ll x = 0, nb = 0, sum = 0, nl = 0, nr = 0;
  pll mid = {-INF, -INF};
  priority_queue<pll> l;
  heap<pll> r;
  while(q--) {
    int t;
    cin >> t;
    if(t == 1) {
      ll a, b;
      cin >> a >> b;
      nb += b;
      if(mid == make_pair(-INF, -INF)) {
        mid = {a, 1};
        x = a;
      } else {
        if(mid.first == a) {
          mid.second++;
        } else {
          sum += abs(a-x);
          if(a < x) {
            l.push({a, 1});
            nl++;
            while(nl >= mid.second + nr) {
              pll tmp = l.top(); l.pop();
              while(!l.empty() && tmp.first == l.top().first) {
                tmp.second += l.top().second;
                l.pop();
              }
              r.push(mid);
              nr += mid.second;
              sum -= abs(tmp.first-mid.first) * (nl-nr);
              nl -= tmp.second;
              mid = tmp;
            }
          } else {
            r.push({a, 1});
            nr++;
            while(nr > mid.second + nl) {
              pll tmp = r.top(); r.pop();
              while(!r.empty() && tmp.first == r.top().first) {
                tmp.second += r.top().second;
                r.pop();
              }
              l.push(mid);
              nl += mid.second;
              sum -= abs(tmp.first-mid.first) * (nr-nl);
              nr -= tmp.second;
              mid = tmp;
            }
          }
          x = mid.first;
        }
      }
      // DEBUG(x);DEBUG(nl);DEBUG_EN(nr);
    } else {
      // DEBUG(x);DEBUG(sum);DEBUG_EN(nb);
      cout << x << " " << sum + nb << endl;
    }
  }
  return 0;
}
