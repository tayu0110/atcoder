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
    char c;
    cin >> c;

    if(c=='a' || c=='i' || c=='u' || c=='e' || c=='o')
        cout << "vowel" << endl;
    else
        cout << "consonant" << endl;

    return 0;
}
