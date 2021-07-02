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
    cin >> s;
    ll k;
    cin >> k;
    for(int i=0;i<s.size();i++){
        int diff='z'-s[i]+1;
        if(diff==26)diff=0;
        if(diff<=k){
            s[i]='a';
            k-=diff;
        }else continue;
        if(k==0)break;
    }
    k%=26;
    s[s.size()-1]+=k;
    cout << s << endl;
    return 0;
}
