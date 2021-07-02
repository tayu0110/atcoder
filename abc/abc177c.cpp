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

    vector<ll> a(n);
    ll sum =0;
    for(auto &x:a){
        cin >> x;
        sum += x;
    }

    ll summod=0;
    for(int i=0;i<n;i++){
        sum-=a[i];
        summod+=a[i]*(sum%MOD);
        summod%=MOD;
    }

    cout << summod << endl;

    return 0;
}
