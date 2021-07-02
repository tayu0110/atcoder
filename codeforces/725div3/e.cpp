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

ll calc(string str) {
  ll res = 0;
  ll c = 0;
  for(int i=0;i < str.length();i++) {
    if(i+1 < str.length() && str[i] == 'h' && str[i+1] == 'a') {
      c++;
      i++;
    } else {
      res += (c == 0 ? 0 : c - 1);
      c = 0;
    }
  }
  if(c != 0) res += c;
  return res;
}
string parse(string s, map<string, string>& vt, map<string, ll>& haha) {
  s += " ";
  int len = s.length();
  bool fvn = false, fop = false, fs = false, fop2 = false, fs2 = false;
  string vn, op, str, op2, str2;
  string now = "";
  for(int i=0;i<len;i++) {
    if(s[i] == ' ') {
      if(now != "") {
        if(!fvn) {vn = now;fvn = true;}
        else if(!fop) {op = now;fop = true;}
        else if(!fs) {str = now;fs = true;}
        else if(!fop2) {op2 = now;fop2 = true;}
        else if(!fs2) {str2 = now;fs2 = true;}
        now = "";
      }
    } else {
      now += s[i];
    }
  }
  if(op == ":=") {
    ll k = calc(str);
    haha[vn] = k;
    string ns = "";ns += str[0];if(str.length()>1)ns += str[str.length()-1];
    cout << "ns: " << ns << " k: " << k << endl; 
    vt[vn] = ns;
  } else {
    string s1 = vt[str], s2 = vt[str2];
    ll k = haha[str] + haha[str2];
    if(s1[s1.length()-1] == 'h' && s2[0] == 'a') k++;
    haha[vn] = k;
    string ns = "";ns += s1[0];ns += s2[s2.length()-1];
    cout << "ns: " << ns << " k:" << k << endl; 
    vt[vn] = ns;
  }
  return vn;
}
int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int t;
  cin >> t;
  vector<ll> res;
  while(t--) {
    int n;
    cin >> n;
    vector<string> s;
    string none;
    getline(cin, none);
    for(int i=0;i<n;i++) {
      getline(cin, none);
      s.push_back(none);
    }
    map<string, ll> haha;
    map<string, string> vt;
    string vn;
    for(int i=0;i<n;i++) {
      vn = parse(s[i], vt, haha);
    }
    ll ans = max(haha[vn]-1, 0LL);
    res.push_back(ans);
  }
  for(auto e: res) cout << e << endl;
  return 0;
}
