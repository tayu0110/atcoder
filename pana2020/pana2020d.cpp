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

set<string> ans;

void solve(string str, int nt){
    if(nt==str.size()){
        ans.insert(str);
        return;
    }
    for(int i=0;i<26;i++){
        if('z'-(str[nt]+i)<0)return;
        char c=str[nt]+i,maxc='a';
        for(int j=0;j<=nt;j++){
            maxc=max(maxc,str[j]);
        }
        if(c-maxc>1)return;
        string nxt=str;
        nxt[nt]=c;
        solve(nxt, nt+1);
    }
}

int main(int argc,char* argv[]){
    cin.tie(0);
    ios::sync_with_stdio(0);
    cout << fixed << setprecision(20);
    int n;
    cin >> n;
    string s(n,'a');
    solve(s, 1);
    for(auto it=ans.begin();it!=ans.end();it++){
        string str=*it;
        cout << str << endl;
    }
    return 0;
}
