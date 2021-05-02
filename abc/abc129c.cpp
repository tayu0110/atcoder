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
    int n,m;
    cin >> n >> m;
    set<int> a;
    for(int i=0;i<m;i++){
        int b;
        cin >> b;
        a.insert(b);
    }
    vector<ll> stair(n+1);
    stair[0]=1;
    for(int i=1;i<n+1;i++){
        if(a.find(i)!=a.end())continue;
        if(a.find(i-1)==a.end() && i-1>=0)stair[i]+=stair[i-1];
        if(a.find(i-2)==a.end() && i-2>=0)stair[i]+=stair[i-2];
        stair[i]%=MOD;
    }
    cout << stair[n] << endl;
    return 0;
}
