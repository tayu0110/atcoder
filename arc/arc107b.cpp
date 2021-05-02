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

    int n,k;
    cin >> n >> k;
    k=abs(k);
    ll count=0;
    for(int A=2;A<=2*n;A++){
        int B=A-k;
        if(B>=2 && B<=2*n){
            ll aa,ab;
            if(A-1>n){
                aa=2*n-A+1;
                // cout << "reachA" << endl;
            }else{
                aa=A-1;
            }
            if(B-1>n){
                ab=2*n-B+1;
                // cout << "reachB" << endl;
            }else{
                ab=B-1;
            }
            // cout << A << " " << B << endl;
            count+=aa*ab;
        }
    }
    cout << count << endl;

    return 0;
}
