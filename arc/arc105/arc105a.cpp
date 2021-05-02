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

    ll a,b,c,d;
    cin >> a >> b >> c >> d;

    ll sum = a+b+c+d;
    bool ans=false;
    for(ll bit = 0; bit < (1<<4); bit++){
        ll ate=0;
        for(int j=1;j<5;j++){
            if(bit & (1<<j)){
                /*digitsが1だと真*/
                if(j==1)ate+=a;
                else if(j==2)ate+=b;
                else if(j==3)ate+=c;
                else if(j==4)ate+=d;
            }
        }
        if(ate==sum-ate)ans=true;
    }

    if(ans)cout << "Yes" << endl;
    else cout << "No" << endl;

    return 0;
}
