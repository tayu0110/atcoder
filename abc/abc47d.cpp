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
    ll n,t;
    cin >> n >> t;

    vector<ll> a(n);
    for(auto &x:a)
        cin >> x;
    
    ll maxben=0,ben=0,termmax=a[n-1],ans=0;
    for(ll i=n-2;i>=0;i--){
        if(a[i]>a[i+1]){
            if(termmax<=a[i]){
                termmax=a[i];
            }else
                continue;
        }else{
            ben=termmax-a[i];
            if(maxben<ben){
                maxben=ben;
                ans=0;
                ans++;
            }else if(maxben==ben){
                ans++;
            }else
                continue;
        }
    }

    cout << ans << endl;

    return 0;
}
