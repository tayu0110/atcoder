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
#define INF (1LL<<62)           //1LL<<63でオーバーフロー

int main(int argc,char* argv[]){
    int n;
    cin >> n;

    vector<ll> a(n);
    for(auto &x:a)
        cin >> x;
    
    sort(a.begin(),a.end());

    ll ans=0;
    int count=0;
    for(int i=1;i<n;i++){
        if(i%2==1){
            ans+=a.at(n-1-count);
            count++;
        }else{
            ans+=a.at(n-1-count);
        }
    }

    cout << ans << endl;

    return 0;
}
