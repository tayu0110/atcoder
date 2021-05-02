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
    set<int> r,g,b;
    for(int i=0;i<n;i++){
        if(s[i]=='R') r.insert(i);
        if(s[i]=='G') g.insert(i);
        if(s[i]=='B') b.insert(i);
    }
    ll ans=0;
    for(auto rit=r.begin();rit!=r.end();rit++){
        for(auto git=g.begin();git!=g.end();git++){
            int R=*rit,G=*git;
            int c=b.size();
            if(R>G)swap(R,G);
            if(b.find(R-(G-R))!=b.end()) c--;
            if(b.find(G+(G-R))!=b.end()) c--;
            if((R+G)%2==0 && b.find((R+G)/2)!=b.end()) c--;
            ans+=c;
        }
    }
    cout << ans << endl;
    return 0;
}
