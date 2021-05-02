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
#include<memory>
#include<cstdio>
#include<cstdlib>
#include<cstring>
#include<cmath>

using namespace std;

using ll = long long;
using ld = long double;
using pii = pair<int, int>;
using pll = pair<ll, ll>;

#define BIL ((ll)1e9)
#define MOD ((ll)1e9+7)
#define INF (1LL<<60)           //1LL<<63でオーバーフロー
#define inf (1<<29)             //1<<29でオーバーフロー

int main(int argc,char* argv[]){
    cin.tie(0);
    ios::sync_with_stdio(0);
    cout << fixed << setprecision(20);

    shared_ptr<int> p1(new int(2));
    {
        shared_ptr<int> p2 = p1;
        cout << *p2 << endl;
    }
    cout << *p1 << endl;
    cout << sizeof(p1) << endl;

    int *p = new int(30);	// 確保と初期化
    *p += 20;
    cout << *p << endl;
    delete p;		// 解放

    return 0;
}
