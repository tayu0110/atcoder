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
    ll h,w;
    cin >> h >> w;

    if(h==1 || w==1){
        cout << 1 << endl;
        return 0;
    }

    ll ans;
    if(h%2==0){
        ans=w*(h/2);
    }else{
        if(w%2==0){
            ans=w*(h/2)+(w/2);
        }else{
            ans=w*(h/2)+(w/2)+1;
        }
    }

    cout << ans << endl;

    return 0;
}
