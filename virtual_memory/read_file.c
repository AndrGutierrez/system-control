#include <stdio.h>
#include <stdlib.h>

char *read_file(char *filename) {
  FILE *file = fopen(filename, "r");
  char *text = malloc(256);
  fgets(text, 256, file);
  printf("%s", text);
  fclose(file);
  return text;
}
