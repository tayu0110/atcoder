//bit全探索による部分点解法
//満点解法は動的計画法により、時間計算量をO(XN^3)に圧縮
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
#include<bitset>
#include<cstdio>
#include<cstdlib>
#include<cstring>
#include<cmath>

using namespace std;

using ll = long long;
using pii = pair<int, int>;

#define bil ((ll)1e9)
#define mod ((ll)1e9+7)

int main(int argc,char* argv[]){
    int n,a;
    cin >> n >> a;

    vector<int> x(n);
    for(auto &i:x)
        cin >> i;

    ll ans=0;
    for(ll bit=0;bit<(1<<n);bit++){
        int sum=0,num=0;
        for(int j=0;j<n;j++){
            if(bit & (1<<j)){
                sum+=x.at(j);
                num++;
            }
        }
        if(sum==0 && num==0){
            continue;
        }
        if(sum==a*num)
            ans++;
    }

    cout << ans << endl;

    return 0;
}
