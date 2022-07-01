#include <iostream>
#include <vector>

using namespace std;

struct LazySegmentTree {
 private:
  int n;
  vector<long long> node, lazy;
 public:
  void init(int sz) {
    n = 1;
    while(n < sz) n *= 2;
    node.resize(2*n-1); lazy.resize(2*n-1);
  }
  LazySegmentTree(int sz) {
   init(sz);
  }
  LazySegmentTree(vector<long long> &v) {
    int sz = v.size();
    init(sz);
    for(int i = 0; i < sz; i++) node[i+n-1] = v[i];
    // RSQ initialize
    for(int i = n-2; i >= 0; i--) node[i] = node[i*2+1] + node[i*2+2];
    // RMQ initialize
    // for(int i = n-2; i >= 0; i--) node[i] = max(node[i*2+1], node[i*2+2]);
  }
  void eval(int now, int l, int r) {
    if(lazy[now] != 0) {
      node[now] += lazy[now];
      if(r - l > 1) {
        lazy[2*now+1] += lazy[now] / 2;
        lazy[2*now+2] += lazy[now] / 2;
      }
      lazy[now] = 0;
    }
  }
  void evalMax(int now, int l, int r) {
    if(lazy[now] > 0) {
      node[now] = max(node[now], lazy[now]);
      if(r - l > 1) {
        lazy[2*now+1] = max(lazy[now], lazy[2*now+1]);
        lazy[2*now+2] = max(lazy[now], lazy[2*now+2]);
      }
      lazy[now] = 0;
    }
  }
  void add(int a, int b, long long x, int k=0, int l=0, int r=-1) {
    if(r < 0) r = n;
    eval(k, l, r);
    if(b <= l || r <= a) return;
    if(a <= l && r <= b) {
      lazy[k] += (r-l) * x;
      eval(k, l, r);
    } else {
      add(a, b, x, 2*k+1, l, (l+r)/2);
      add(a, b, x, 2*k+2, (l+r)/2, r);
      node[k] = node[2*k+1] + node[2*k+2];
    }
  }
  long long getSum(int a, int b, int k=0, int l=0, int r=-1) {
    if(r < 0) r = n;
    if(b <= l || r <= a) return 0;
    eval(k, l, r);
    if(a <= l && r <= b) return node[k];
    long long res = 0;
    res += getSum(a, b, 2*k+1, l, (l+r)/2);
    res += getSum(a, b, 2*k+2, (l+r)/2, r);
    return res;
  }
  void updateMax(int a, int b, long long x, int now=0, int l=0, int r=-1) {
    if(r < 0) r = n;
    evalMax(now, l, r);
    if(b <= l || r <= a) return;
    if(a <= l && r <= b) {
      lazy[now] = max(lazy[now], x);
      evalMax(now, l, r);
      return;
    }
    updateMax(a, b, x, 2*now+1, l, (l+r)/2);
    updateMax(a, b, x, 2*now+2, (l+r)/2, r);
    node[now] = max(node[2*now+1], node[2*now+2]);
  }
  long long getMax(int a, int b, int now=0, int l=0, int r=-1) {
    if(r < 0) r = n;
    if(b <= l || r <= a) return 0;
    evalMax(now, l, r);
    if(a <= l && r <= b) return node[now];
    long long res = 0;
    res = max(res, getMax(a, b, 2*now+1, l, (l+r)/2));
    res = max(res, getMax(a, b, 2*now+2, (l+r)/2, r));
    return res;
  }
};

int main() {
  int n, k;
  cin >> n >> k;
  vector<int> a(n);
  for(int i=0;i<n;i++) cin >> a[i];
  const int mx = 300010;
  LazySegmentTree st(mx);
  st.updateMax(0, mx, 1);
  for(int i=0;i<n-1;i++) {
    int now = st.getMax(a[i], a[i]+1);
    int l = max(a[i]-k, 0);
    int r = min(a[i]+k+1, mx);
    st.updateMax(l, r, now+1);
  }
  cout << st.getMax(0, mx) << endl;
}