#include<iostream>
#include<string>
#include<vector>
#include<algorithm>
#include<utility>
#include<tuple>
#include<map>
#include<queue>
#include<set>
#include<stack>
#include<cstdio>
#include<cstdlib>
#include<cstring>
#include<cmath>

using namespace std;

using ll = long long;
using pii = pair<int, int>;
using pll = pair<ll, ll>;

#define BIL ((ll)1e9)
#define MOD ((ll)1e9+7)
#define INF (1LL<<60)           //1LL<<63でオーバーフロー
#define inf (1<<29)             //1<<29でオーバーフロー

int main(int argc,char* argv[]){
    string s;
    cin >> s;

    int E=0,W=0,S=0,N=0;
    for(int i=0;i<s.size();i++){
        if(s[i]=='E' && E==0)E++;
        if(s[i]=='W' && W==0)W++;
        if(s[i]=='S' && S==0)S++;
        if(s[i]=='N' && N==0)N++;
    }    

    if(abs(N-S)==1 || abs(E-W)==1){
        cout << "No" << endl;
    }else{
        cout << "Yes" << endl;
    }

    return 0;
}
