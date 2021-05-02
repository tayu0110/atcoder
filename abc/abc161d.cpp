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
    int k;
    cin >> k;
    queue<ll> ans;
    for(int i=1;i<10;i++) ans.push(i);
    for(int i=0;i<k-1;i++){
        ll x=ans.front();
        // cout << "i: " << i << " x: " << x << endl;
        ans.pop();
        if(x%10!=0) ans.push(10*x+x%10-1);
        ans.push(10*x+x%10);
        if(x%10!=9) ans.push(10*x+x%10+1);
    }
    cout << ans.front() << endl;
    return 0;
}

