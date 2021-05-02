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

    int n;
    cin >> n;
    vector<int> a(n);
    for(auto &x:a){
        cin >> x;
    }

    ll distsum=abs(a[0]);
    vector<int> dist(n-1);
    for(int i=0;i<n-1;i++){
        dist[i]=abs(a[i+1]-a[i]);
        distsum+=dist[i];
    }
    distsum+=abs(a[n-1]);

    for(int i=0;i<n;i++){
        if(i==0){
            cout << distsum-abs(a[i])-dist[i]+abs(a[i+1]) << endl;
        }else if(i==n-1){
            cout << distsum-abs(a[n-1])-dist[i-1]+abs(a[n-2]) << endl;
        }else{
            cout << distsum-dist[i]-dist[i-1]+abs(a[i+1]-a[i-1]) << endl;
        }
    }

    return 0;
}
