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

    string s;
    cin >> s;
    int q;
    cin >> q;

    int t1=1;
    string front="";
    for(int i=0;i<q;i++){
        int t;
        cin >> t;
        if(t==1){
            if(t1)t1=0;
            else t1=1;
        }else{
            int f;
            char c;
            cin >> f >> c;
            if(f==1){
                if(t1)front+=c;
                else s=s+c;
            }else{
                if(t1)s=s+c;
                else front+=c;
            }
        }
    }

    if(t1){
        reverse(front.begin(),front.end());
        cout << front << s << endl;
    }else{
        reverse(s.begin(),s.end());
        cout << s << front <<  endl;
    }

    return 0;
}
