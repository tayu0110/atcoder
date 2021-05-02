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

    if(a==b && a==c)
        cout << "1" << endl;    
    else if(a!=b && a!=c && b!=c)
        cout << "3" << endl;
    else
        cout << "2" << endl;
    
    return 0;
}
