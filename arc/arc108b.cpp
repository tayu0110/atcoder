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
    string s;
    cin >> n >> s;
    int now=0;
    while(now<s.size()){
        stack<string> prev;
        if(s[now]!='f' && s[now]!='o' && s[now]!='x'){
            now++;
            continue;
        } else {
            while(now<s.size()){
                if(s[now]=='f'){
                    now++;
                    prev.push("f");
                }else if(s[now]=='o'){
                    if(!prev.empty() && prev.top()=="f"){
                        now++;
                        prev.push("o");
                        continue;
                    }else {
                        now++;
                        break;
                    }
                }else if(s[now]=='x'){
                    if(!prev.empty() && prev.top()=="o"){
                        prev.pop();
                        if(!prev.empty() && prev.top()=="f"){
                            now++;
                            prev.pop();
                            n-=3;
                            continue;
                        }else{
                            now++;
                            break;
                        }
                    }else {
                        now++;
                        break;
                    }
                }else {
                    now++;
                    break;
                }
            }
        }
    }
    cout << n << endl;
    return 0;
}
