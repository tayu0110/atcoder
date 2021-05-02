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
    int n,m;
    cin >> n >> m;
    vector<ll> a(n),b(m);
    for(int i=0;i<n;i++){
        cin >> a[i];
    }
    for(int i=0;i<m;i++){
        cin >> b[i];
    }
    if(n<m){
        vector<ll> diff(n);
        for(int i=0;i<n;i++){
            for(int j=i;j<m;j++){
                if(a[i]==b[j]){
                    diff[i]=j-i;
                    break;
                }
            }
        }
        int len=m-n;
        int ans=0;
        for(int i=0;i<n;i++){
            if(diff[i]>len)continue;
            int mi=0,l=0;
            for(int j=i+1;j<n;j++){
                if(diff[i]>diff[j])mi++;
                else if(diff[i]<diff[j] && diff[j]<=len)l++;
                else continue;
            }
            if(l-mi>=0)ans++;
        }
        cout << len+ans << endl;
    }else{
        vector<ll> diff(m);
        for(int i=0;i<m;i++){
            for(int j=i;j<n;j++){
                if(b[i]==a[j]){
                    diff[i]=j-i;
                    break;
                }
            }
        }
        int len=n-m;
        int ans=0;
        for(int i=0;i<m;i++){
            if(diff[i]>len)continue;
            int mi=0,l=0;
            for(int j=i+1;j<m;j++){
                if(diff[i]>diff[j])mi++;
                else if(diff[i]<diff[j] && diff[j]<=len)l++;
                else continue;
            }
            if(l-mi>=0)ans++;
        }
        cout << len+ans << endl;
    }
    return 0;
}
