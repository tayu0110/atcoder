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
    ll x;
    cin >> x;

    if(x==2){
        cout << 2 << endl;
        return 0;
    }

    ll num = 3;
    vector<ll> prime(0);
    prime.push_back(2);
    while(num < x){
        for(ll i=0;i<prime.size()+1;i++){
            if(i==prime.size()){
                prime.push_back(num);
            }
            if(prime[i]*prime[i]>num){
                prime.push_back(num);
                break;
            }
            if(num%prime[i]==0){
                break;
            }
        }
        num++;
    }

    ll nnum=x;
    ll ans=-1;
    while(nnum >= x){
        for(int i=0;i<prime.size()+1;i++){
            if(i==prime.size()){
                ans = nnum;
                break;
            }
            if(prime[i]*prime[i]>nnum){
                ans = nnum;
                break;
            }
            if(nnum%prime[i]==0){
                break;
            }
        }
        if(ans != -1){
            break;
        }
        nnum++;
    }

    cout << ans << endl;

    return 0;
}
