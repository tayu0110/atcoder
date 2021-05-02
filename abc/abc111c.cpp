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
    vector<int> eve(n/2), odd(n/2);
    for(int i=0;i<n;i++){
        if(i%2==0)cin >> eve[i/2];
        else cin >> odd[i/2];
    }
    int osum=0,esum=0;
    vector<int> oddnum(100010),evenum(100010);
    for(int i=0;i<odd.size();i++){
        oddnum[odd[i]]++;osum++;
        evenum[eve[i]]++;esum++;
    }
    pii mo=make_pair(0,0),me=make_pair(0,0),so=make_pair(0,0),se=make_pair(0,0);
    for(int i=1;i<100000;i++){
        if(mo.second<oddnum[i]){
            so=mo;
            mo=make_pair(i,oddnum[i]);
        }else if(so.second<oddnum[i]){
            so=make_pair(i,oddnum[i]);
        }
        if(me.second<evenum[i]){
            se=me;
            me=make_pair(i,evenum[i]);
        }else if(se.second<evenum[i]){
            se=make_pair(i,evenum[i]);
        }
    }
    if(me.first!=mo.first) cout << n-me.second-mo.second << endl;
    else {
        cout << n-max(me.second+so.second, mo.second+se.second) << endl;
    }
    return 0;
}
