#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct {
    unsigned int min;
    unsigned int max;
} Range;

static char* read_input(void)
{
    FILE* fd = fopen("input.txt", "rb");

    if (!fd) {
        printf("Error opening \"input.txt\"\n");
        exit(0);
    }

    fseek(fd, 0, SEEK_END);
    size_t len = ftell(fd);
    fseek(fd, 0, SEEK_SET);

    char* input = (char*)malloc(len + 1);
    input[len] = 0;

    size_t read_len = fread(input, 1, len, fd);

    if (read_len != len) {
        printf("Error reading \"input.txt\"\n");
        exit(0);
    }

    fclose(fd);
    return input;
}

static void get_range(const char* line, Range* out_range)
{
    const char* second = strchr(line, '-') + 1;
    out_range->min = strtoul(line, NULL, 10);
    out_range->max = strtoul(second, NULL, 10);
}

static int is_fully_contained(const Range* e1, const Range* e2)
{
    if (e1->min < e2->min || e1->min > e2->max) {
        return 0;
    }

    if (e1->max < e2->min || e1->max > e2->max) {
        return 0;
    }

    return 1;
}

static int is_either_fully_contained(const Range* e1, const Range* e2)
{
    return is_fully_contained(e1, e2) || is_fully_contained(e2, e1);
}

static int is_overlapping(const Range* e1, const Range* e2)
{
    if (e1->min < e2->min && e1->max < e2->min) {
        return 0;
    }

    if (e1->min > e2->max) {
        return 0;
    }

    return 1;
}

static int is_either_overlapping(const Range* e1, const Range* e2)
{
    return is_overlapping(e1, e2) || is_overlapping(e2, e1);
}

static void part1(char* input)
{
    char* line = strtok(input, "\n");
    unsigned int total = 0;

    while (line != NULL) {
        char* sep = strchr(line, ',');
        *sep = 0;

        const char* first = line;
        const char* second = sep + 1;

        Range e1, e2;
        get_range(first, &e1);
        get_range(second, &e2);

        if (is_either_fully_contained(&e1, &e2)) {
            total += 1;
        }

        line = strtok(NULL, "\n");
    }

    printf("%u\n", total);
}

static void part2(char* input)
{
    char* line = strtok(input, "\n");
    unsigned int total = 0;

    while (line != NULL) {
        char* sep = strchr(line, ',');
        *sep = 0;

        const char* first = line;
        const char* second = sep + 1;

        Range e1, e2;
        get_range(first, &e1);
        get_range(second, &e2);

        if (is_either_overlapping(&e1, &e2)) {
            total += 1;
        }

        line = strtok(NULL, "\n");
    }

    printf("%u\n", total);
}

int main()
{
    char* input = read_input();
    part1(strdup(input));
    part2(strdup(input));
}
