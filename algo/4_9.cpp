#include<iostream>
#include<vector>
#include<cstring>

using namespace std;

#define MAX_LENGTH 10000

bool memo[MAX_LENGTH];

bool func(int i, int w, const vector<int> &a) {
    if(i == 0) {
        if(w == 0) return true;
        else false;
    }
    if(memo[i]) return true;

    if(func(i - 1, w, a)) return memo[i] = true;
    if(func(i - 1, w - a[i - 1], a)) return memo[i] = true;
    return memo[i] = false;
}

int main() {
    int N, W;
    cin >> N >> W;
    vector<int> a(N);
    for(int i = 0; i < N; i++) {
        cin >> a[i];
    }
    memset(memo, false, sizeof(memo));
    if(func(N, W, a)) cout << "Yes" << endl;
    else cout << "No" << endl;
}