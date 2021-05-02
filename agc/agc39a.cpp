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
    string s;
    ll k;
    cin >> s >> k;
    vector<vector<ll>> tab(2, vector<ll>(1,-1));
    for(int i=0;i<s.size();i++){
        char c=s[i];
        int len=tab[0].size();
        if((c-'a')!=tab[0][len-1]){
            tab[0].push_back(c-'a');
            tab[1].push_back(1);
        }else{
            tab[1][len-1]++;
        }
    }
    int len=tab[0].size();
    if(len==2){
        ll ans=s.size()*k/2;
        cout << ans << endl;
    }else if(tab[0][1]!=tab[0][len-1]){
        ll ans=0;
        for(int i=1;i<len;i++){
            ans+=tab[1][i]/2;
        }
        cout << ans*k << endl;
    }else{
        ll ans=0;
        for(int i=1;i<len;i++){
            ans+=tab[1][i]/2;
        }
        ll l=tab[1][1],r=tab[1][len-1];
        // for(int i=1;i<len;i++){
        //     cout << tab[1][i] << " ";
        // }
        cout << ans*k-(l/2+r/2-(l+r)/2)*(k-1) << endl;
    }
    return 0;
}
