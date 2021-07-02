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
    int k;
    cin >> k;

    if(k%2==0 || k%5==0){
        cout << -1 << endl;
        return 0;
    }

    ll nownum;
    if(k%10==1)nownum=k*7;
    else if(k%10==3)nownum=k*9;
    else if(k%10==7)nownum=k*1;
    else if(k%10==9)nownum=k*3;

    nownum=(nownum-7)/10;

    ll count=1;
    while(nownum){
        int i=0;
        for(i;i<10;i++){
            ll num=nownum+k*i;
            if((num)%10==7){
                nownum=(num-7)/10;
                count++;
                i=-1;
                break;
            }
        }
        if(i!=-1){
            cout << -1 << endl;
            return 0;
        }
    }

    cout << count << endl;

    return 0;
}
