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
    int q;
    cin >> q;
    vector<int> prime(100005);
    set<int> ck;
    ck.insert(2);
    prime[0]=0;
    prime[1]=0;
    prime[2]=0;
    for(int i=3;i<100001;i++){
        bool flag=true;
        for(auto it=ck.begin();it!=ck.end();it++){
            int p=*it;
            if(i%p==0){
                flag=false;
                break;
            }
            if(p*p>=i) break;
        }
        if(flag){
            ck.insert(i);
            if(ck.find((i+1)/2)!=ck.end()) prime[i]=prime[i-1]+1;
            else prime[i]=prime[i-1];
        }else prime[i]=prime[i-1];
    }
    for(int i=0;i<q;i++){
        int l,r;
        cin >> l >> r;
        if(prime[l]==prime[l-1]) cout << prime[r]-prime[l] << endl;
        else cout << prime[r]-prime[l]+1 << endl;
    }
    return 0;
}
