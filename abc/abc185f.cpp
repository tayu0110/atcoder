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

class SegmentTree {
	int sz;
	vector<int> segt;
public:
	SegmentTree(int n) {
		int x = 1;
		while(x < n) x *= 2;
		this->sz = x;
		this->segt.assign(2*x-1, 0);
	}

	void update(int k, int val) {
		k += this->sz - 1;
		this->segt[k] ^= val;
		int i = k;
		while(i > 0) {
			i = (i-1)/2;
			this->segt[i] ^= val;
		}
		return;
	}

	int getSum(int reql, int reqr, int now=0, int l=0, int r=-1) {
    if(now >= this->segt.size()) return 0;
		if(r < 0) r = this->sz;
		int res = 0;
		if(r < reql || reqr < l) return 0;
		if(l >= reql && r <= reqr) return this->segt[now];
		if(r-l < 1) return this->segt[now];
		res ^= this->getSum(reql, reqr, 2*now+1, l, (l+r)/2);
		res ^= this->getSum(reql, reqr, 2*now+2, (l+r)/2, r);
		return res;
	}

	int &operator[](int k) {
		k += this->sz-1;
		return this->segt[k];
	}
	int const &operator[](int k) const {
		k += this->sz-1;
		return this->segt[k];
	}
};

int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n, q;
  cin >> n >> q;
  SegmentTree a(n);
  for(int i=0;i<n;i++){
    int k;
    cin >> k;
    a.update(i, k);
  }
  for(int i=0;i<q;i++){
    int t, x, y;
    cin >> t >> x >> y;
    if(t == 1) {
      x--;
      a.update(x, y);
    } else {
      x--;
      cout << a.getSum(x, y) << endl;
    }
  }
  return 0;
}
