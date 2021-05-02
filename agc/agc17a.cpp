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

ll nCr(ll n, ll r){
    ll ans=1;
    for(int i=0;i<r;i++){
        ans*=n-i;
    }
    for(int i=1;i<r+1;i++){
        ans/=i;
    }
    return ans;
}

int main(int argc,char* argv[]){
    cin.tie(0);
    ios::sync_with_stdio(0);
    cout << fixed << setprecision(20);

    int n, p;
    cin >> n >> p;

    int odd=0,even=0;
    vector<int> a(n);
    for(auto &x:a){
        cin >> x;
        if(x%2==1)odd++;
        else even++;
    }

    if(odd==0){
        if(p==0){
            ll ans=1;
            for(int i=0;i<n;i++){
                ans*=2;
            }
            cout << ans << endl;
        }else{
            cout << 0 << endl;
        }
    }else{
        ll ans=1;
        for(int i=0;i<n-1;i++){
            ans*=2;
        }
        cout << ans << endl;
    }

    // ll evensum=(even==0?1:0);
    // for(int i=1;i<even+1;i++){
    //     evensum+=nCr(even, i);
    // }
    // if(p==0){
    //     ll oddsum=(odd==0?1:0);
    //     for(int i=2;i<odd+1;i+=2){
    //         oddsum+=nCr(odd, i);
    //     }
    //     cout << oddsum * evensum + 1 << endl;
    // }else{
    //     ll oddsum=(odd==0?1:0);
    //     for(int i=1;i<odd+1;i+=2){
    //         oddsum+=nCr(odd,i);
    //     }
    //     cout << (evensum+1)*oddsum << endl;
    //     cout << oddsum << endl;
    //     cout << evensum << endl;
    // }

    // cout << odd << endl;
    // cout << even << endl;

    return 0;
}
