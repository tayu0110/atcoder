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
    int h,w;
    cin >> h >> w;
    vector<string> s(h);
    for(int i=0;i<h;i++){
        cin >> s[i];
    }
    vector<vector<int>> ans(h, vector<int>(w,0));
    if(s[0][0]=='#')ans[0][0]=1;
    for(int r=0;r<h;r++){
        for(int c=0;c<w;c++){
            if(r==0 && c==0)continue;
            if(r-1>=0 && c-1<0){
                if(s[r][c]=='#'){
                    if(s[r-1][c]=='#')ans[r][c]=ans[r-1][c];
                    else ans[r][c]=ans[r-1][c]+1;
                }else{
                    ans[r][c]=ans[r-1][c];
                }
            }else if(r-1<0 && c-1>=0){
                if(s[r][c]=='#'){
                    if(s[r][c-1]=='#')ans[r][c]=ans[r][c-1];
                    else ans[r][c]=ans[r][c-1]+1;
                }else{
                    ans[r][c]=ans[r][c-1];
                }
            }else{
                if(s[r][c]=='#'){
                    if(s[r-1][c]=='.' && s[r][c-1]=='.')ans[r][c]=min(ans[r-1][c]+1, ans[r][c-1]+1);
                    else if(s[r-1][c]=='.' && s[r][c-1]=='#')ans[r][c]=min(ans[r-1][c]+1, ans[r][c-1]);
                    else if(s[r-1][c]=='#' && s[r][c-1]=='.')ans[r][c]=min(ans[r-1][c], ans[r][c-1]+1);
                    else ans[r][c]=min(ans[r-1][c], ans[r][c-1]);
                }else{
                    ans[r][c]=min(ans[r-1][c], ans[r][c-1]);
                }
            }
            // cout << "r: " << r << " c: " << c << " ans: " << ans[r][c] << endl;
        }
    }
    cout << ans[h-1][w-1] << endl;
    // for(int i=0;i<h;i++){
    //     for(int j=0;j<w;j++){
    //         cout << ans[i][j] << " ";
    //     }
    //     cout << endl;
    // }
    return 0;
}
