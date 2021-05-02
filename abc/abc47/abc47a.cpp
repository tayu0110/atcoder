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
    int a,b,c;
    cin >> a >> b >> c;

    if(a+b==c || a+c==b || b+c==a){
        cout << "Yes" << endl;
    }else{
        cout << "No" << endl;
    }

    return 0;
}
