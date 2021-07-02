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

#define bil ((ll)1e9)
#define mod ((ll)1e9+7)

ll pow_mod(ll,ll);

int main(int argc,char* argv[]){
    int n;
    cin >> n;

    vector<int> a(n);
    for(auto &x:a)
        cin >> x;
    
    ll ans=0;
    if(n%2==0){
        ans=pow_mod(2,n/2);
    }else{
        ll x=n/2;
        ans=pow_mod(2,x);
    }

    sort(a.begin(),a.end());
    if(n%2==1){
        a.erase(a.begin());
        n-=1;
    }

    for(int i=0;i<n-1;i+=2){
        if(a[i]!=a[i+1]){
            cout << 0 << endl;
            return 0;
        }
    }

    cout << ans << endl;

    return 0;
}

ll pow_mod(ll x,ll y){
    if(y==0){
        return 1;
    }else if(y%2==0){
        ll t=pow_mod(x,y/2);
        return t*t%mod;
    }else if(y%2==1){
        return pow_mod(x,y-1)*x%mod;
    }

    return 0;
}
