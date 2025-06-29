#include "get_user_input.h"
#include "semaforo.h"
#include "shm_read.h"
#include "shm_write.h"
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
  int mutex_semid;  // Semaphore for mutual exclusion
  int signal_semid; // Semaphore for signaling

  if ((mutex_semid = semget(IPC_PRIVATE, 1, 0666 | IPC_CREAT)) == -1) {
    perror("semget mutex");
    return 1;
  }
  if (init_sem(mutex_semid, 1) == -1) {
    del_sem(mutex_semid);
    return 1;
  }

  // Create signaling semaphore (initial value 0)
  if ((signal_semid = semget(IPC_PRIVATE, 1, 0666 | IPC_CREAT)) == -1) {
    perror("semget signal");
    del_sem(mutex_semid);
    return 1;
  }
  if (init_sem(signal_semid, 0) == -1) {
    del_sem(mutex_semid);
    del_sem(signal_semid);
    return 1;
  }

  if (pipe(fd1) == -1) {
    printf("Error occurred opening pipe");
    del_sem(mutex_semid);
    del_sem(signal_semid);
    return 1;
  }

  pid_t pid = fork();
  if (pid == 0) {
    // Child 1
    close(fd1[0]);

    sem_wait(mutex_semid);
    printf("Proceso hijo 1:\n");
    char *parsed_content = get_user_input("string.txt");
    size_t len = strlen(parsed_content) + 1;

    write(fd1[1], &len, sizeof(size_t));
    write(fd1[1], parsed_content, len);

    free(parsed_content);
    close(fd1[1]);

    sem_signal(mutex_semid);

    // Wait for signal from child 2
    sem_wait(signal_semid);

    printf("\nProceso hijo 1:\n");
    shm_read();

  } else {
    close(fd1[1]);
    pid_t pid2 = fork();

    if (pid2 == 0) {
      // Child 2
      sem_wait(mutex_semid);

      size_t len;
      read(fd1[0], &len, sizeof(size_t));
      char *content = malloc(len);
      read(fd1[0], content, len);
      close(fd1[0]);

      size_t length = strlen(content);
      char uppercase[length + 1];
      for (size_t i = 0; i < length; i++) {
        uppercase[i] = toupper(content[i]);
      }
      uppercase[length] = '\0';

      printf("\nProceso hijo 2:");
      shm_write(uppercase);
      free(content);

      sem_signal(mutex_semid);

      // Signal child 1 to proceed
      sem_signal(signal_semid);

    } else {
      // Parent
      wait(NULL); // Wait for child 1
      wait(NULL); // Wait for child 2
      del_sem(mutex_semid);
      del_sem(signal_semid);
      printf("\nProceso padre:\n");
      shm_read();
    }
  }

  return 0;
}
