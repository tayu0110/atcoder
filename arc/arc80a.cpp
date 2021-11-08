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
    vector<ll> a(n),multi(n);
    int multi2=0,multi4=0;
    for(int i=0;i<n;i++){
        cin >> a[i];
        if(a[i]%4==0){
            multi[i]=4;
            multi4++;
            multi2++;
        }else if(a[i]%2==0){
            multi[i]=2;
            multi2++;
        }else{
            multi[i]=0;
        }
    }

    int multi0=a.size()-multi2;

    if(multi0==0){
        cout << "Yes" << endl;
    }else{
        if(multi0<=multi4){
            cout << "Yes" << endl;
        }else{
            if(n%2==1 && multi0-multi4==1){
                cout << "Yes" << endl;
            }else{
                cout << "No" << endl;
            }
        }
    }

    return 0;
}
