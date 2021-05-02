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
    ll a,b,c;
    cin >> a >> b >> c;

    ll count=0;
    while(a%2==0 && b%2==0 && c%2==0){
        ll na,nb,nc;
        na=b/2+c/2;
        nb=a/2+c/2;
        nc=b/2+a/2;
        count++;

        if(na%2==1 || nb%2==1 || nc%2==1){
            break;
        }

        if(na==nb && nb==nc){
            count=-1;
            break;
        }

        a=na;
        b=nb;
        c=nc;
    }

    cout << count << endl;

    return 0;
}
