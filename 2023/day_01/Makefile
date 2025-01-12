CC = clang 
CFLAGS = -std=c99 -Weverything -Wno-missing-prototypes -Wno-unsafe-buffer-usage

main: main.c
	$(CC) -o $@ $^ $(CFLAGS)

.PHONY: clean
clean:
	rm *.o
