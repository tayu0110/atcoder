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
    vector<ll> march(5,0);
    vector<string> s(n);
    for(int i=0;i<n;i++){
        cin >> s[i];
        if(s[i][0]=='M')march[0]++;
        else if(s[i][0]=='A')march[1]++;
        else if(s[i][0]=='R')march[2]++;
        else if(s[i][0]=='C')march[3]++;
        else if(s[i][0]=='H')march[4]++;
    }
    ll ans=0;
    for(int i=0;i<5;i++){
        for(int j=i+1;j<5;j++){
            for(int k=j+1;k<5;k++){
                ans+=march[i]*march[j]*march[k];
            }
        }
    }

    cout << ans << endl;

    return 0;
}
