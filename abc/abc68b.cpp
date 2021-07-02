#include<iostream>
#include<string>
#include<vector>
#include<algorithm>
#include<utility>
#include<tuple>
#include<map>
#include<queue>
#include<set>
#include<stack>
#include<cstdio>
#include<cstdlib>
#include<cstring>
#include<cmath>

using namespace std;

using ll = long long;
using pii = pair<int, int>;
using pll = pair<ll, ll>;

#define BIL ((ll)1e9)
#define MOD ((ll)1e9+7)
#define INF (1LL<<60)           //1LL<<63でオーバーフロー
#define inf (1<<29)             //1<<29でオーバーフロー

int main(int argc,char* argv[]){
    int n;
    cin >> n;

    if(n<2) cout << 1 << endl;
    else if(n<4)cout << 2 << endl;
    else if(n<8) cout << 4 << endl;
    else if(n<16) cout << 8 << endl;
    else if(n<32) cout << 16 << endl;
    else if(n<64) cout << 32 << endl;
    else cout << 64 << endl;

    return 0;
}
