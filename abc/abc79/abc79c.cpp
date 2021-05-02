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

    int a=s[0]-'0',b=s[1]-'0',c=s[2]-'0',d=s[3]-'0';
    int ans=0;
    for(int i=0;i<(1<<3);i++){
        int sum=a;
        if((1<<2)&i)sum+=b;
        else sum-=b;

        if((1<<1)&i)sum+=c;
        else sum-=c;
        
        if((1<<0)&i)sum+=d;
        else sum-=d;

        if(sum==7){
            ans=i;
            break;
        }
    }

    string exp="";

    exp+=a+'0';
    if((1<<2)&ans)exp+="+";
    else exp+="-";
    
    exp+=b+'0';
    if((1<<1)&ans)exp+="+";
    else exp+="-";
    
    exp+=c+'0';
    if((1<<0)&ans)exp+="+";
    else exp+="-";

    exp+=d+'0';
    exp+="=7";

    cout << exp << endl;

    return 0;
}
