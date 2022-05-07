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
using tssi = tuple<string, string, int>;
tssi solve(string a, string b, vector<int> &na, vector<int> &nb) {
  int now = (stoi(a) + stoi(b)) % 10;
  while(true) {
    bool ff = false;
    for(int i=9;i<19;i++) {
      bool f = false;
      for(int j=1;j<10;j++) {
        if(i-j >= 10) continue;
        if(!na[j]) continue;
        if(!nb[i-j]) continue;
        f = true;
        now += (i + 1) % 10;
        a += to_string(j);
        b += to_string(i-j);
        na[j]--;
        nb[i-j]--;
        break;
      }
      if(f) {
        ff = true;
        break;
      }
    }
    if(!ff) break;
  }
  if(na[9]) {
    for(int i=0;i<na[9];i++) a += "9";
    na[9] = 0;
  }
  if(nb[9]) {
    for(int i=0;i<nb[9];i++) b += "9";
    nb[9] = 0;
  }
  for(int i=1;i<9;i++) {
    now += i * (na[i] + nb[i]);
    for(int j=0;j<na[i];j++) a += to_string(i);
    for(int j=0;j<nb[i];j++) b += to_string(i);
  }
  now++;
  return make_tuple(a, b, now);
}
int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  string a, b;
  cin >> a >> b;
  vector<int> na(10), nb(10);
  for(int i=0;i<a.length();i++) {
    int t = a[i] - '0';
    na[t]++;
  }
  for(int i=0;i<b.length();i++) {
    int t = b[i] - '0';
    nb[t]++;
  }
  string ra, rb;
  int mn = inf;
  for(int i=1;i<10;i++) {
    if(!na[i]) continue;
    vector<int> nna = na, nnb = nb;
    nna[i]--;
    string nowa = to_string(i);
    string nowb;
    for(int j=10-i;j<10;j++) {
      if(!nnb[j]) continue;
      nowb = to_string(j);
      nnb[j]--;
      break;
    }
    if(nowb.empty()) continue;
    auto [resa, resb, res] = solve(nowa, nowb, nna, nnb);
    if(res < mn) {
      mn = res;
      ra = resa;
      rb = resb;
    }
  }
  if(ra.empty()) {
    for(int i=1;i<10;i++) {
      for(int j=0;j<na[i];j++) ra += to_string(i);
      for(int j=0;j<nb[i];j++) rb += to_string(i);
    }
  }
  reverse(ra.begin(), ra.end());
  reverse(rb.begin(), rb.end());
  cout << ra << endl;
  cout << rb << endl;
  return 0;
}