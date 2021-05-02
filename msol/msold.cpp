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

    vector<int> a(n);
    for(auto &x:a)
        cin >> x;

    vector<bool> pm(n);
    for(int i=0;i<n-1;i++){
        if(a[i+1]-a[i]>=0) pm[i]=true;
        else pm[i]=false;
    }

    pm.at(n-1)=true;

    int stock=0,money=1000;

    if(a[1]>a[0]){
        // 株を買う
            stock+=money/a[0];
            money-=money/a[0]*a[0];
    }
    for(int i=1;i<n-1;i++){
        if(pm[i]==true && pm[i+1]==true && pm[i-1]==true){
            continue;
        }else if(pm[i]==true && pm[i+1]==false && pm[i-1]==true){
            // 株を買う
            stock+=money/a[i];
            money-=money/a[i]*a[i];
        }else if(pm[i]==false && pm[i+1]==false && pm[i-1]==false){
            continue;
        }else if(pm[i]==false && pm[i+1]==true && pm[i-1]==false){
            // 株を売る
            money+=stock*a[i];
            stock=0;
        }else if(pm[i]==true && pm[i+1]==false && pm[i-1]==true){
            // 株を売る
            money+=stock*a[i];
            stock=0;
        }else if(pm[i]==true && pm[i+1]==true && pm[i-1]==false)
    }

    if(stock!=0){
        money+=stock*a[n-1];
        stock=0;
    }

    cout << money << endl;

    return 0;
}
