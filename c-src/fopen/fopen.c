#include<stdio.h>

int main(){
    FILE *fp;

    fp=fopen("test.txt","w");

    if(fp==NULL){
        printf("ファイルのオープンに失敗しました\n");
        return -1;
    }else{
        printf("ファイルは正常にオープンされました\n");
    }

    fprintf(fp,"初めてのファイルオープン\n");

    if(fclose(fp)!=0){
        perror("ファイルのクローズに失敗しました\n");
    }else{
        printf("ファイルは正常にクローズされました\n");
    }

    return 0;
}