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

    vector<ll> a(n+1,0),b(n,0);
    for(auto &x:a)cin >> x;
    for(auto &y:b)cin >> y;

    ll count=0;
    for(int i=0;i<n;i++){
        if(a[i]>=b[i]){
            count+=b[i];
        }else{
            if(a[i]+a[i+1]>=b[i]){
                count+=b[i];
                a[i+1]-=b[i]-a[i];
            }else{
                count+=a[i]+a[i+1];
                a[i+1]=0;
            }
        }
    }

    cout << count << endl;

    return 0;
}
