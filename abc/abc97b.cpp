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
    int x;
    cin >> x;

    int maxval=1;
    for(int b=1;b<x;b++){
        for(int p=2;p<x;p++){
            int nowval=1;
            for(int i=0;i<p;i++){
                nowval*=b;
            }
            if(nowval>x){
                break;
            }
            maxval=max(maxval,nowval);
        }
        if(maxval==x)break;
    }

    cout << maxval << endl;

    return 0;
}
