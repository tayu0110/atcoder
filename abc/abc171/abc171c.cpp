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
#define DOGS ((ll)1e15+1)

int main(int argc,char* argv[]){
    ll n;
    cin >> n;

    queue<int> name;
    ll nnum=n;

    do{
        if(nnum%26!=0){
            name.push(nnum%26);
            nnum=nnum/26;
        }else{
            name.push(26);
            nnum=(nnum-26)/26;
        }
    }while(nnum>0);

    string dogname;
    while(!(name.empty())){
        if(name.front()==1){
            dogname.push_back('a');
            name.pop();
            continue;
        }else if(name.front()==2){
            dogname.push_back('b');
            name.pop();
            continue;
        }else if(name.front()==3){
            dogname.push_back('c');
            name.pop();
            continue;
        }else if(name.front()==4){
            dogname.push_back('d');
            name.pop();
            continue;
        }else if(name.front()==5){
            dogname.push_back('e');
            name.pop();
            continue;
        }else if(name.front()==6){
            dogname.push_back('f');
            name.pop();
            continue;
        }else if(name.front()==7){
            dogname.push_back('g');
            name.pop();
            continue;
        }else if(name.front()==8){
            dogname.push_back('h');
            name.pop();
            continue;
        }else if(name.front()==9){
            dogname.push_back('i');
            name.pop();
            continue;
        }else if(name.front()==10){
            dogname.push_back('j');
            name.pop();
            continue;
        }else if(name.front()==11){
            dogname.push_back('k');
            name.pop();
            continue;
        }else if(name.front()==12){
            dogname.push_back('l');
            name.pop();
            continue;
        }else if(name.front()==13){
            dogname.push_back('m');
            name.pop();
            continue;
        }else if(name.front()==14){
            dogname.push_back('n');
            name.pop();
            continue;
        }else if(name.front()==15){
            dogname.push_back('o');
            name.pop();
            continue;
        }else if(name.front()==16){
            dogname.push_back('p');
            name.pop();
            continue;
        }else if(name.front()==17){
            dogname.push_back('q');
            name.pop();
            continue;
        }else if(name.front()==18){
            dogname.push_back('r');
            name.pop();
            continue;
        }else if(name.front()==19){
            dogname.push_back('s');
            name.pop();
            continue;
        }else if(name.front()==20){
            dogname.push_back('t');
            name.pop();
            continue;
        }else if(name.front()==21){
            dogname.push_back('u');
            name.pop();
            continue;
        }else if(name.front()==22){
            dogname.push_back('v');
            name.pop();
            continue;
        }else if(name.front()==23){
            dogname.push_back('w');
            name.pop();
            continue;
        }else if(name.front()==24){
            dogname.push_back('x');
            name.pop();
            continue;
        }else if(name.front()==25){
            dogname.push_back('y');
            name.pop();
            continue;
        }else if(name.front()==26){
            dogname.push_back('z');
            name.pop();
            continue;
        }
    }

    for(int i=dogname.size()-1;i>=0;i--){
        cout << dogname.at(i);
    }

    return 0;
}
