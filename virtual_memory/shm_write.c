#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/ipc.h>
#include <sys/shm.h>
#include <sys/types.h>
#define SHM_SIZE 1024 // Size of shared memory segment
//
void shm_write(const char *str) {
  int shmid;
  key_t key = 1234; // Arbitrary key for shared memory

  // Create the shared memory segment
  if ((shmid = shmget(key, SHM_SIZE, IPC_CREAT | 0666)) < 0) {
    perror("shmget");
    exit(1);
  }

  // Attach to the shared memory segment
  char *shm = shmat(shmid, NULL, 0);
  if (shm == (char *)-1) {
    perror("shmat");
    exit(1);
  }

  // Write the string to shared memory
  strncpy(shm, str, SHM_SIZE - 1);
  shm[SHM_SIZE - 1] = '\0'; // Ensure null-termination

  printf("\nEscrito en memoria compartida: %s\n", shm);

  // Detach from shared memory (but keep it alive)
  shmdt(shm);
}
