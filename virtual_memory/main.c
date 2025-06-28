#include "read_file.h"
#include <ctype.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

int main() {
  int fd1[2];
  if (pipe(fd1) == -1) {
    printf("Error occurred opening pipe");
    return 1;
  }

  pid_t pid = fork();
  if (pid == 0) {
    // Child process 1
    close(fd1[0]);
    printf("child \n");
    char *parsed_content = read_file("string.txt");

    size_t len = strlen(parsed_content) + 1;
    write(fd1[1], &len, sizeof(size_t));
    write(fd1[1], parsed_content, len);

    free(parsed_content);
    close(fd1[1]);

  } else {
    close(fd1[1]);
    pid_t pid2 = fork();

    if (pid2 == 0) {
      // Child process 2
      size_t len;
      read(fd1[0], &len, sizeof(size_t));
      char *content = malloc(len);
      read(fd1[0], content, len);
      close(fd1[0]);
      size_t length = strlen(content);
      char uppercase[length];
      for (int i = 0; i <= length; i++) {
        uppercase[i] = toupper(content[i]);
      }
      printf("parsed from child2: %s \n", uppercase);
      free(content);
    } else {
      // parent process
      printf("parent \n");
    }
  }
  return 0;
}
