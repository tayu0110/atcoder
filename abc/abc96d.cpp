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
    vector<int> p(0);
    p.push_back(2);
    for(int i=3;i<=55555;i+=2){
        bool flag=true;
        for(int j=0;j<p.size();j++){
            int d=p[j];
            if(i%d==0) {
                flag=false;
                break;
            }
        }
        if(flag) p.push_back(i);
    }
    int cnt=0;
    for(int i=0;i<p.size();i++){
        int num=p[i];
        if(num%5==1){
            cout << p[i] << " ";
            cnt++;
        }
        if(cnt==n) break;
    }
    cout << endl;
    return 0;
}
