#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <regex.h>
#include <string.h>

int spiral_threshold(int threshold) {
    int ring_level = 1;
    int cur_size = 0;
    int prev_ring = {0};
    int num = 0;
    while (num <= threshold) {
        int cur_size = ring_level^2 - cur_size;

        int cur_ring[cur_size];
        




        // the next ring has a larger side length by 2
        ring_level += 2;

        prev_ring = cur_ring;
    }
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
