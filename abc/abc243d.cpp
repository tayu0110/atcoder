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

#include <atcoder/all>

using namespace std;
using namespace atcoder;

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
void mul(string &t) {
  reverse(t.begin(), t.end());
  int carry = 0;
  string res;
  for(auto e : t) {
    int k = (e - '0') * 2;
    res += (char)(((k+carry) % 10) + '0');
    if(k < 10) carry = 0;
    else carry = k / 10;
  }
  if(carry) res += (char)(carry + '0');
  reverse(res.begin(), res.end());
  t = res;
}
void add(string &t) {
  reverse(t.begin(), t.end());
  int carry = 1;
  string res;
  for(auto e : t) {
    int k = (e - '0') + carry;
    res += (char)((k%10) + '0');
    carry = k / 10;
    if(!carry) break;
  }
  if(carry) res += (char)(carry + '0');
  reverse(res.begin(), res.end());
  t = res;
}
void div(string &t) {
  int carry = 0;
  string res;
  for(auto e : t) {
    int k = carry * 10 + (e - '0');
    if(res.length() || k/2) res += (char)((k/2) + '0');
    carry = k & 1;
  }
  t = res;
}
int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n;
  ll x;
  string s;
  cin >> n >> x >> s;
  deque<pair<int, char>> u, lr;
  for(int i=0;i<n;i++) {
    char c = s[i];
    if(c == 'U') {
      if(!lr.empty()) {
        lr.pop_back();
      } else {
        u.push_back({i, 'U'});
      }
    } else if(c == 'L') {
      lr.push_back({i, 'L'});
    } else if(c == 'R') {
      lr.push_back({i, 'R'});
    }
  }
  string t;
  while(!u.empty() || !lr.empty()) {
    if(u.empty()) {
      t += lr.front().second;
      lr.pop_front();
    } else if(lr.empty()) {
      t += u.front().second;
      u.pop_front();
    } else {
      if(u.front().first < lr.front().first) {
        t += u.front().second;
        u.pop_front();
      } else {
        t += lr.front().second;
        lr.pop_front();
      }
    }
  }
  for(auto e : t) {
    if(e == 'U') x /= 2;
    else if(e == 'L') x = x * 2;
    else x = x * 2 + 1;
  }
  cout << x << endl;
  return 0;
}
