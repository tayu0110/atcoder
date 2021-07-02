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
    vector<ll> a(n);
    for(auto &x:a)cin >> x;

    int updown=-2;
    int ans=0;
    for(int i=1;i<n;i++){
        if(a[i]==a[i-1]){
            continue;
        }
        int nowupdown;
        if(a[i]>a[i-1])nowupdown=1;
        else nowupdown=-1;
        if(updown==-2){
            updown=nowupdown;
        }else if(updown!=nowupdown){
            ans++;
            updown=-2;
        }else{
            continue;
        }
    }

    cout << ans+1 << endl;

    return 0;
}
