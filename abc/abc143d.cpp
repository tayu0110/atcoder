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
    vector<int> l(n+1);
    l[n]=inf;
    for(int i=0;i<n;i++){
        cin >> l[i];
    }
    sort(l.begin(),l.end());
    ll ans=0;
    for(int i=0;i<n;i++){
        for(int j=i+1;j<n;j++){
            int ab=l[i]+l[j];
            int R=n-1,L=R/2;
            while(!(l[L]<ab && l[L+1]>=ab)){
                if(l[L]>=ab){
                    int temp=R;
                    R=L;
                    if((temp-L)/2<1)L--;
                    else L=L-(temp-L)/2;
                }else if(l[L+1]<ab){
                    if((R-L)/2<1)L++;
                    else L=L+(R-L)/2;
                }
                if(L==n)break;
            }
            if(L<=j)continue;
            if(L==n)continue;
            // cout << "i: " << i << " j: " << j << " L: " << L << endl;
            ans+=L-j;
            // cout << "ans: " << ans << endl;
        }
    }
    cout << ans << endl;
    return 0;
}
