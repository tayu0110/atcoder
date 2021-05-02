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

    //Right Answer
    ll n,x,m;
    cin >> n >> x >> m;
    vector<int> id(m,-1);
    vector<int> a;
    int len=0;
    ll tot=0;
    while(id[x]==-1){
        a.push_back(x);
        id[x]=len;
        len++;
        tot+=x;
        x=x*x%m;
    }

    int c=len-id[x];
    ll s=0;
    for(int i=id[x];i<len;i++){
        s+=a[i];
    }

    ll ans=0;
    if(n<=len){
        for(int i=0;i<n;i++){
            ans+=a[i];
        }
    }else{
        ans+=tot;
        n-=len;
        ans+=s*(n/c);
        n%=c;
        for(int i=0;i<n;i++){
            ans+=a[id[x]+i];
        }
    }
    cout << ans << endl;

    //Wrong Answer
    // int count=0;
    // vector<ll> sum(n,0);
    // map<int,int> check;
    // sum[0]=x;
    // check.insert(make_pair(x,count));
    // count++;
    // x=x*x%m;
    // while(check.find(x)==check.end()){
    //     check.insert(make_pair(x,count));
    //     sum[count]=sum[count-1]+x;
    //     x=x*x%m;
    //     count++;
    //     if(count==n)break;
    // }

    // cout << count << endl;

    // ll ans=0;
    // if(count==n){
    //     ans=sum[n-1];
    // }else{
    //     int xpt=check[x];
    //     cout << xpt << endl;
    //     ans+=sum[xpt-1];
    //     cout << ans << endl;
    //     n-=xpt-1;
    //     cout << n << endl;
    //     int rep=n/(count-xpt);
    //     cout << rep << endl;
    //     ans+=(sum[count]+x-sum[xpt])*rep;
    //     cout << sum[count] << endl;
    //     cout << sum[xpt] << endl;
    //     cout << x << endl;
    //     n-=rep*(count-xpt);
    //     ans+=sum[xpt+n]-sum[xpt-1];
    // }

    // cout << ans << endl;

    // Wrong Answer
    // ll n;
    // ll x,m;
    // cin >> n >> x >> m;

    // if(x%m==0){
    //     cout << 0 << endl;
    //     return 0;
    // }

    // ll nowsize=0,sum=0,sum2=0,diff=0,diffsum=0,now=0;
    // set<ll> check;
    // queue<ll> order;
    // while(check.find(x)==check.end() && now<m){
    //     check.insert(x);
    //     order.push(x);
    //     x=x*x%m;
    //     now++;
    //     if(now==m){
    //         x=-1;
    //         break;
    //     }

    // }

    // ll size=order.size();
    // while(!order.empty()){
    //     if(order.front()==x){
    //         nowsize=order.size();
    //         break;
    //     }else{
    //         sum+=order.front();
    //         order.pop();
    //     }
    // }

    // diff=n-(size-nowsize)-(n-(size-nowsize))/nowsize*nowsize;

    // now=0;
    // while(!order.empty()){
    //     sum2+=order.front();
    //     if(now<diff){
    //         diffsum+=order.front();
    //         now++;
    //     }
    //     order.pop();
    // }

    // ll ans=sum+diffsum+sum2*(n-(size-nowsize))/nowsize*nowsize;


    // cout << ans << endl;
    // cout << sum << endl;
    // cout << diffsum << endl;
    // cout << sum2 << endl;
    // cout << (n-(size-nowsize))/nowsize*nowsize << endl;
    // cout << size << endl;
    // cout << nowsize << endl;

    return 0;
}
