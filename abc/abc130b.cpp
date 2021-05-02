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
    int n,x;
    cin >> n >> x;
    vector<int> l(n+1);
    for(int i=1;i<n+1;i++){
        cin >> l[i];
    }
    vector<ll> d(n+2);
    d[1]=0;
    int count=1;
    for(int i=2;i<n+2;i++){
        d[i]=d[i-1]+l[i-1];
        if(d[i]<=x)count++;
        else break;
    }
    cout << count << endl;
    return 0;
}
