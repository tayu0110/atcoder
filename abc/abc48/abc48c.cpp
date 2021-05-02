//計算過程が見づらく煩雑な時は、変数を何個も置いて、計算過程をたどれるようにする
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

#define bil ((ll)1e9)
#define mod ((ll)1e9+7)

int main(int argc,char* argv[]){
    int n;
    ll x;
    cin >> n >> x;

    vector<ll> a(n);
    for(auto &y:a)
        cin >> y;
    
    ll ans=0;
    for(int i=0;i<n-1;i++){
        if(a[i]+a[i+1]<=x){
            continue;
        }else{
            ll sum=a[i]+a[i+1];
            ans+=sum-x;
            if(a[i+1]-(sum-x)>=0)
                a[i+1]-=sum-x;
            else{
                a[i]-=sum-x-a[i+1];
                a[i+1]=0;
            }
        }
    }

    cout << ans << endl;

    return 0;
}
