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

    int wcount=0,bcount=0;
    for(int i=0;i<s.size();i++){
        if(i%2==0 && s[i]=='0')wcount++;
        else if(i%2==1 && s[i]=='1')wcount++;
    }
    for(int i=0;i<s.size();i++){
        if(i%2==0 && s[i]=='1')bcount++;
        else if(i%2==1 && s[i]=='0')bcount++;
    }

    cout << min(wcount, bcount) << endl;

    return 0;
}
