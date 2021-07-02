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
    int n;
    cin >> n;
    
    vector<int> a(n);
    int mostmax=0,secmax=0;
    for(int i=0;i<n;i++){
        cin >> a[i];
        if(a[i]>mostmax){
            secmax=mostmax;
            mostmax=a[i];
        }else if(a[i]>secmax){
            secmax=a[i];
        }
    }
    
    for(int i=0;i<n;i++){
        if(a[i]==mostmax){
            cout << secmax << endl;
        }else{
            cout << mostmax << endl;
        }
    }

    return 0;
}
