#include<iostream>
#include<iomanip>
#include<string>
#include<vector>
#include<algorithm>
#include<utility>
#include<tuple>
#include<map>
#include<queue>
#include<set>
#include<stack>
#include<numeric>
#include<cstdio>
#include<cstdlib>
#include<cstring>
#include<cmath>

using namespace std;

using ll = long long;
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

    vector<pii> students(n);
    for(auto &x:students)cin >> x.first >> x.second;
    vector<pll> plane(m);
    for(auto &x:plane)cin >> x.first >> x.second;

    vector<ll> ans(n,0);
    for(int i=0;i<n;i++){
        ll maxval=INF;
        int anspt;
        int x=students[i].first,y=students[i].second;
        for(int j=0;j<m;j++){
            if(maxval>abs(plane[j].first-x)+abs(plane[j].second-y)){
                maxval=abs(plane[j].first-x)+abs(plane[j].second-y);
                anspt=j+1;
            }
        }
        ans[i]=anspt;
    }

    for(auto x:ans)cout << x << endl;

    return 0;
}
