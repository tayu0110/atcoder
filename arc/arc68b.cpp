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
    map<int, int> ck;
    for(int i=0;i<n;i++){
        cin >> a[i];
        int m=a[i];
        if(ck.find(m)==ck.end())ck.insert(make_pair(m,1));
        else ck[m]++;
    }
    int ans=0;
    bool flag=false;
    for(auto it=ck.begin();it!=ck.end();it++){
        int num=it->second;
        if(num%2==0){
            it->second=2;
            if(flag){
                ans+=2;
                flag=false;
            }else{
                flag=true;
            }
        }else{
            it->second=1;
            ans++;
        }
    }
    cout << ans << endl;
    return 0;
}
