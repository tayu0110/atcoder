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
#define mod 998244353
#define INF (1LL<<60)           //1LL<<63でオーバーフロー
#define inf (1<<29)             //1<<29でオーバーフロー

// ll modPow(long long a, long long n, long long p) {
//     if (n == 0) return 1;
//     if (n == 1) return a % p;
//     if (n % 2 == 1) return (a * modPow(a, n - 1, p)) % p;
//     long long t = modPow(a, n / 2, p);
//     return (t * t) % p;
// }

int main(int argc,char* argv[]){
    cin.tie(0);
    ios::sync_with_stdio(0);
    cout << fixed << setprecision(20);
    int H,W,k;
    cin >> H >> W >> k;
    vector<vector<char>> g(H, vector<char>(W, ' '));
    for(int i=0;i<k;i++){
        int h,w;
        char c;
        cin >> h >> w >> c;
        h--;w--;
        g[h][w]=c;
    }
    // ll start=modPow(3,H*W-k,mod);
    vector<vector<ll>> ans(H, vector<ll>(W));
    ans[0][0]=1;
    for(int i=0;i<H;i++){
        for(int j=0;j<W;j++){
            if(g[i][j]=='R' && j+1<W){
                ans[i][j+1]+=3*ans[i][j];
                if(i!=H-1 && j+1!=W-1)ans[i][j+1]%=mod;
            } else if(g[i][j]=='D' && i+1<H){
                ans[i+1][j]+=3*ans[i][j];
                if(i+1!=H-1 && j!=W-1)ans[i+1][j]%=mod;
            } else if(g[i][j]=='X'){
                if(i+1<H){
                    ans[i+1][j]+=3*ans[i][j];
                    if(i+1!=H-1 && j!=W-1)ans[i+1][j]%=mod;
                }
                if(j+1<W){
                    ans[i][j+1]+=3*ans[i][j];
                    if(i!=H-1 && j+1!=W-1)ans[i][j+1]%=mod;
                }
            } else {
                if(i+1<H){
                    ans[i+1][j]+=2*ans[i][j];
                    if(i+1!=H-1 && j!=W-1)ans[i+1][j]%=mod;
                }
                if(j+1<W){
                    ans[i][j+1]+=2*ans[i][j];
                    if(i!=H-1 && j+1!=W-1)ans[i][j+1]%=mod;
                }
            }
        }
    }
    if(g[H-1][W-1]!=' ') ans[H-1][W-1]/=3;
    ans[H-1][W-1]%=mod;
    // for(int i=0;i<H;i++){
    //     for(int j=0;j<W;j++){
    //         cout << ans[i][j] << " ";
    //     }
    //     cout << endl;
    // }
    cout << ans[H-1][W-1] << endl;
    return 0;
}
