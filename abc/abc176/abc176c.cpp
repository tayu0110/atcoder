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

int main(int argc,char* argv[]){
    int n;
    cin >> n;

    vector<ll> a(n);
    for(auto &x:a)
        cin >> x;
    
    ll height=0;
    for(int i=1;i<a.size();i++){
        if(a[i-1]>a[i]){
            height+=a[i-1]-a[i];
            a[i]=a[i-1];
        }
    }

    cout << height << endl;

    return 0;
}
