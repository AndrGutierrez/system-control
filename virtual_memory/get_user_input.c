
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

char *get_user_input() {

  printf("Ingrese un texto: ");

  char buffer[256];
  if (fgets(buffer, sizeof(buffer), stdin) == NULL) {
    return strdup(""); // Return empty string on error
  }

  // Remove trailing newline if present
  size_t len = strlen(buffer);
  if (len > 0 && buffer[len - 1] == '\n') {
    buffer[len - 1] = '\0';
  }

  return strdup(buffer);
}
