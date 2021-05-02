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
        int n,m;
        cin >> n >> m;

        bool flag=false;
        for(int j=0;j<n;j++){
            int a,b,c,d;
            cin >> a >> b >> c >> d;
            if(b==c){
                flag=true;
            }
        }
        if(m%2!=0){
            cout << "NO" << endl;
            continue;
        }
        if(!flag)cout << "NO" << endl;
        else cout << "YES" << endl;
    }

    return 0;
}
