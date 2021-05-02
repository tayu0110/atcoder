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

    int t;
    cin >> t;

    for(int i=0;i<t;i++){
        int n;
        cin >> n;

        vector<int> a(n),sum(n);
        for(int j=0;j<n;j++){
            cin >> a[j];
            if(j==0)sum[j]=a[j];
            else sum[j]=a[j]+sum[j-1];
        }
        if(sum[n-1] == 0){
            cout << "NO" << endl;
            continue;
        }
        sort(a.begin(),a.end());
        bool ans=true;
        for(int j=0;j<n;j++){
            if(j==0)sum[j]=a[j];
            else sum[j]=a[j]+sum[j-1];
            if(sum[j]==0){
                for(int k=1;j+k<n;k++){
                    if(sum[j]-a[j]+a[j+k]!=0){
                        sum[j]=a[j+k]-a[j];
                        swap(a[j],a[j+k]);
                        break;
                    }
                    if(j+k==n-1){
                        ans=false;
                        break;
                    }
                }
            }
            if(!ans){
                break;
            }
        }
        if(ans){
            cout << "YES" << endl;
            for(int j=0;j<n;j++){
                cout << a[j] << ' ';
            }
            cout << endl;
        }else{
            cout << "NO" << endl;
        }
    }

    return 0;
}
