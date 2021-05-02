#include<stdio.h>

//Euclid's Algorithm
int gcd(int a, int b){
    int x,y,temp;

    x=a;
    y=b;

    while(y!=0){
        temp=x%y;
        x=y;
        y=temp;
    }

    return (x);
}