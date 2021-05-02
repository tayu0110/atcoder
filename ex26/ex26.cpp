#include<iostream>
#include<map>
#include<vector>
#include<queue>
#include<cstdio>
#include<string>
#include<cstdlib>
#include<utility>
#include<cstring>

using namespace std;

void intdef(map<char, int>& iv);
int intexp(map<char, int>& iv, queue<char>& op);
void prtint(map<char, int>& iv);
void vecdef(map<char, vector<int>>& vv);
vector<int> vecexp(map<char, vector<int>>& vv,queue<char> &exp);
void prtvec(map<char, vector<int>>& vv);

int main() {
    int n;
    cin >> n;

    map<char, int> intvar;
    map<char, vector<int>> vecvar;

    for (int i = 0;i < n;n++) {
        string cmd;
        cin >> cmd;
        if (cmd == "int")               intdef(intvar);
        else if (cmd == "print_int")    prtint(intvar);
        else if (cmd == "vec")          vecdef(vecvar);
        else if (cmd == "print_vec")    prtvec(vecvar);
        else {
            cout << "Error" << endl;
            return 0;
        }
    }
    return 0;
}

//整数変数の宣言
void intdef(map<char, int>& iv) {
    char name, sbst;
    cin >> name >> sbst;
    if (!(iv.count(name)))          iv.insert(make_pair(name, 0));
    queue<char> exp;
    for (char op;op != ';';) {
        cin >> op;
        if (op != ';')  exp.push(op);
    }
    iv.at(name) = intexp(iv, exp);
}

//整数式の実装
int intexp(map<char, int>& iv, queue<char>& op) {
    int result = 0;
    while (!(op.empty())) {
        if (!(op.front() == '+' || op.front() == '-')) {
            if (iv.count(op.front()))   result += iv.at(op.front());
            else                        result += atoi(op.front());
            op.pop();
        }
        else if (op.front() == '+') {
            op.pop();
            if (iv.count(op.front()))   result += iv.at(op.front());
            else                        result += atoi(op.front());
            op.pop();
        }
        else if (op.front() == '-') {
            op.pop();
            if (iv.count(op.front()))   result += iv.at(op.front());
            else                        result += atoi(op.front());
            op.pop();
        }
    }
    return result;
}

//整数出力の実装
void prtint(map<char, int>& iv) {
    queue<char> exp;
    for (char op = NULL;op != ';';) {
        cin >> op;
        if (op != ';')
            exp.push(op);
    }
    int result = 0;
    result = intexp(iv, exp);
    cout << result << endl;
}

//配列宣言
void vecdef(map<char, vector<int>>& vv) {
    char name, sbst;
    cin >> name >> sbst;
    if (!(vv.count(name)))
        vv.insert(make_pair(name, vector<int>(0, 0)));
    queue<char> exp;
    for (char op='a';op != ';';) {
        cin >> op;
        if ((op != ';') && (op != ' ') && (op != ','))
            exp.push(op);
    }
    vv.at(name) = vecexp(vv, exp);
}

//配列式の実装
vector<int> vecexp(map<char, vector<int>>& vv,queue<char> &exp) {
    vector<int> result(0);
    if (!(exp.empty())){
        if(vv.count(exp.front())){
            for (auto x:vv.at(exp.front()))         result.push_back(x);
            exp.pop();
        }else if (exp.front()=='['){
            exp.pop();
            for(char c=exp.front();c!=']';c=exp.front()){
                result.push_back(atoi((c));
                exp.pop();
            }
            exp.pop();
        }
    }
    while(!(exp.empty())){
        if(exp.front()=='+'){
            exp.pop();
            if(vv.count(exp.front())){
                for (int i=0;i<result.size();i++)   result.at(i)+=vv.at(exp.front()).at(i);
                exp.pop();
            }else if (exp.front()=='['){
                exp.pop();
                for(int i=0;i<result.size();i++){
                    result.at(i)+=atoi(exp.front());
                    exp.pop();
                }
                exp.pop();
            }
        }else if(exp.front()=='-'){
            exp.pop();
            if(vv.count(exp.front())){
                for (int i=0;i<result.size();i++)   result.at(i)-=vv.at(exp.front()).at(i);
                exp.pop();
            }else if (exp.front()=='['){
                exp.pop();
                for(int i=0;i<result.size();i++){
                    result.at(i)-=atoi(exp.front());
                    exp.pop();
                }
                exp.pop();
            }
        }
    }
    return result;
}

//配列出力の実装
void prtvec(map<char, vector<int>>& vv) {
    queue<char> exp;
    for (char op;op != ';';) {
        cin >> op;
        if (op != ';')
            exp.push(op);
    }
    vector<int> result;
    result = vecexp(vv, exp);
    cout << "[ ";
    for(int i=0;i<result.size();i++){
        cout << result.at(i) << " ";
    }
    cout << ']' << endl;
}
