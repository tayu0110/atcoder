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
    
    vector<ll> a(n);
    vector<ll> asum(n,0);
    for(int i=0;i<n;i++){
        cin >> a[i];
        if(i!=0)asum[i]=a[i]+asum[i-1];
        else asum[i]=a[i];
    }
    
    ll mindiff=asum[n-1],sum=asum[n-1];
    for(int i=0;i<n;i++){
        mindiff=min(mindiff,abs(sum-2*asum[i]));
    }

    cout << mindiff << endl;

    //Wrong Answer
    // 
    // ll mindiff=INF;
    // int pt=-1;
    // for(int i=0;i<n;i++){
    //     if(mindiff>abs(asum[n-1]-2*asum[i])){
    //         mindiff=abs(asum[n-1]-2*asum[i]);
    //         pt=i;
    //     }else{
    //         break;
    //     }
    //     if(mindiff==0)break;
    // }

    // cout << mindiff << endl;

    // ll sum=asum[n-1],half=asum[pt];

    // if(mindiff==0){
    //     cout << 0 << endl;
    // }else if(sum-2*half>0){
    //     int ans=0,diff=mindiff;
    //     while(diff){
    //         ans+=ceil(diff/2);
    //         sum+=ans;
    //         half+=ans;
    //         diff=sum-2*half;
    //     }
    //     cout << ans << endl;
    // }else{
    //     int ans=0,diff=mindiff;
    //     while(diff){
    //         ans+=ceil(diff/2);
    //         sum-=ans;
    //         half-=ans;
    //         diff=abs(sum-2*half);

    //         cout << diff << endl;
    //     }
    //     cout << ans << endl;
    // }

    return 0;
}
