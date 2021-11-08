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

#define DEBUG(var) cout << #var << ": " << var << " ";
#define DEBUG_EN(var) cout << #var << ": " << var << endl;

struct Edge {
  int to;
  long long weight;
  Edge() : to(0), weight(0) {}
  Edge(int to, long long weight) : to(to), weight(weight) {}
  Edge(const Edge& e) {
    to = e.to;
    weight = e.weight;
  }
  bool operator>(const Edge &e) const { return weight > e.weight; }
  bool operator<(const Edge &e) const { return weight < e.weight; }
  bool operator==(const Edge &e) const { return weight == e.weight; }
  bool operator<=(const Edge &e) const { return weight <= e.weight; }
  bool operator>=(const Edge &e) const { return weight >= e.weight; }
};

using ll = long long;
using ld = long double;
using pii = pair<int, int>;
using pll = pair<ll, ll>;
using Graph = vector<vector<int>>;
using weightedGraph = vector<vector<Edge>>;
using heap = priority_queue<int, vector<int>, greater<int>>;

const ll BIL = 1e9;
const ll MOD = 1e9 + 7;
const ll INF = 1LL << 60;
const int inf = 1 << 29;
const ld PI = 3.141592653589793238462643383;

int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  string s;
  cin >> s;
  string t = s;
  int f = 0, b = 0;
  while(t.length() > 0 && t[0] == '_') {
    t = t.substr(1);
    f++;
  }
  while(t.length() > 0 && t[t.length()-1] == '_') {
    t.pop_back();
    b++;
  }
  if(t.length() == 0 || !islower(t[0])) {
    cout << s << endl;
    return 0;
  }
  int n = t.length();
  bool iscamel = false, issnake = false;
  for(int i=0;i<n;i++) {
    if(t[i] == '_') issnake = true;
    if(isupper(t[i])) iscamel = true;
  }
  if(issnake && iscamel) {
    cout << s << endl;
  } else if(issnake) {
    string ans;
    for(int i=0;i<n;i++) {
      if(i == 0) ans += t[i];
      else {
        if(t[i-1] != '_') {
          if(t[i] == '_') continue;
          else ans += t[i];
        } else {
          if(t[i] == '_') {
            cout << s << endl;
            return 0;
          } else {
            if(islower(t[i])) ans += (char)toupper(t[i]);
            else {
              cout << s << endl;
              return 0;
            }
          }
        }
      }
    }
    for(int i=0;i<f;i++) ans = "_" + ans;
    for(int i=0;i<b;i++) ans += "_";
    cout << ans << endl;
  } else if(iscamel) {
    string ans;
    for(int i=0;i<n;i++) {
      if(isupper(t[i])) {
        ans += "_";
        ans += (char)tolower(t[i]);
      }
      else ans += t[i];
    }
    for(int i=0;i<f;i++) ans = "_" + ans;
    for(int i=0;i<b;i++) ans += "_";
    cout << ans << endl;
  } else {
    cout << s << endl;
  }
  return 0;
}
