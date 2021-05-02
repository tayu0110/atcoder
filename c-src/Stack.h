#include<stdio.h>
#include<stdlib.h>

#define MAX_STACK_SIZE 100

struct stack{
    int top;
    int element[MAX_STACK_SIZE];
};

void push(int x, struct stack *S){
    if(S->top>=MAX_STACK_SIZE || S->top<0) S->top=MAX_STACK_SIZE;
    if(S->top==0){
        printf("Error: Stack is full.\n");
        exit(1);
    }else{
        S->top=S->top-1;
        S->element[S->top]=x;
    }
    return;
}

void pop(struct stack *S){
    if(S->top<MAX_STACK_SIZE){
        S->top=S->top+1;
    }else{
        printf("Error: Stack is empty.\n");
        exit(1);
    }
    return;
}
