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
    int k,t;
    cin >> k >> t;
    multiset<int> a;
    for(int i=0;i<t;i++){
        int j;
        cin >> j;
        a.insert(j);
    }
    while(a.size()>1){
        auto b=a.begin(),e=a.end();
        e--;
        int en=*e;
        // cout << "en: " << en << endl;
        while(en>0){
            b=a.begin();
            // cout << "en: " << en << " *b: " << *b << endl;
            if(en-*b>0){
                en-=*b;
                a.erase(b);
            }else if(en-*b==0){
                a.erase(e);
                a.erase(b);
                break;
            }else{
                a.erase(e);
                a.insert(en);
                break;
            }
        }
    }
    if(a.empty()) cout << 0 << endl;
    else{
        auto b=a.begin();
        cout << *b-1 << endl;
    }
    return 0;
}
