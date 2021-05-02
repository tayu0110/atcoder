//時間計算量O(N)からO(root(N))への削減
#include<iostream>
#include<string>
#include<vector>
#include<algorithm>
#include<utility>
#include<tuple>
#include<map>
#include<queue>
#include<cstdio>
#include<cstdlib>
#include<cstring>
#include<cmath>

using namespace std;

using ll = long long;
using pii = pair<int,int>;


#define bil ((ll)1e9)
#define mod ((ll)1e9+7)

ll func(ll,ll);

int main(int argc,char* argv[]){
    ll n,s;
    cin >> n >> s;

    if(s==n){
        cout << n+1 << endl;
        return 0;
    }

    for(ll b=2;b<=sqrt((long double)n);b++){
        if(s==func(b,n)){
            cout << b << endl;
            return 0;
        }
    }

    ll p=1,b=0,ans=0;
    for(p; p < sqrt((long double)n); p++){
        if((n-s)%p!=0)
            continue;
        else
            b=(n-s)/p+1;
        if(func(b,n)==s){
            ans=b;
        }
    }
    if(ans!=0)
        cout << ans << endl;
    else
        cout << "-1" << endl;

    return 0;
}

ll func(ll b,ll n){
    if(b<=0)
        return -1;
    if(n<b)
        return n;
    else{
    	return func(b,n/b)+n%b;
    }
}
