//bit全探索による文字数列へのoperater挿入
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

    ll ans=0;
    for(ll bit = 0; bit < (1<<s.size()-1); bit++){
        int fow=0,bac=0;
        for(int i=0;i<s.size()-1;i++){
            if(bit & (1<<i)){
                /*digitsが1だと真*/
                bac=i+1;
                ans+=stoll(s.substr(fow,bac-fow));
                fow=bac;
            }
        }
        ans+=stoll(s.substr(fow,s.size()-fow));
    }

    cout << ans << endl;

    return 0;
}
