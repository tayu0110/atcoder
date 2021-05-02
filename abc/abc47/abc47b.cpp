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
    int w,h,n;
    cin >> w >> h >> n;

    vector<pii> xy(n);
    vector<int> a(n);
    for(int i=0;i<n;i++)
        cin >> xy[i].first >> xy[i].second >> a[i];

    vector<bool> wbh(w,true),hbh(h,true);
    for(int i=0;i<n;i++){
        if(a[i]==1){
            for(int count=0;count<xy[i].first;count++)
                wbh[count]=false;
        }else if(a[i]==2){
            for(int count=w-1;count>xy[i].first-1;count--)
                wbh[count]=false;
        }else if(a[i]==3){
            for(int count=0;count<xy[i].second;count++)
                hbh[count]=false;
        }else if(a[i]==4){
            for(int count=h-1;count>xy[i].second-1;count--)
                hbh[count]=false;
        }
    }

    int wh=0,hh=0;
    for(int wide=0;wide<w;wide++){
        if(wbh[wide]==true)
            wh++;
    }
    for(int high=0;high<h;high++){
        if(hbh[high]==true)
            hh++;
    }

    cout << wh*hh << endl;

    return 0;
}
