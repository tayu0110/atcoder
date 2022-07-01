#include <iostream>
#include <fstream>
#include <random>

using namespace std;

int main() {
  const int n = 1024;
  const int mod = 998244353;

  ofstream ofs("testcase.txt");
  ofs << n << " " << n << " " << n << endl;

  random_device rnd;
  for(int k = 0; k < 2; k++) {
    for(int i = 0; i < n; i++) {
      for(int j = 0; j < n; j++) {
        if(j) ofs << " "; ofs << (unsigned int)rnd() % mod;
      }
      ofs << endl;
    }
  }

  return 0;
}