#include "get_user_input.h"
#include "semaforo.h"
#include <ctype.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/ipc.h>
#include <sys/sem.h>
#include <sys/types.h>
#include <sys/wait.h>
#include <unistd.h>

int main() {
  int fd1[2];
  int semid;

  // crear semáforo
  if ((semid = semget(IPC_PRIVATE, 1, 0666 | IPC_CREAT)) == -1) {
    perror("semget");
    return 1;
  }

  // inicializar semáforo
  if (init_sem(semid, 1) == -1) {
    return 1;
  }

  if (pipe(fd1) == -1) {
    printf("Error occurred opening pipe");
    del_sem(semid);
    return 1;
  }

  pid_t pid = fork();
  if (pid == 0) {
    // hijo 1
    close(fd1[0]);

    sem_wait(semid);
    printf("Child 1 acquired semaphore\n");

    char *parsed_content = get_user_input("string.txt");
    size_t len = strlen(parsed_content) + 1;

    write(fd1[1], &len, sizeof(size_t));
    write(fd1[1], parsed_content, len);

    free(parsed_content);
    close(fd1[1]);

    sem_signal(semid);
    printf("Child 1 released semaphore\n");

  } else {
    close(fd1[1]);
    pid_t pid2 = fork();

    if (pid2 == 0) {
      // hijo 2
      sem_wait(semid);
      printf("Child 2 acquired semaphore\n");

      size_t len;
      read(fd1[0], &len, sizeof(size_t));
      char *content = malloc(len);
      read(fd1[0], content, len);
      close(fd1[0]);

      size_t length = strlen(content);
      char uppercase[length + 1]; // +1 for null terminator
      for (size_t i = 0; i < length; i++) {
        uppercase[i] = toupper(content[i]);
      }
      uppercase[length] = '\0';

      printf("Parsed from child2: %s\n", uppercase);
      free(content);

      sem_signal(semid);
      printf("Child 2 released semaphore\n");

    } else {
      // padre
      printf("Parent waiting for children...\n");
      wait(NULL); // esperar por hijo 1
      wait(NULL); // esperar por hijo 2
      del_sem(semid);
      printf("Parent done\n");
    }
  }
  return 0;
}
