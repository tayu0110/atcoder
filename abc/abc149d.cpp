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
    int ans=0;
    string prev="";
    for(int i=0;i<n;i++){
        if(i-k<0){
            if(t[i]=='r'){
                ans+=p;
                prev+='p';
            }else if(t[i]=='s'){
                ans+=r;
                prev+='r';
            }else{
                ans+=s;
                prev+='s';
            }
        }else{
            if(t[i]=='r' && prev[i-k]!='p'){
                ans+=p;
                prev+='p';
            }else if(t[i]=='r' && prev[i-k]=='p'){
                prev+='n';
            }else if(t[i]=='s' && prev[i-k]!='r'){
                ans+=r;
                prev+='r';
            }else if(t[i]=='s' && prev[i-k]=='r'){
                prev+='n';
            }else if(t[i]=='p' && prev[i-k]!='s'){
                ans+=s;
                prev+='s';
            }else if(t[i]=='p' && prev[i-k]=='s'){
                prev+='n';
            }
        }
    }
    cout << ans << endl;
    return 0;
}
