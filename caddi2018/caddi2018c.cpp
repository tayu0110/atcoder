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
    ll n,p;
    cin >> n >> p;
    if(n==1){
        cout << p << endl;
        return 0;
    }
    ll np=p;
    map<ll,ll> dv;
    dv.insert(make_pair(1,1));
    dv.insert(make_pair(p,1));
    ll count=2;
    while(count<sqrtl(p)+1){
        if(np%count==0){
            if(dv.find(count)!=dv.end()) dv[count]++;
            else dv.insert(make_pair(count,1));
            if(dv.find(np/count)!=dv.end()) dv[np/count]++;
            else dv.insert(make_pair(np/count,1));
            np/=count;
            continue;
        }else{
            count++;
        }
        if(np==1)break;
    }
    ll ans=1;
    for(auto it=dv.begin();it!=dv.end();it++){
        ll d=it->first,num=it->second;
        if(num<n) continue;
        int exp=num/n;
        for(int i=0;i<exp;i++){
            ans*=d;
        }
    }
    cout << ans << endl;
    return 0;
}
