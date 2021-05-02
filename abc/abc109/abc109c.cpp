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

template<class T>T GCD(T a, T b){if(a<b){T temp=a;a=b;b=temp;}T res=a%b;while(res!=0){a=b;b=res;res=a%b;}return b;}

int main(int argc,char* argv[]){
    cin.tie(0);
    ios::sync_with_stdio(0);
    cout << fixed << setprecision(20);

    int n;
    ll x;
    cin >> n >> x;

    vector<ll> xi(n);
    ll minxi=INF;
    for(int i=0;i<n;i++){
        cin >> xi[i];
        xi[i]=abs(xi[i]-x);
    }
    if(n==1){
        cout << xi[0] << endl;
        return 0;
    }

    ll allgcd=GCD(xi[0],xi[1]);
    for(int i=2;i<n;i++){
        allgcd=GCD(allgcd,xi[i]);
    }

    cout << allgcd << endl;

    return 0;
}
