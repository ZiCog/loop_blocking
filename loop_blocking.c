#include <stdio.h>
#include <time.h>
#include <inttypes.h>
#include <stdlib.h>

#define MAX 8192
#define BLOCK_SIZE 64

typedef int32_t board[MAX][MAX];

void do_it_0 (int32_t a[MAX][MAX], int32_t b[MAX][MAX]) {
    for (size_t i = 0; i < MAX;     i++) {
        for (size_t j = 0; j < MAX; j++) {
            a[i][j] += b[j][i];
        }
    }
}

void do_it_1 (int32_t a[MAX][MAX], int32_t b[MAX][MAX]) {
    for (size_t i = 0; i < MAX; i += BLOCK_SIZE) {
        for (size_t j = 0; j < MAX; j += BLOCK_SIZE) {
            for (size_t ii = i; ii < i + BLOCK_SIZE; ii++) {
                for (size_t jj = j; jj < j + BLOCK_SIZE; jj++) {
                    a[ii][jj] += b[jj][ii];
                }
             }
        }
    }
}

void fill_arrays (int32_t a[MAX][MAX], int32_t b[MAX][MAX]) {
    int32_t count = 0.0; 
    for (size_t i = 0; i < MAX; i++) {
        for (size_t j =0; j < MAX; j++) {
            a[i][j] = count;
            b[j][i] = -count;
            count += 1.0;
        }
    }
}

void print_array (int32_t a[MAX][MAX]) {
    for (size_t i = 0; i < 32; i++) {
        for (size_t j = 0; j < 32; j++) {
            printf("%.0f, ", a[i][j]);
        }
        printf("\n");
    }
}

// Return a timestamp in nano second resolution.
uint64_t time_ns( void ) {
  struct timespec now;
  clock_gettime( CLOCK_MONOTONIC, &now );
  return (uint64_t)now.tv_sec * UINT64_C(1000000000) + (uint64_t)now.tv_nsec;
}

int main () {
    printf("MAX:        %d\n", MAX);
    printf("BLOCK_SIZE: %d\n", BLOCK_SIZE);

    int32_t (*a)[MAX * MAX] = malloc(MAX * MAX * sizeof(int32_t));
    int32_t (*b)[MAX * MAX] = malloc(MAX * MAX * sizeof(int32_t));

    {
        fill_arrays(a, b);
        uint64_t startTime = time_ns();
        do_it_0(a, b);
        uint64_t endTime = time_ns();
        printf ("do_it_0:    %" PRIu64 "us\n", (endTime - startTime) / 1000U);
        //print_array(a);
    }
    {
        fill_arrays(a, b);
        uint64_t startTime = time_ns();
        do_it_1(a, b);
        uint64_t endTime = time_ns();
        printf ("do_it_1:    %" PRIu64 "us\n", (endTime - startTime) / 1000U);
        //print_array(a);
    }
}

