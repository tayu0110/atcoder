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
    ll n,k;
    cin >> n >> k;

    ll ans;
    for(int i=0;i<n;i++){
        if(i==0){
            ans=k;
            continue;
        }
        ans*=(k-1);
    }

    cout << ans << endl;

    return 0;
}
