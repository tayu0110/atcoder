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

    int n;
    cin >> n;

    vector<int> h(n);
    int minheight=inf;
    for(auto &x:h){
        cin >> x;
        minheight=min(minheight,x);
    }

    for(auto &x:h){
        x-=minheight;
    }

    int ans=minheight;
    for(int i=0;i<n;i++){
        if(i==0){
            ans+=h[i];
        }else{
            if(h[i]>h[i-1]){
                ans+=h[i]-h[i-1];
            }
        }
    }

    cout << ans << endl;

    return 0;
}
