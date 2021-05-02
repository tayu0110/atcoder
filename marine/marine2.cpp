//The comment below is the wrong answer
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

int main(int argc,char* argv[]){
    ll a,b,v,w,t;
    cin >> a >> v >> b >> w >> t;

    //bool ans=false;

    if(v<=w){
        cout << "NO" << endl;
        return 0;
    }

    ll dist=abs(b-a);
    ll ver=v-w;

    if(ver*t>=dist){
        cout << "YES" << endl;
    }else{
        cout << "NO" << endl;
    }
    
    /*int ap=a,bp=b;
    for(int i=1;i<=t;i++){
        if(ap<bp){
            if(ap+v<=pmax)
                ap+=v;
            else
                ap-=v;
            if(bp+w<=pmax)
                bp+=w;
            else
                bp-=w;
        }
        if(ap>bp){
            if(ap-v>=mmax)
                ap-=v;
            else
                ap+=v;
            if(bp-w>=mmax)
                bp-=w;
            else
                bp+=w;
        }
        if(ap==bp){
            ans=true;
            break;
        }
    }
    if(ans==true)
        cout << "YES" << endl;
    else
        cout <<"NO" << endl;*/

    return 0;
}
