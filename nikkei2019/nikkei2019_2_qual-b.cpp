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
const ll MOD2 = 998244353;

int main(int argc,char* argv[]){
    cin.tie(0);
    ios::sync_with_stdio(0);
    cout << fixed << setprecision(20);
    int n;
    cin >> n;
    map<int,int> d;
    for(int i=0;i<n;i++){
        int m;
        cin >> m;
        if(i==0 && m!=0){
            cout << 0 << endl;
            return 0;
        }
        if(i!=0 && m==0){
            cout << 0 << endl;
            return 0;
        }
        if(d.find(m)==d.end()){
            d.insert(make_pair(m,1));
        }else{
            d[m]++;
        }
    }
    ll ans=1;
    auto it=d.begin();
    int pkey=it->first,pnum=it->second;
    for(++it;it!=d.end();it++){
        int nkey=it->first,nnum=it->second;
        // cout << "pkey: " << pkey << " pnum: " << pnum << " nkey: " << nkey << " nnum: " << nnum << endl;
        if(nkey-pkey!=1){
            cout << 0 << endl;
            return 0;
        }
        for(int i=0;i<nnum;i++){
            ans*=pnum;
            ans%=MOD2;
        }
        ans%=MOD2;
        pkey=nkey;
        pnum=nnum;
    }
    cout << ans << endl;
    return 0;
}
