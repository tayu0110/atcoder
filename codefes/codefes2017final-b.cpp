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

    string s;
    cin >> s;

    if(s.size()==1){
        cout << "YES" << endl;
        return 0;
    }

    int a=0,b=0,c=0;
    for(int i=0;i<s.size();i++){
        if(s[i]=='a')a++;
        else if(s[i]=='b')b++;
        else c++;
    }

    if(abs(a-b)<2 && abs(b-c)<2 && abs(c-a)<2){
        cout << "YES" << endl;
    }else{
        cout << "NO" << endl;
    }

    return 0;
}
