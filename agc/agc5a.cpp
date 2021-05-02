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
    string x;
    cin >> x;
    string ans="";
    bool sflag=false;
    for(int i=0;i<x.size();i++){
        if(i+1<x.size() && x[i]=='S' && x[i+1]=='T') i++;
        else ans+=x[i];
        if(ans.size()>=2 && ans[ans.size()-1] =='T' && ans[ans.size()-2]=='S') ans=ans.substr(0,ans.size()-2);
    }
    cout << ans.size() << endl;
    return 0;
}
