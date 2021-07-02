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
    string sa,sb,sc;
    cin >> sa >> sb >> sc;

    queue<char> a,b,c;
    for(int i=0;i<max(sa.size(),max(sb.size(),sc.size()));i++){
        if(i<sa.size())
            a.push(sa[i]);
        if(i<sb.size())
            b.push(sb[i]);
        if(i<sc.size())
            c.push(sc[i]);
    }

    char winner,turn=a.front();
    a.pop();
    while(1){
        if(turn=='a'){
            if(a.size()==0){
                winner='A';
                break;
            }
            turn=a.front();
            a.pop();
        }else if(turn=='b'){
            if(b.size()==0){
                winner='B';
                break;
            }
            turn=b.front();
            b.pop();
        }else{
            if(c.size()==0){
                winner='C';
                break;
            }
            turn=c.front();
            c.pop();
        }
    }

    cout << winner << endl;

    return 0;
}
