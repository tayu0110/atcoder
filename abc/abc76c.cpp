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
    string s,t;
    cin >> s >> t;
    if(s.size()<t.size()){
        cout << "UNRESTORABLE" << endl;
        return 0;
    }
    int ss=s.size(),ts=t.size();
    for(int i=ss-ts;i>=0;i--){
        bool flag=true;
        for(int j=0;j<ts;j++){
            if(s[i+j]==t[j] || s[i+j]=='?')continue;
            else{
                flag=false;
                break;
            }
        }
        if(flag){
            for(int j=0;j<ts;j++){
                s[i+j]=t[j];
            }
            break;
        }
        if(i==0){
            cout << "UNRESTORABLE" << endl;
            return 0;
        }
    }
    for(int i=0;i<ss;i++){
        if(s[i]=='?')cout << 'a';
        else cout << s[i];
    }
    return 0;
}
