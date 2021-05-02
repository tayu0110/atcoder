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
#include<bitset>
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
#define INF (1LL<<62)           //1LL<<63でオーバーフロー

int main(int argc,char* argv[]){
    int n;
    string x;
    cin >> n >> x;

    ll xnum=0;
    for(int i=0;i<x.size();i++){
        if(x.size()-1-i=='1')  xnum+=(ll)pow(2,(x.size()-1-i));
    }

    vector<int> f(n,0);
    for(int i=0;i<n;i++){
        ll xi=xnum;
        if(x[i]==1)xi-=(int)pow(2,x.size()-1-i);
        else xi+=(int)pow(2,x.size()-1-i);
        ll pc=0;
    }

    return 0;
}

ll popcount(ll xi){
    
}