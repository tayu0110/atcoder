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
    int n,m;
    cin >> n >> m;
    map<ll,ll> a;
    for(int i=0;i<n;i++){
        ll aa;
        cin >> aa;
        if(a.find(aa)==a.end()){
            a.insert(make_pair(aa,1));
        }else{
            a[aa]++;
        }
    }
    vector<pll> cb(m);
    for(int i=0;i<m;i++){
        ll b,c;
        cin >> b >> c;
        cb[i]=make_pair(c,b);
    }
    sort(cb.begin(),cb.end(),greater<pll>());
    for(int i=0;i<m;i++){
        auto it=a.begin();
        ll aa=it->first;
        ll c=cb[i].first,b=cb[i].second;
        // cout << "c: " << c << " b: " << b << endl;
        // cout << "it->first: " << it->first << " it->second: " << it->second << endl;
        if(aa<c){
            while(b>0){
                auto it=a.begin();
                if(it->first>=c)break;
                ll num=it->second;
                if(num>b){
                    if(a.find(c)==a.end()){
                        a.insert(make_pair(c,b));
                        it->second-=b;
                        break;
                    }else{
                        a[c]+=b;
                        it->second-=b;
                        break;
                    }
                }else{
                    if(a.find(c)==a.end()){
                        a.insert(make_pair(c,num));
                        b-=num;
                        a.erase(it);
                    }else{
                        a[c]+=num;
                        b-=num;
                        a.erase(it);
                    }
                }
            }
        }
    }
    ll ans=0;
    for(auto it=a.begin();it!=a.end();it++){
        ll aa=it->first,num=it->second;
        ans+=aa*num;
        // cout << "aa: " << aa << endl;
    }
    cout << ans << endl;
    return 0;
}
