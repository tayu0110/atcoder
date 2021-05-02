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

    if(n==0){
        cout << 2 << endl;
        return 0;
    }
    if(n==1){
        cout << 1 << endl;
        return 0;
    }

    vector<ll> l(n+1);
    l[0]=2;
    l[1]=1;
    for(int i=2;i<=n;i++){
        l[i]=l[i-1]+l[i-2];
    }

    cout << l[n] << endl;

    return 0;
}
