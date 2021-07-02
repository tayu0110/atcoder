#include<iostream>
#include<string>
#include<vector>
#include<algorithm>
#include<utility>
#include<tuple>
#include<map>
#include<queue>
#include<set>
#include<stack>
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
    int n;
    cin >> n;

    vector<int> x(n);
    int sum=0;
    for(auto &y:x){
        cin >> y;
        sum+=y;
    }

    int p=sum/n;
    ll ansx=0;
    for(int i=0;i<n;i++){
        int diff=abs(x[i]-p);
        ansx+=diff*diff;
    }

    p++;
    ll ansy=0;
    for(int i=0;i<n;i++){
        int diff=abs(x[i]-p);
        ansy+=diff*diff;
    }

    ll ans=min(ansx,ansy);
    
    cout << ans << endl;

    return 0;
}
