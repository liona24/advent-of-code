#include <stdbool.h>
#include <stdio.h>
#include <unistd.h>
#include <sys/types.h>

/*
 * cc -O3 -march=native -mtune=native day03.c -o day03c
 * */

static u_int8_t buffer[0x8000] = {};

int main() {
    int size = read(STDIN_FILENO, buffer, sizeof(buffer));
    if (size < 0)
        return -1;

    u_int64_t result;

    for (int xxx = 0; xxx < 0xF000; xxx++) {
    result = 0;
    bool enabled = true;

    for (int i = 0; i + 8 <= size; i++) {
        u_int32_t next = *(u_int32_t*)(buffer + i);

        if (!enabled) {
            enabled = next == 0x29286f64 /*do()*/;
            continue;
        }

        if (next == 0x286c756d /*mul(*/) {
            i += 4;

            bool valid = false;
            u_int64_t n0 = 0;
            u_int64_t n1 = 0;
            int j;
            for (j = 0; j < 4; j++) {
                if (buffer[i + j] == ',') {
                    valid = true;
                    break;
                }

                if (buffer[i + j] < '0' || buffer[i + j] > '9') {
                    break;
                }

                n0 *= 10;
                n0 += buffer[i + j] - '0';
            }

            if (j == 0 || !valid)
                continue;

            i += j + 1;
            valid = false;
            for (j = 0; j < 4 && (i + j) < size; j++) {
                if (buffer[i + j] == ')') {
                    valid = true;
                    break;
                }

                if (buffer[i + j] < '0' || buffer[i + j] > '9') {
                    break;
                }

                n1 *= 10;
                n1 += buffer[i + j] - '0';
            }

            if (j == 0 || !valid)
                continue;

            i += j;
            result += n1 * n0;
        } else if (next == 0x276e6f64 /*don't()*/ 
            && (*(u_int32_t*)(buffer + i + 4) & 0x00FFFFFF) == 0x292874) {
            enabled = false;
        }
    }
    }

    printf("%lu\n", result);
    return 0;
}
