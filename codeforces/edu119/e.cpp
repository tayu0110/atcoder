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
struct Node {
  Node() : idx{0}, next{NULL} {}
  int idx;
  Node *next;
};
struct linkedList {
  linkedList() : start{NULL}, end{NULL} {}
  Node *start;
  Node *end;
};
int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int q;
  cin >> q;
  int sz = 0;
  vector<linkedList*> p(500010);
  for(int i=0;i<p.size();i++) {
    p[i] = (linkedList *)malloc(sizeof(linkedList));
    p[i]->start = NULL;
    p[i]->end = NULL;
  }
  while(q--) {
    int t;
    int x;
    cin >> t >> x;
    if(t == 1) {
      Node *node = (Node *)malloc(sizeof(Node));
      if(node == NULL) {
        cout << "PANIC!!!!!!" << endl;
        exit(1);
      }
      node->idx = sz;
      node->next = NULL;
      if(p[x]->start == NULL) {
        p[x]->start = node;
        p[x]->end = node;
      } else {
        p[x]->end->next = node;
        p[x]->end = node;
      }
      sz++;
    } else {
      int y;
      cin >> y;
      if(x == y) continue;
      if(p[x]->start == NULL) continue;
      if(p[y]->start == NULL) {
        p[y]->start = p[x]->start;
        p[y]->end = p[x]->end;
      } else {
        p[y]->end->next = p[x]->start;
        p[y]->end = p[x]->end;
      }
      p[x]->start = NULL;
      p[x]->end = NULL;
    }
  }
  vector<int> res(sz);
  for(int i=0;i<p.size();i++) {
    Node *now = p[i]->start;
    while(now != NULL) {
      res[now->idx] = i;
      now = now->next;
    }
  }
  for(int i=0;i<sz;i++) {
    cout << res[i];
    if(i == sz-1) cout << endl;
    else cout << " ";
  }
  return 0;
}
