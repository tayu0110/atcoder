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
    int h,w;
    cin >> h >> w;
    vector<string> a(h);
    for(int i=0;i<h;i++){
        cin >> a[i];
    }
    int f=0,e=0,odd=0;
    int c[26];
    memset(c,0,sizeof(c));
    for(auto x:a)for(auto y:x)c[y-'a']++;
    for(int i=0;i<26;i++){
        // cout << "i: " << i << " c[i]: " << c[i] << endl;
        if(c[i]%2==1){
            c[i]--;
            odd++;
        }
        int x=c[i]/4;
        int y=c[i]%4;
        f+=x;
        e+=(y==0 ? 0 : 1);
    }
    if(odd>1){
        cout << "No" << endl;
        return 0;
    }else{
        if(h%2==1 && w%2==1){
            if(e<=h/2+w/2) cout << "Yes" << endl;
            else cout << "No" << endl;
        }else if(h%2==0 && w%2==0){
            if(e==0) cout << "Yes" << endl;
            else cout << "No" << endl;
        }else if(h%2==1){
            if(e<=w/2) cout << "Yes" << endl;
            else cout << "No" << endl;
        }else{
            if(e<=h/2) cout << "Yes" << endl;
            else cout << "No" << endl;
        }
    }
    return 0;
}
