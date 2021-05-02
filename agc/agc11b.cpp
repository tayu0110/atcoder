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
    vector<ll> a(n);
    vector<ll> sum(n);
    for(int i=0;i<n;i++){
        cin >> a[i];
    }
    sort(a.begin(),a.end());
    for(int i=0;i<n;i++){
        if(i==0)sum[i]=a[i];
        else sum[i]=sum[i-1]+a[i];
    }
    vector<bool> ans(n,false);
    ans[n-1]=true;
    for(int i=n-2;i>=0;i--){
        ll now=sum[i]*2;
        int l=i,r=n-1;
        do{
            if(now>=a[r] && ans[r]){
                ans[i]=true;
                break;
            }
            if(now<a[l]){
                break;
            }
            if(now>=a[(l+r)/2]){
                l=(l+r)/2;
                now=sum[l]*2;
                if(ans[l]){
                    ans[i]=true;
                    break;
                }
            }else{
                r=(l+r)/2;
            }
        }while(r-l>1);
    }
    int res=0;
    for(int i=0;i<n;i++){
        if(ans[i])res++;
        // cout << "i: " << i << " ans: " << ans[i] << endl;
    }
    cout << res << endl;
    return 0;
}
