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

    ll x,y,a,b;
    cin >> x >> y >> a >> b;

    ll count=0, exp;
    for(exp=1;x*exp<y;exp*=a){
        if(exp >= b/(x*(a-1))){
            break;
        }
        count++;
    }
    
    y-=x*exp;
    if(y%b==0){
        count+=y/b-1;
    }else{
        count+=y/b;
    }    
    
    cout << count << endl;

    return 0;
}
