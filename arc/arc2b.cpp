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

struct SegmentTree {
  int sz;
  vector<int> t;
  SegmentTree(int n) {
    sz = 1;
    while(sz < n) sz *= 2;
    t.assign(2 * sz - 1, 0);
  }
  void update(int idx, int val) {
    idx += sz - 1;
    t[idx] = val;
    while(idx > 0) {
      idx = (idx - 1) / 2;
      if(t[idx] > val) break;
      t[idx] = val;
    }
    return;
  }
  int getMax(int l, int r, int now=0, int a=0, int b=-1) {
    if(now >= t.size()) return 0;
    if(b < 0) b = sz;
    if(l < 0) l = 0;
    if(r > sz) r = sz;
    if(l > b || r < a) return 0;
    if(l <= a && r >= b) return t[now];
    int res = 0;
    res = max(res, getMax(l, r, 2*now+1, a, (a+b)/2));
    res = max(res, getMax(l, r, 2*now+2, (a+b)/2, b));
    return res;
  }
};

class Date {
  int year, month, day;
  void subMonth(int m) {
    month -= m;
    while(month < 0) {
      year--;
      month += 12;
    }
  }
  int endOfManth(int m=-1) {
    if(m < 1) m = month+1;
    int mx;
    if(m == 2) {
      if(isLeap()) mx = 29;
      else mx = 28;
    } else if(m == 4 || m == 6 || m == 9 || m == 11) mx = 30;
    else mx = 31;
    return mx;
  }
public:
  Date(int year, int month, int day)
    : year(year-1), month(month-1), day(day-1) {}
  Date(string year, string month, string day)
    : year(stoi(year)), month(stoi(year)), day(stoi(day)) {}
  Date(const Date &date)
    : year(date.year), month(date.month), day(date.day) {}
  int yearVal() {return year+1;}
  int monthVal() {return month+1;}
  int dayVal() {return day+1;}
  string yearStr() {return to_string(year+1);}
  string monthStr() {return to_string(month+1);}
  string dayStr() {return to_string(day+1);}
  bool isLeap(int y=-1) {
    if(y == -1) y = year+1;
    if(y % 4 != 0) return false;
    if(y % 100 != 0) return true;
    if(y % 400 != 0) return false;
    return true;
  }
  void addYear(int y) {year += y;}
  void addMonth(int m) {
    month += m;
    if(month > 11) {
      addYear(1);
      month -= 12;
    }
  }
  void addDay(int d) {
    day += d;
    int mx = endOfManth(month+1);
    while(day+1 > mx) {
      day -= mx;
      addMonth(1);
      mx = endOfManth(month+1);
    }
  }
  void subDay(int d) {
    day -= d;
    while(day < 0) {
      subMonth(1);
      day += endOfManth(month+1);
    }
  }
  Date &operator++() {
    addDay(1);
    return *this;
  }
  Date &operator--() {
    subDay(1);
    return *this;
  }
};

int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  string s;
  cin >> s;
  string t, u, v;
  int w = 0;
  for(int i=0;i<s.size();i++) {
    if(s[i] == '/') {
      w++;
      continue;
    }
    if(w == 0) t += s[i];
    else if(w == 1) u += s[i];
    else v += s[i];
  }
  int a = stoi(t), b = stoi(u), c = stoi(v);
  Date d(a, b, c);
  while(d.yearVal() % (d.monthVal() * d.dayVal()) != 0) {
    // cout << "y: " << d.yearVal() << " m: " << d.monthVal() << " d: " << d.dayVal() << endl;
    ++d;
  }
  string year = d.yearStr();
  string month = d.monthStr();
  string day = d.dayStr();
  if(month.size() == 1) month = "0" + month;
  if(day.size() == 1) day = "0" + day;
  cout << year << "/" << month << "/" << day << endl;
  return 0;
}
