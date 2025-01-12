#define _GNU_SOURCE
#include <stdbool.h>
#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int get_calibration_value(char *line, size_t len) {
    char two_digit[3] = {0};
    for (int i = 0; i < len; ++i) {
        if (line[i] >= '0' && line[i] <= '9') {
            two_digit[0] = line[i];
            break;
        }
    }

    for (int i = len - 1; i >= 0; --i) {
        if (line[i] >= '0' && line[i] <= '9') {
            two_digit[1] = line[i];
            break;
        }
    }

    return atoi(two_digit);
}

static char const *const words[] = {"one", "two",   "three", "four", "five",
                                    "six", "seven", "eight", "nine"};

static int const digits[] = {1, 2, 3, 4, 5, 6, 7, 8, 9};

int check_word(char *line, size_t len, int pos) {
    int words_len = (sizeof words / sizeof words[0]);
    for (int i = 0; i < words_len; ++i) {
        int word_len = strlen(words[i]);
        if (pos + word_len > len) {
            continue;
        }

        bool match = true;
        for (int j = 0; j < word_len; ++j) {
            if (line[pos + j] != words[i][j]) {
                match = false;
            }
        }

        if (match == true) {
            return digits[i];
        }
    }

    return -1;
}

int check_word_reverse(char *line, size_t len, int pos) {
    int words_len = (sizeof words / sizeof words[0]);

    for (int i = 0; i < words_len; ++i) {
        int word_len = strlen(words[i]);

        if (pos - (word_len - 1) < 0) {
            continue;
        }

        bool match = true;
        for (int j = 0; j < word_len; ++j) {
            if (line[pos - (word_len - 1) + j] != words[i][j]) {
                match = false;
            }
        }

        if (match == true) {
            return digits[i];
        }
    }

    return -1;
}

int get_calibration_value_2(char *line, size_t len) {
    char two_digit[3] = {0};
    for (int i = 0; i < len; ++i) {
        int res = check_word(line, len, i);
        // printf("res = %d\n", res);
        if (line[i] >= '0' && line[i] <= '9') {
            two_digit[0] = line[i];
            break;
        } else if (res != -1) {
            two_digit[0] = res + '0';
            break;
        }
    }

    for (int i = len - 1; i >= 0; --i) {
        int res = check_word_reverse(line, len, i);
        if (line[i] >= '0' && line[i] <= '9') {
            two_digit[1] = line[i];
            break;
        } else if (res != -1) {
            two_digit[1] = res + '0';
            break;
        }
    }

    //    printf("%s\n", two_digit);

    return atoi(two_digit);
}

int main(int argc, char *argv[]) {
    FILE *fp = fopen("./input/input.txt", "r");

    if (fp == NULL) {
        printf("invalid file\n");
        return 1;
    }

    int sum = 0;
    char *line = NULL;
    size_t cap = 0;
    __ssize_t len = 0;
    while ((len = getline(&line, &cap, fp)) != -1) {
        sum += get_calibration_value(line, len);
    }
    printf("part 1: sum = %d\n", sum);

    rewind(fp);
    sum = 0;
    line = NULL;
    cap = 0;
    len = 0;
    while ((len = getline(&line, &cap, fp)) != -1) {
        sum += get_calibration_value_2(line, len);
    }
    printf("part 2: sum = %d\n", sum);

    //    printf("%d\n", check_word("asdfive1nine", 8, 3));
    //    printf("%d\n", check_word_reverse("asdfive1nine", 8, 3));

    free(line);
    fclose(fp);
    return EXIT_SUCCESS;
}
