#include "kuiper_portable.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

static unsigned char *read_file(const char *path, uint64_t *length) {
  FILE *file = fopen(path, "rb");
  if (!file || fseek(file, 0, SEEK_END)) exit(20);
  long size = ftell(file);
  if (size < 0 || size > 16777216 || fseek(file, 0, SEEK_SET)) exit(21);
  unsigned char *bytes = malloc((size_t)size + 1);
  if (!bytes || fread(bytes, 1, (size_t)size, file) != (size_t)size) exit(22);
  fclose(file);
  *length = (uint64_t)size;
  return bytes;
}
int main(int argc, char **argv) {
  if (argc != 5) return 10;
  uint64_t package_len, input_len, required = 0;
  unsigned char *package = read_file(argv[2], &package_len);
  unsigned char *input = read_file(argv[3], &input_len);
  unsigned char *output = malloc(16777216);
  if (!output) return 23;
  uint32_t (*run)(const uint8_t *, uint64_t, const uint8_t *, uint64_t,
                 const uint8_t *, uint64_t, uint32_t, uint8_t *, uint64_t,
                 uint64_t *) = strcmp(argv[4], "plan") == 0 ? kuiper_run_plan_v1 : kuiper_run_v1;
  uint32_t status = run((const uint8_t *)argv[1], strlen(argv[1]), package, package_len,
                       input, input_len, 1, output, 16777216, &required);
  if ((status == 0 || status == 3 || status == 4) && required <= 16777216)
    fwrite(output, 1, (size_t)required, stdout);
  putchar('\n');
  free(output); free(input); free(package);
  return (int)status;
}
