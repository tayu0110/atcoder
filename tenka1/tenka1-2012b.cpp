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
  string s;
  cin >> s;
  vector<set<int>> ck(4);
  map<char, int> mp;
  mp['S'] = 0;
  mp['H'] = 1;
  mp['D'] = 2;
  mp['C'] = 3;
  char mk;
  int i=0;
  while(i<s.size()){
    char mark = s[i];
    int num = -1;
    string num_str;
    int j;
    for(j=i+1;j<s.size();j++){
      if(s[j] == 'A') {
        num = 1;
        break;
      } else if(s[j] == 'J') {
        num = 11;
        break;
      } else if(s[j] == 'Q') {
        num = 12;
        break;
      } else if(s[j] == 'K') {
        num = 13;
        break;
      } else if(isdigit(s[j])) {
        num_str += s[j];
        continue;
      } else {
        j--;
        break;
      }
    }
    i = j+1;
    if(num == -1) {
      num = stoi(num_str);
    }
    if(num == 1 || num >= 10) {
      ck[mp[mark]].insert(num);
      if(ck[mp[mark]].size() == 5) {
        mk = mark;
        break;
      }
    }
  }
  bool flag = false;
  set<int> ans;
  i = 0;
  while(i<s.size()){
    char mark = s[i];
    int num = -1;
    string num_str;
    int j;
    for(j=i+1;j<s.size();j++){
      if(s[j] == 'A') {
        num = 1;
        break;
      } else if(s[j] == 'J') {
        num = 11;
        break;
      } else if(s[j] == 'Q') {
        num = 12;
        break;
      } else if(s[j] == 'K') {
        num = 13;
        break;
      } else if(isdigit(s[j])) {
        num_str += s[j];
        continue;
      } else {
        j--;
        break;
      }
    }
    i = j+1;
    if(num == -1) {
      num = stoi(num_str);
    }
    if(mark != mk || (num != 1 && num < 10)) {
      if(num == 1) num_str = "A";
      else if(num == 11) num_str = "J";
      else if(num == 12) num_str = "Q";
      else if(num == 13) num_str = "K";
      else num_str = to_string(num);
      cout << mark << num_str;
      flag = true;
    } else {
      ans.insert(num);
      if(ans.size() == 5) {
        break;
      }
    }
  }
  if(!flag) cout << 0;
  cout << endl;
  return 0;
}
