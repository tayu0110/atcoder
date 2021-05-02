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
    vector<vector<int>> a(h,vector<int>(w));
    for(int i=0;i<h;i++){
        for(int j=0;j<w;j++){
            cin >> a[i][j];
        }
    }
    int n=0;
    vector<pair<pii,pii>> ans(0);
    for(int i=0;i<h;i++){
        if(i%2==0){
            for(int j=0;j<w;j++){
                if(a[i][j]%2==0)continue;
                if(j+1<w){
                    a[i][j+1]++;
                    a[i][j]--;
                    n++;
                    ans.push_back(make_pair(make_pair(i+1,j+1),make_pair(i+1,j+2)));
                }else if(i+1<h){
                    a[i+1][j]++;
                    a[i][j]--;
                    n++;
                    ans.push_back(make_pair(make_pair(i+1,j+1),make_pair(i+2,j+1)));
                }
            }
        }else{
            for(int j=w-1;j>=0;j--){
                if(a[i][j]%2==0)continue;
                if(j-1>=0){
                    a[i][j-1]++;
                    a[i][j]--;
                    n++;
                    ans.push_back(make_pair(make_pair(i+1,j+1),make_pair(i+1,j)));
                }else if(i+1<h){
                    a[i+1][j]++;
                    a[i][j]--;
                    n++;
                    ans.push_back(make_pair(make_pair(i+1,j+1), make_pair(i+2,j+1)));
                }
            }
        }
    }
    cout << n << endl;
    for(int i=0;i<ans.size();i++){
        cout << ans[i].first.first << " " << ans[i].first.second << " " << ans[i].second.first << " " << ans[i].second.second << endl;
    }
    // cout << endl;
    // for(int i=0;i<h;i++){
    //     for(int j=0;j<w;j++){
    //         cout << a[i][j] << " ";
    //     }
    //     cout << endl;
    // }
    return 0;
}
