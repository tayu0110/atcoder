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

int main(int argc,char* argv[]){
    cin.tie(0);
    ios::sync_with_stdio(0);
    cout << fixed << setprecision(20);
    int n;
    cin >> n;
    vector<pair<ll,pll>> ab(n);
    for(int i=0;i<n;i++){
        ll a,b;
        cin >> a >> b;
        ab[i].first=2*a+b;
        ab[i].second.first=a;
        ab[i].second.second=b;
    }
    sort(ab.begin(),ab.end(), greater<pair<ll,pll>>());
    vector<ll> as(n),bs(n);
    for(int i=0;i<n;i++){
        ll a=ab[i].second.first,b=ab[i].second.second;
        if(i==0){
            as[i]=a;
            bs[i]=b;
        }else{
            as[i]=as[i-1]+a;
            bs[i]=bs[i-1]+b;
        }
    }
    int ans=inf;
    for(int i=0;i<n;i++){
        ll tak=as[i]+bs[i];
        ll aok=as[n-1]-as[i];
        if(tak>aok){
            ans=i+1;
            break;
        }
    }
    cout << ans << endl;
    return 0;
}
