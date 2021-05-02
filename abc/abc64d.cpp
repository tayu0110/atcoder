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
    string s;
    cin >> n >> s;
    int d,x=inf;
    vector<int> num(n+1);
    for(int i=1;i<n+1;i++){
        if(s[i-1]=='(') num[i]=num[i-1]+1;
        else num[i]=num[i-1]-1;
        x=min(x,num[i]);
    }
    d=num[n];
    string ans;
    for(int i=0;i<-x;i++){
        ans+='(';
        d++;
    }
    ans+=s;
    for(int i=0;i<d;i++){
        ans+=')';
    }
    cout << ans << endl;
    // string ans="";
    // int now=0;
    // while(s[now]==')'){
    //     ans+='(';
    //     now++;
    // }
    // ans+=s.substr(0,now);
    // s=s.substr(now);
    // int m=0;
    // for(int i=0;i<s.size();i++){
    //     if(s[i]=='(')m++;
    //     else m--;
    // }
    // if(m>0){
    //     if(s!="")ans+=s;
    //     for(int i=0;i<m;i++){
    //         ans+=')';
    //     }
    //     cout << ans << endl;
    // }else if(m<0){
    //     for(int i=0;i<abs(m);i++){
    //         ans='('+ans;
    //     }
    //     if(s!="")ans+=s;
    //     cout << ans << endl;
    // }else{
    //     if(s!="")ans+=s;
    //     cout << ans << endl;
    // }
    return 0;
}
