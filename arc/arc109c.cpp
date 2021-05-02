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
    int n,k;
    string s;
    cin >> n >> k >> s;
    if(n%2==1){
        n*=2;
        s+=s;
    }
    for(int i=0;i<k;i++){
        string ns="";
        for(int j=0;j<s.size();j+=2){
            if(s[j]=='R'){
                if(s[j+1]=='R')ns+='R';
                if(s[j+1]=='S')ns+='R';
                if(s[j+1]=='P')ns+='P';
            }else if(s[j]=='S'){
                if(s[j+1]=='R')ns+='R';
                if(s[j+1]=='S')ns+='S';
                if(s[j+1]=='P')ns+='S';
            }else if(s[j]=='P'){
                if(s[j+1]=='R')ns+='P';
                if(s[j+1]=='S')ns+='S';
                if(s[j+1]=='P')ns+='P';
            }
        }
        // cout << ns << endl;
        ns+=ns;
        s=ns;
    }
    // cout << s << endl;
    cout << s[0] << endl; 
    return 0;
}
