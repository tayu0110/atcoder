#include<iostream>
#include<iomanip>
#include<string>
#include<vector>
#include<algorithm>
#include<utility>
#include<tuple>
#include<map>
#include<queue>
#include<set>
#include<stack>
#include<numeric>
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
    cin.tie(0);
    ios::sync_with_stdio(0);
    cout << fixed << setprecision(20);

    int n,m;
    cin >> n >> m;

    if(m==0){
        if(n==1){
            cout << 0 << endl;
            return 0;
        }else{
            string s(n,'0');
            s[0]='1';
            cout << s << endl;
            return 0;
        }
    }

    map<int,int> sc;
    for(int i=0;i<m;i++){
        int s,c;
        cin >> s >> c;
        if(sc.find(s)!=sc.end() && sc.at(s)!=c){
            cout << -1 << endl;
            return 0;
        }else{
            sc.insert(make_pair(s,c));
        }
    }

    string ans(n,'0');
    auto it=sc.begin();
    for(int i=1;i<=n;i++){
        if(it==sc.end())break;
        if(it->first!=i){
            if(i==1)ans[i-1]='1';
            continue;
        }else{
            if(i==1 && it->second==0){
                if(n==1)ans[i-1]='0';
                else{
                    cout << -1 << endl;
                    return 0;
                }
            }else{
                ans[i-1]=it->second+'0';
            }
        }
        it++;
    }

    cout << ans << endl;

    return 0;
}
