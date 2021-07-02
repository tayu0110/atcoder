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
    vector<int> a(n);
    for(auto &x:a)cin >> x;

    vector<ll> ball(n+1,0);
    for(int i=0;i<n;i++){
        int now=a[i];
        ball[now]++;
    }

    vector<ll> conbination(n+1,-1);
    conbination[0]=0;
    ll sum=0;
    for(int i=1;i<n+1;i++){
        ll now=ball[i];
        if(now==0)continue;
        if(conbination[now]==-1)conbination[now]=(now*(now-1))/2;
        sum+=conbination[now];
    }

    for(int i=0;i<n;i++){
        ll now=a[i];
        ll b=ball[now];
        if(conbination[b-1]!=-1){
            cout << sum-conbination[b]+conbination[b-1] << endl;
            continue;
        }
        conbination[b-1]=(b-1)*(b-2)/2;
        cout << sum-conbination[b]+conbination[b-1] << endl;
    }

    return 0;
}
