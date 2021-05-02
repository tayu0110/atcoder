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
#define INF (1LL<<60)           //1LL<<63でオーバーフロー
#define inf (1<<29)             //1<<29でオーバーフロー

int main(int argc,char* argv[]){
    string a,b;
    cin >> a >> b;

    if(a.size()>b.size()){
        cout << "GREATER" << endl;
        return 0;
    }else if(a.size()<b.size()){
        cout << "LESS" << endl;
        return 0;
    }else{
        for(int i=0;i<a.size();i++){
            if(a[i]<b[i]){
                cout << "LESS" << endl;
                return 0;
            }else if(a[i]>b[i]){
                cout << "GREATER" << endl;
                return 0;
            }
        }
    }

    cout << "EQUAL" << endl;

    return 0;
}
