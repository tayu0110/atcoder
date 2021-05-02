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
    int n;
    cin >> n;
    vector<ll> a(n),b(n),c(n);
    for(int i=0;i<n;i++)cin >> a[i];
    for(int i=0;i<n;i++)cin >> b[i];
    for(int i=0;i<n;i++)cin >> c[i];
    sort(a.begin(),a.end());
    sort(b.begin(),b.end());
    sort(c.begin(),c.end());
    vector<ll> mid(n);
    for(int i=n-1;i>=0;i--){
        int j;
        bool flag=false;
        if(i==n-1) j=n-1;
        else j=mid[i+1];
        for(j;j>=0;j--){
            if(b[i]>a[j]){
                flag=true;
                mid[i]=j;
                break;
            }
        }
        if(!flag) mid[i]=-1;
    }
    vector<ll> midsum(n);
    for(int i=0;i<n;i++){
        if(i==0) midsum[i]=mid[i]+1;
        else midsum[i]=midsum[i-1]+mid[i]+1;
    }
    vector<ll> und(n);
    ll ans=0;
    for(int i=n-1;i>=0;i--){
        int j;
        if(i==n-1) j=n-1;
        else j=und[i+1];
        for(j;j>=0;j--){
            if(c[i]>b[j]){
                und[i]=j;
                ans+=midsum[j];
                break;
            }
        }
    }
    cout << ans << endl;
    return 0;
}
