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
    const ll len=10000000000;
    int n;
    cin >> n;
    string t;
    cin >> t;
    if(n==1){
        if(t=="0"){
            cout << len << endl;
        }else{
            cout << len*2 << endl;
        }
        return 0;
    }else if(n==2){
        if(t=="00"){
            cout << 0 << endl;
        }else if(t=="01"){
            cout << len-1 << endl;
        }else if(t=="10"){
            cout << len << endl;
        }else if(t=="11"){
            cout << len << endl;
        }
        return 0;
    }else{
        bool ck=true;
        string rp=t.substr(0,3);
        for(int i=3;i<n-3;i+=3){
            if(rp!=t.substr(i,3)){
                ck=false;
                break;
            }
        }
        if(!ck){
            cout << 0 << endl;
            return 0;
        }
        if(rp=="110"){
            if(n%3==0){
                cout << len-(n/3)+1 << endl;
            }else if(n%3==1){
                if(t[n-1]!='1'){
                    cout << 0 << endl;
                    return 0;
                }
                cout << len-(n/3) << endl;
            }else{
                if(t[n-1]!='1' || t[n-2]!='1'){
                    cout << 0 << endl;
                    return 0;
                }
                cout << len-(n/3) << endl;
            }
        }else if(rp=="101"){
            if(n%3==0){
                cout << len-2-(n/3-1)+1 << endl;
            }else if(n%3==1){
                if(t[n-1]!='1'){
                    cout << 0 << endl;
                    return 0;
                }
                cout << len-2-(n/3-1)+1 << endl;
            }else{
                if(t[n-1]!='0' || t[n-2]!='1'){
                    cout << 0 << endl;
                    return 0;
                }
                cout << len-1-(n/3)+1 << endl;
            }
        }else if(rp=="011"){
            if(n%3==0){
                cout << len-2-(n/3-1)+1 << endl;
            }else if(n%3==1){
                if(t[n-1]!='0'){
                    cout << 0 << endl;
                    return 0;
                }
                cout << len-1-(n/3)+1 << endl;
            }else{
                if(t[n-1]!='1' || t[n-2]!='0'){
                    cout << 0 << endl;
                    return 0;
                }
                cout << len-2-(n/3)+1 << endl;
            }
        }else{
            cout << 0 << endl;
        }
    }
    return 0;
}
