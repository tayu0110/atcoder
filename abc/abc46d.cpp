//勝敗で得点が変化する場合、勝敗による得点変化の傾向をつかむことが重要
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

#define bil ((ll)1e9)
#define mod ((ll)1e9+7)

int main(int argc,char* argv[]){
    string s;
    cin >> s;

    ll n=s.size(),penable=0,g=0,p=0;
    //ll lastp=0;

    for(int count=0;count<n;count++){
        if(s[count]=='g'){
            penable++;
            g++;
        }
        else if(s[count]=='p'){
            if(penable>0){
                penable--;
            }else{
                p++;
            }
            //lastp=count;
        }
    }

    ll win,even,lose;
    lose=p;
    if(penable>0)
        win=penable/2;
    even=g;

    cout << win-lose << endl;

    return 0;
}
