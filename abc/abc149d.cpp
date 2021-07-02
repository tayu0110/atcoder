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

    int n,k,r,s,p;
    string t;
    cin >> n >> k >> r >> s >> p >> t;
    queue<int> rf,sf,pf;
    bool rflag=false,sflag=false,pflag=false;
    ll ans=0;
    for(int i=0;i<n;i++){
        if(t[i]=='r'){
            int rp=-inf;
            while(!rf.empty()){
                if(rf.front()>=i-k){
                    rp=rf.front();
                    // cout << "rp: " << rp << endl;
                    rf.pop();
                    break;
                }
                rf.pop();
            }
            if(rp!=i-k)ans+=p;
            else{
                if(rflag){
                   ans+=p;
                   rflag=false; 
                }else rflag=true;
            }
            rf.push(i);
        }else if(t[i]=='s'){
            int sp=-inf;
            while(!sf.empty()){
                if(sf.front()>=i-k){
                    sp=sf.front();
                    // cout << "sp: " << sp << endl;
                    sf.pop();
                    break;
                }
                sf.pop();
            }
            if(sp!=i-k)ans+=r;
            else{
                if(sflag){
                    ans+=r;
                    sflag=false;
                }else sflag=true;
            }
            sf.push(i);
        }else{
            int pp=-inf;
            while(!pf.empty()){
                if(pf.front()>=i-k){
                    pp=pf.front();
                    // cout << "pp: " << pp << endl;
                    pf.pop();
                    break;
                }
                pf.pop();
            }
            if(pp!=i-k)ans+=s;
            else{
                if(pflag){
                    ans+=s;
                    pflag=false;
                }else pflag=true;
            }
            pf.push(i);
        }
        cout << "i: " << i << " " << ans << endl;
    }
    cout << ans << endl;
    return 0;
}
