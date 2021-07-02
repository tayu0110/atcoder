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
    string s;
    cin >> s;

    int count=0;
    for(int i=0;i<s.size();i++){
        int ncount=0;
        for(int j=i;j<s.size();j++){
            if(s[j]=='A' || s[j]=='G' || s[j]=='C' || s[j]=='T')ncount++;
            else break;
        }
        count=max(count,ncount);
    }

    cout << count << endl;

    return 0;
}
