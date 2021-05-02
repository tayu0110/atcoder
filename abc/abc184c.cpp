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
    ll a,b,c,d;
    cin >> a >> b >> c >> d;
    if(a==c && b==d){
        cout << 0 << endl;
    }else if(a+b==c+d || a-b==c-d || abs(a-c)+abs(b-d)<=3){
        cout << 1 << endl;
    }else{
        ll x[]={3,0,-3,0};
        ll y[]={0,3,0,-3};
        for(int i=0;i<4;i++){
            int na=a+x[i],nb=b+y[i];
            if(na+nb==c+d || na-nb==c-d || abs(na-c)+abs(nb-d)<=3){
                cout << 2 << endl;
                return 0;
            }
        }
        for(int i=-2;i<3;i++){
            for(int j=-2;j<3;j++){
                int na=a+i,nb=b+j;
                if(na+nb==c+d || na-nb==c-d || abs(na-c)+abs(nb-d)<=3){
                    cout << 2 << endl;
                    return 0;
                }
            }
        }
        if(abs(c-a)%2==abs(d-b)%2){
            cout << 2 << endl;
            return 0;
        }
        cout << 3 << endl;
    }
    return 0;
}
