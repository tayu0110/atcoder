#include <iostream>
#include <utility>
#include <map>

using namespace std;

int main() {
  int q;
  cin >> q;

  map<long long, int> mp;
  while(q--) {
    int t;
    cin >> t;
    if(t == 1) {
      int x;
      cin >> x;
      mp[x] += 1;
    } else if(t == 2) {
      int x, c;
      cin >> x >> c;
      mp[x] -= min(mp[x], c);
      if(mp[x] == 0) {
        mp.erase(x);
      }
    } else {
      auto b = mp.begin();
      auto e = mp.end();
      e--;
      cout << e->first - b->first << endl;
    }
  }
}