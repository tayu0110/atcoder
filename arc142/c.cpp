#include <iostream>
using namespace std;
int main() {
  int n;
  cin >> n;

  int root = 3;
  int dist[110];
  for (int i = 1; i <= n; i++) {
    dist[i] = 0;
    if (i == root) {
      continue;
    }
    cout << "? " << root << " " << i << endl;
    int d;
    cin >> d;
    if (d < 0) {
      return 0;
    }
    dist[i] = d;
  }

  int target = 1;
  if (dist[1] <= dist[2]) {
    target = 2;
  }

  bool flag = false;
  for (int i = 1; i <= n; i++) {
    if (i <= 2) {
      continue;
    }

    if (dist[i] + 1 == dist[target]) {
      cout << "? " << i << " " << target << endl;
      int d;
      cin >> d;
      if (d < 0) {
        return 0;
      }
      if (d == 1) {
        root = i;
        flag = true;
        break;
      }
    }
  }

  if (!flag) {
    cout << "! " << 1 << endl;
  } else {
    int to = target == 2 ? 1 : 2;
    cout << "? " << root << " " << to << endl;
    int d;
    cin >> d;
    if (d < 0) {
      return 0;
    }
    cout << "! " << d+1 << endl;
  }
}