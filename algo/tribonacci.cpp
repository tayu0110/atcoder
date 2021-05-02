#include<iostream>
#include<cstring>

#define MAX_LENGTH 10000

long long t[MAX_LENGTH + 1];

long long tribonacci(long long i) {
    if(i == 0) return 0;
    if(i == 1) return 0;
    if(i == 2) return 1;
    if(t[i] != -1) return t[i];
    return t[i] = tribonacci(i - 1) + tribonacci(i - 2) + tribonacci(i - 3);
}

int main() {
    long long n;
    std::cin >> n;
    if(n > MAX_LENGTH) {
        std::cout << "Error: Input value is too large." << std::endl;
        return 0;
    }
    memset(t, -1, sizeof(t));
    long long ans = tribonacci(n);
    std::cout << ans << std::endl;
}