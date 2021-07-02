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

#define bil ((ll)1e9)
#define mod ((ll)1e9+7)

int main(int argc,char* argv[]){
    int n;
    cin >> n;

    int sum=0;
    vector<int> a(n);
    for(int &x:a){
        cin >> x;
        sum+=x;
    }

    int ave1=sum/n+1,ave2=sum/n;
    
    int ans1=0,ans2=0;
    for(int x:a){
        ans1+=(x-ave1)*(x-ave1);
        ans2+=(x-ave2)*(x-ave2);
    }


    cout << min(ans1,ans2) << endl;

    return 0;
}
