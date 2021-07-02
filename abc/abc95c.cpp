#include<iostream>
#include<string>
#include<vector>
#include<algorithm>
#include<utility>
#include<tuple>
#include<map>
#include<queue>
#include<set>
#include<stack>
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
    int a,b,c,x,y;
    cin >> a >> b >> c >> x >> y;

    int ans=0;
    if(a+b>2*c){
        if(x>y){
            ans+=y*2*c;
            if(a<2*c){
                ans+=(x-y)*a;
            }else{
                ans+=(x-y)*2*c;
            }
        }else{
            ans+=x*2*c;
            if(b<2*c){
                ans+=(y-x)*b;
            }else{
                ans+=(y-x)*2*c;
            }
        }
    }else{
        ans+=a*x+b*y;
    }

    cout << ans << endl;

    return 0;
}
