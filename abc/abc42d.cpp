//DFSによる実装の検討→不正解
//マップによる選択肢の積算→不正解
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

using ll = long long;
using namespace std;

#define mod ((ll)1e9+7)

int main(int argc,char* argv[]){
    ll h,w,a,b;
    cin >> h >> w >> a >> b;

    vector<vector<bool>> mj(h,vector<bool>(w,true));
    vector<vector<ll>> mapping(h,vector<ll>(w,0));
    for(int y=h-a;y<h;y++){
        for(int x=0;x<b;x++)
            mj.at(y).at(x)=false;
    }

    for(int y=0;y<h;y++){
        for(int x=0;x<w;x++){
            if(mj.at(y).at(x)==false)
                continue;
            if(x==0 && y==0)
                continue;
            if(y==0 && x!=0)
                mapping.at(y).at(x)=1;
            else if(x==0 && y!=0)
                mapping.at(y).at(x)=1;
            else
                mapping.at(y).at(x)=mapping.at(y-1).at(x)+mapping.at(y).at(x-1);
        }
    }

    cout << mapping.at(h-1).at(w-1)%mod << endl;

    return 0;        
}
   