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
    if(m>0){
        if(m>n-2){
            cout << -1 << endl;
            return 0;
        }
        for(int i=1;i<n+1;i++){
            if(i>=1 && i<m+2){
                cout << 2*i << " " << 2*i+1 << endl;
            }else if(i==m+2){
                cout << 1 << " " << 2*(m+1)+2 << endl;
            }else{
                cout << 2*i << " " << 2*i+1 << endl;
            }
        }
    }else if(m==0){
        for(int i=1;i<n+1;i++){
            cout << 2*i-1 << " " << 2*i << endl;
        }
    }else{
        cout << -1 << endl;
    }

    return 0;
}
