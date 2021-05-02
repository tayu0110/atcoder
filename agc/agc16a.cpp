#include<iostream>
#include<iomanip>
#include<string>
#include<vector>
#include<algorithm>
#include<utility>
#include<tuple>
#include<map>
#include<queue>
#include<set>
#include<stack>
#include<numeric>
#include<cstdio>
#include<cstdlib>
#include<cstring>
#include<cmath>

using namespace std;

using ll = long long;
using ld = long double;
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
    int n = s.size();

    set<char> chars;
    for(int i=0;i<n;i++){
        chars.insert(s[i]);
    }

    int ans=inf;
    for(int i=0;i<26;i++){
        if(chars.find('a'+i)==chars.end())continue;
        int nn=n,count=0;
        string ns=s;
        char c='a'+i;
        bool flag=false;
        while(!flag){
            flag=true;
            for(int j=0;j<nn-1;j++){
                if(ns[j]==c)continue;
                else if(ns[j+1]==c)ns[j]=ns[j+1];
                else{
                    flag=false;
                }
            } 
            if(flag){
                if(n!=nn)count++;
                break;
            }
            count++;
            nn--;
        }
        ans=min(count,ans);
    }

    cout << ans << endl;

    return 0;
}
