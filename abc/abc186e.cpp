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
    int t;
    cin >> t;
    for(int i=0;i<t;i++){
        ll n,s,k;
        cin >> n >> s >> k;
        if(n%k==0){
            if((n-s)%k==0){
                cout << (n-s)/k << endl;
            }else{
                cout << -1 << endl;
            }
        }else{
            if((n-s)%k==0){
                cout << (n-s)/k << endl;
                continue;
            }
            set<ll> ck;
            while(1){
                ll f,b;
                f=n%k;
                b=k-f;
                if(s%b==0 && (n-s)%f==0){
                    cout << "reached" << endl;
                    cout << min(n/k*(s/b), (n/k+1)*(s/b)) << endl;
                }else if(s%b==0){
                    cout << n/k*(s/b) << endl;
                }else if((n-s)%f==0){
                    cout << (n/k+1)*(s/b) << endl;
                }else{
                    cout << -1 << endl;
                }

            }
        }
    }
    return 0;
}
