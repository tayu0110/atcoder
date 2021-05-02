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
    vector<ll> a(n);
    map<ll,int> ck;
    for(int i=0;i<n;i++){
        cin >> a[i];
        if(ck.find(a[i])!=ck.end()){
            ck[a[i]]++;
        }else{
            ck.insert(make_pair(a[i],1));
        }
    }
    if(ck.size()>3){
        cout << "No" << endl;
        return 0;
    }
    if(ck.size()==1){
        ll key=ck.begin()->first;
        if(key==0){
            cout << "Yes" << endl;
        }else{
            cout << "No" << endl;
        }
    }else if(ck.size()==2){
        ll key[2];
        int num[2];
        int i=0;
        for(auto it=ck.begin();it!=ck.end();it++){
            key[i]=it->first;
            num[i]=it->second;
            i++;
        }
        if(num[0]<num[1]){
            swap(num[0],num[1]);
            swap(key[0],key[1]);
        }
        if(num[0]!=num[1]*2){
            cout << "No" << endl;
        }else{
            if((key[0]^key[1])==key[0] && (key[0]^key[0])==key[1]){
                cout << "Yes" << endl;
            }else{
                cout << "No" << endl;
            }
        }
    }else{
        ll key[3];
        int num[3];
        int i=0;
        for(auto it=ck.begin();it!=ck.end();it++){
            key[i]=it->first;
            num[i]=it->second;
            i++;
        }
        if(num[0]!=num[1] || num[0]!=num[2] || num[1]!=num[2]){
            cout << "No" << endl;
        }else{
            if((key[0]^key[1])==key[2] && (key[0]^key[2])==key[1] && (key[1]^key[2])==key[0]){
                cout << "Yes" << endl;
            }else{
                cout << "No" << endl;
            }
        }
    }
    return 0;
}
