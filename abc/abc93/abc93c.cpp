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
    int a,b,c;
    cin >> a >> b >> c;

    int count=0;
    while(!(a==b && b==c)){
        if(a==max(a,max(b,c))){
            if(a==b){
                c+=2;
            }else if(a==c){
                b+=2;
            }else if(b==c){
                b++;
                c++;
            }else{
                if(b<c)b+=2;
                else c+=2;
            }
        }else if(b==max(a,max(b,c))){
            if(b==c){
                a+=2;
            }else if(a==c){
                a++;
                c++;
            }else{
                if(a<c)a+=2;
                else c+=2;
            }
        }else{
            if(a==b){
                a++;
                b++;
            }else{
                if(a<b)a+=2;
                else b+=2;
            }
        }
        count++;
    }

    cout << count << endl;

    return 0;
}
