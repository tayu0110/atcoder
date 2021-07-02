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
#include<iomanip>
#include<numeric>
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
    cin.tie(0);
    ios::sync_with_stdio(0);
    cout << fixed << setprecision(20);

    string s;
    cin >> s;
    int n=s.size()+1;

    vector<int> a(n);
    for(int i=0;i<n-1;i++){
        if(s[i]=='<'){
            if(a[i+1]<a[i]+1)a[i+1]=a[i]+1;
        }
    }
    for(int i=n-2;i>=0;i--){
        if(s[i]=='>'){
            if(a[i]<a[i+1]+1)a[i]=a[i+1]+1;
        }
    }

    ll sum=0;
    for(int i=0;i<n;i++){
        sum+=a[i];
    }

    cout << sum << endl;

    // Wrong Answer
    // a[0]=0;
    // int begin=0,minus=0,sum=0;
    // for(int i=0;i<s.size();i++){
    //     if(s[i]=='<'){
    //         sum+=a[i]+1;
    //         a[i+1]=a[i]+1;
    //         if(begin!=0){
    //             if(a[begin]<=minus){
    //                 sum=sum-a[begin]+minus+1;
    //             }
    //             begin=0;
    //             minus=0;
    //         }
    //     }else{
    //         if(begin==0){
    //             begin=i;
    //         }
    //         sum+=minus;
    //         minus++;
    //     }
    // }

    // cout << sum << endl;

    return 0;
}
