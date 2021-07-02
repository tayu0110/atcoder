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

    vector<int> a(n+1,0);
    for(int i=1;i<=n;i++){
        cin >> a[i];
    }

    int count=0,pt=1;
    vector<bool> check(n+1,false);
    bool reach=false;
    while(!reach){
        if(a[pt]==pt)break;
        if(check[pt])break;
        check[pt]=true;
        pt=a[pt];
        count++;
        if(pt==2)reach=true;
    }

    if(reach)cout << count << endl;
    else cout << -1 << endl;

    return 0;
}
