#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <regex.h>
#include <string.h>


int match_line(int buf[], char str[], regex_t* preg) {

    // only match one subgroup (the main one)
    const int nmatch = 1;     

    // store position information of match
    regmatch_t pmatch[nmatch];     

    // part of the line, the start of which is to be moved up
    char *slice = str;

    // counter
    int i = 0;

    // while there are matches left in the slice
    int match;
    while ((match = regexec(preg, slice, nmatch, pmatch, 0)) == 0) {
        
        // extract the num as a string from the match
        char str_num[10];
        size_t start = pmatch[0].rm_so;
        size_t end = pmatch[0].rm_eo;
        strncpy(str_num, slice + start, end - start);
        str_num[end - start] = '\0';
        //printf("%s\n", str_num);

        // convert string to int: base 10
        int num = strtol(str_num, NULL, 10);

        //printf("%d\n", num);
        buf[i++] = num;

        // move up the start of the string to match
        slice = slice + end;
    }
    return i;
}

int even_division_res(int buf[], int len) {
    for (int i = 0; i < len - 1; i++) {
        for (int j = i + 1; j < len; j++) {
            if (buf[i] % buf[j] == 0) {
                return buf[i] / buf[j];
            }
            else if (buf[j] % buf[i] == 0) {
                return buf[j] / buf[i];
            }
        }
    }
    return -1;
}

int parse_file(FILE* fp) {
    const int len = 256;
    char str[len];

    // compile regex
    regex_t reg;


    // match integers
    // note: also matches those that start with leading 0
    int comp = regcomp(&reg, "[0-9]+", REG_EXTENDED);

    int sum = 0;

    // for each line in file
    while (fgets(str, len, fp) != NULL) {
   //     printf("%s\n", str);

        int buf[20];
        int len = match_line(buf, str, &reg);

        sum += even_division_res(buf, len);   
    }

    regfree(&reg);

    return sum;
}


int main(int argc, char *argv[]) {
    FILE* fp;
    char filename[] = "input/input2.txt";

    fp = fopen(filename, "r");
    
    if (!fp) {
        perror("file opening failed");
        return EXIT_FAILURE;
    }


    int sum = parse_file(fp);


    printf("%d\n", sum);

    fclose(fp);
    return EXIT_SUCCESS;
}
