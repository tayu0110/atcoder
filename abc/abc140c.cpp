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
    int n;
    cin >> n;

    vector<int> b(n-1);
    for(auto &x:b)cin >> x;

    vector<int> a(n,0);
    a[0]=b[0];
    a[1]=b[0];
    for(int i=0;i<n-1;i++){
        if(b[i]>=max(a[i],a[i+1])){
            a[i+1]=b[i];
        }else if(b[i]<max(a[i],a[i+1])){
            a[i]=b[i];
            a[i+1]=b[i];
        }
    }

    int sum=0;
    for(int i=0;i<n;i++){
        sum += a[i];
    }

    cout << sum << endl;

    return 0;
}
