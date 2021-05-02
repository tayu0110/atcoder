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
    vector<string> s(n);
    for(int i=0;i<n;i++){
        cin >> s[i];
        sort(s[i].begin(), s[i].end());
    }
    vector<int> c(26,0);
    for(int i=0;i<s[0].size();i++){
        c[s[0][i]-'a']++;
    }
    for(int i=1;i<n;i++){
        vector<int> nowc(26,0);
        for(int pt=0;pt<s[i].size();pt++){
            nowc[s[i][pt]-'a']++;
        }
        for(int j=0;j<26;j++){
            c[j]=min(c[j],nowc[j]);
        }
    }

    bool flag=false;
    for(int i=0;i<26;i++){
        for(int j=0;j<c[i];j++){
            flag=true;
            cout << (char)(i+'a');
        }
    }
    if(!flag)cout << "" << endl;

    return 0;
}
