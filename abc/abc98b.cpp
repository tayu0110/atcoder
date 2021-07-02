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

    int n;
    string s;
    cin >> n >> s;

    int maxval=0;
    for(int i=1;i<n;i++){
        string x=s.substr(0,i),y=s.substr(i);
        vector<bool> alph(26,false);
        for(int j=0;j<x.size();j++){
            char c=x[j];
            alph[c-'a']=true;
        }
        int val=0;
        for(int j=0;j<y.size();j++){
            char c=y[j];
            if(alph[c-'a']){
                val++;
                alph[c-'a']=false;
            }
        }
        maxval=max(maxval,val);
        if(maxval==26)break;
    }

    cout << maxval << endl;

    return 0;
}
