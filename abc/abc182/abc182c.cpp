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

    string n;
    cin >> n;
    int k=n.size();
    ll nn=stoll(n);
    if(nn%3==0){
        cout << 0 << endl;
        return 0;
    }else{
        int nsum=0;
        for(int i=0;i<k;i++){
            nsum+=n[i]-'0';
        }
        int ans=k;
        for(ll bit = 0; bit < (1<<k); bit++){
            int sum=nsum;
            int count=0;
            for(int i=0;i<k;i++){
                if(bit & (1<<i)){
                    /*digitsが1だと真*/
                    sum-=n[i]-'0';
                    count++;
                }
            }
            if(sum%3==0 && count<ans){
                ans=count;
            }
        }
        if(ans!=k)cout << ans << endl;
        else cout << -1 << endl;
    }


    return 0;
}
