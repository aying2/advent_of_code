#include <stdio.h>
#include <stdlib.h>
#include <string.h>

uint sum_halfway_duplicates(const uint* arr, const uint len) {
    uint sum = 0;
    for (uint i = 0; i < len/2; i++) {
       if (arr[i] == arr[i + len/2]){
           sum += 2 * arr[i];
       }
    }
    return sum; 
}

uint parse_file(FILE* fp, uint buf[]) {

    uint i = 0;
    int c;
    while ((c = fgetc(fp)) != EOF) {
        buf[i] = c - '0';
        
        i++;
    }

    return i;

}
int main(int argc, char *argv[]) {
    FILE* fp;
    char filename[] = "input/input1.txt";

    fp = fopen(filename, "r");
    
    if (!fp) {
        perror("file opening failed");
        return EXIT_FAILURE;
    }

    uint buf[5000];

    uint len = parse_file(fp, buf);

    uint res = sum_halfway_duplicates(buf, len);

    printf("%d", res); 


    fclose(fp);
    return EXIT_SUCCESS;
}
