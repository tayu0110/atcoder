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
    vector<ll> x(n),y(n);
    for(int i=0;i<n;i++){
        cin >> x[i] >> y[i];
    }
    bool flag=false;
    for(int i=0;i<n;i++){
        for(int j=i+1;j<n;j++){
            // double l1;
            // if(x[j]-x[i]==0) l1=inf;
            // else l1=(y[j]-y[i])/(x[j]-x[i]);
            for(int k=j+1;k<n;k++){
                // double l2;
                // if(x[k]-x[j]==0)l2=inf;
                // else l2=(y[k]-y[j])/(x[k]-x[j]);
                if((y[j]-y[i])*(x[k]-x[j])==(y[k]-y[j])*(x[j]-x[i])){
                    flag=true;
                    cout << i << " " << j << " " << k << endl;
                    break;
                }
            }
        }
    }
    if(flag)cout << "Yes" << endl;
    else cout << "No" << endl;

    return 0;
}
