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
    vector<string> a(h);
    for(int i=0;i<h;i++){
        cin >> a[i];
    }
    int ans=0;
    for(int i=0;i<h;i++){
        for(int j=0;j<w;j++){
            if(a[i][j]=='#')ans++;
        }
    }
    if(ans==h+w-1)cout << "Possible" << endl;
    else cout << "Impossible" << endl;

    // int wid=0;
    // for(int y=0;y<h;y++){
    //     for(int x=wid;x<w;x++){
    //         if(x==w-1 && y==h-1)break;
    //         if((x+1<w && a[y][x+1]=='#') && (y+1<h && a[y+1][x]=='#')){
    //             cout << "Impossible" << endl;
    //             return 0;
    //         }else if(x+1<w && a[y][x+1]=='#')continue;
    //         else if(y+1<h && a[y+1][x]=='#'){
    //             wid=x;
    //             break;
    //         }else{
    //             cout << "Impossible" << endl;
    //             return 0;
    //         }
    //     }
    // }

    // cout << "Possible" << endl;

    return 0;
}
