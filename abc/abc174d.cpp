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
#define INF (1LL<<62)           //1LL<<63でオーバーフロー

int main(int argc,char* argv[]){
    int n;
    cin >> n;
    string s;
    cin >> s;

    int count=0;
    for(int i=0;i<n;i++){
        static int rp=n-1;
        if(s[i]=='W'){
            for(rp;rp>=0;rp--){
                if(i>=rp)break;
                if(s[rp]=='R'){
                    s[i]='R';
                    s[rp]='W';
                    count++;
                    rp--;
                    break;
                }
            }
        }
        if(i>=rp)break;
    }

    cout << count << endl;

    return 0;
}
