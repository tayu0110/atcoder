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

#define bil ((ll)1e9)
#define mod ((ll)1e9+7)

int main(int argc,char* argv[]){
    ll n;
    cin >> n;
    
    vector<ll> a(n);
    for(auto &x:a)
        cin >> x;
    
    ll q;
    cin >> q;
    
    vector<pll> bc(q);
    for(auto &x:bc)
        cin >> x.first >> x.second;
    
    

    return 0;
}
