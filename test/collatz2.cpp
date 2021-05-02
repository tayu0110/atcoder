//collatz2.cpp
#include<iostream>
#include<set>
#include<ctime>

using namespace std;
using ll = long long;
#define INF (1uLL<<60)

int main(){
    set<ll> check;
    clock_t finish;

    for(ll i=1; i<1000000; i++){
        ll ans = i;
        set<ll> duplication;
        do{
            if(check.count(ans)) break;
            if(duplication.count(ans)){
                cout << i << " : NG" << endl;
                return 0;
            }
            else{
                duplication.insert(ans);
                check.insert(ans);
            }
            if(ans%2==1)    ans = ans*3+1;
            else            ans = ans/2;
        }while(ans != 1);
    }

    finish = clock();
    

    cout << "All Clear" << endl;
    cout << "TIME : " << finish << endl;
    
    return 0;
}
