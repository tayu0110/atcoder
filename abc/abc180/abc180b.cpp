#include<iostream>
#include<iomanip>
#include<string>
#include<vector>
#include<algorithm>
#include<utility>
#include<tuple>
#include<map>
#include<queue>
#include<set>
#include<stack>
#include<numeric>
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
    cin.tie(0);
    ios::sync_with_stdio(0);
    cout << fixed << setprecision(20);

    int n;
    cin >> n;
    long double man=0,euc=0,che=0;
    vector<long double> x(n);
    for(auto &y:x){
        cin >> y;
        man += abs(y);
        che = max(che, abs(y));
    }

    long double distsum =0;
    for(int i=0;i<n;i++){
        distsum+=x[i]*x[i];
    }
    euc=sqrtl(distsum);

    cout << man << endl;
    cout << euc << endl;
    cout << che << endl;

    return 0;
}
