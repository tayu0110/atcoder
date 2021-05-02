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
    int h,w,k;
    cin >> h >> w >> k;
    vector<string> s(h);
    set<int> ck;
    for(int i=0;i<h;i++){
        cin >> s[i];
        for(int j=0;j<s[i].size();j++){
            if(s[i][j]=='#'){
                ck.insert(i);
                break;
            }
        }
    }
    vector<vector<int>> ans(h, vector<int>(w,0));
    int now=1;
    for(int i=0;i<h;i++){
        bool flag=false;
        for(int j=0;j<w;j++){
            if(ck.find(i)!=ck.end()){
                if(s[i][j]=='#'){
                    if(flag) now++;
                    else flag=true;
                }
                ans[i][j]=now;
            }else{
                now--;
                break;
            }
        }
        now++;
    }
    for(int i=0;i<h;i++){
        for(int j=0;j<w;j++){
            if(ans[i][j]!=0)break;
            if(i-1<0)break;
            ans[i][j]=ans[i-1][j];
        }
    }
    for(int i=h-1;i>=0;i--){
        for(int j=0;j<w;j++){
            if(ans[i][j]!=0)break;
            ans[i][j]=ans[i+1][j];
        }
    }
    for(int i=0;i<h;i++){
        for(int j=0;j<w;j++){
            cout << ans[i][j] << " ";
        }
        cout << endl;
    }
    return 0;
}
