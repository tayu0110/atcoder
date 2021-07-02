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
    string s,t;
    cin >> s >> t;

    int mincount=inf;
    for(int i=0;i<s.size()-t.size()+1;i++){
        int count=0;
        for(int j=i;j<t.size()+i;j++){
            if(s.at(j)!=t.at(j-i)){
                count++;
            }
        }
        mincount=min(mincount,count);
    }

    cout << mincount << endl;

    return 0;
}
