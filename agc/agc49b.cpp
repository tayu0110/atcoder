#include<iostream>
#include<iomanip>
#include<string>
#include<vector>
#include<algorithm>
#include<utility>
#include<tuple>
#include<map>
#include<queue>
#include<deque>
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

    int n;
    cin >> n;
    string s,t;
    cin >> s >> t;
    int sone=0,tone=0;
    queue<int> sonept;
    for(int i=0;i<n;i++){
        if(s[i]=='1'){
            sone++;
            sonept.push(i);
        }
        if(t[i]=='1')tone++;
    }
    if(sone<tone){
        cout << -1 << endl;
        return 0;
    }
    ll count=0;
    for(int i=0;i<n;i++){
        if(s[i]!=t[i]){
            if(s[i]=='1'){
                if(sonept.size()<2){
                    cout << -1 << endl;
                    return 0;
                }
                sonept.pop();
                int next=sonept.front();
                sonept.pop();
                count+=next-i;
                s[i]='0';
                s[next]='0';
                sone--;
            }else{
                if(sonept.empty()){
                    cout << -1 << endl;
                    return 0;
                }
                int front=sonept.front();
                sonept.pop();
                count+=front-i;
                s[i]='1';
                s[front]='0';
            }
        }else{
            if(sonept.empty())continue;
            while(sonept.front()<=i){
                sonept.pop();
                if(sonept.empty())break;
            }
        }
        if(sone<tone){
            cout << -1 << endl;
            return 0;
        }
    }
    cout << count << endl;

    return 0;
}
