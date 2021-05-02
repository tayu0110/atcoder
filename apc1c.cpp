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
    string res;
    char prev;
    cout << 0 << endl;
    cin >> res;
    if(res == "Vacant") {
        return 0;
    } else if(res == "Male") {
        prev='M';
    } else {
        prev='F';
    }
    int l=0,r=n;
    for(int i=0;i<19;i++){
        int m=(r+l)/2;
        cout << m << endl;
        string res;
        cin >> res;
        char now;
        if(res == "Vacant") {
            return 0;
        } else if(res == "Male") {
            now='M';
        } else {
            now='F';
        }
        if((prev==now)^((m-l)%2==0)) r = m;
        else {
            l = m;
            prev = now;
        }
    }
    return 0;
}
