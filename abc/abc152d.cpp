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
    ll tab[10][10];
    for(int i=0;i<10;i++)for(int j=0;j<10;j++)tab[i][j]=0;
    for(int i=1;i<n+1;i++){
        string s=to_string(i);
        int t=s[0]-'0';
        int b=s[s.size()-1]-'0';
        tab[t][b]++;
    }
    ll ans=0;
    for(int i=1;i<10;i++){
        for(int j=1;j<10;j++){
            // cout << tab[i][j] << " ";
            ans+=tab[i][j]*tab[j][i];
        }
        // cout << endl;
    }
    cout << ans << endl;
    return 0;
}
