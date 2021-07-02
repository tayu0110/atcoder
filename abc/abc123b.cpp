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
    int a,b,c,d,e;
    cin >> a >> b >> c >> d >> e;

    int afrac=(a%10==0?10:a%10);
    int bfrac=(b%10==0?10:b%10);
    int cfrac=(c%10==0?10:c%10);
    int dfrac=(d%10==0?10:d%10);
    int efrac=(e%10==0?10:e%10);

    int frac=min(afrac,min(bfrac,min(cfrac,min(dfrac,efrac))));

    a=ceil((double)a/10)*10;
    b=ceil((double)b/10)*10;
    c=ceil((double)c/10)*10;
    d=ceil((double)d/10)*10;
    e=ceil((double)e/10)*10;

    cout << a+b+c+d+e-(10-frac) << endl;

    return 0;
}
