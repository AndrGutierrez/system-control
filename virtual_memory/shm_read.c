#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/ipc.h>
#include <sys/shm.h>
#include <sys/types.h>
#define SHM_SIZE 1024 // Size of shared memory segment
//
void shm_read() {
  int shmid;
  key_t key = 1234; // Same key as before

  // Locate the shared memory segment
  if ((shmid = shmget(key, SHM_SIZE, 0666)) < 0) {
    perror("shmget");
    exit(1);
  }

  // Attach to the shared memory segment
  char *shm = shmat(shmid, NULL, 0);
  if (shm == (char *)-1) {
    perror("shmat");
    exit(1);
  }

  printf("Leído desde la memoria compartida: %s\n", shm);

  // Detach from shared memory
  shmdt(shm);

  // Remove the shared memory segment (optional)
  // shmctl(shmid, IPC_RMID, NULL);
}
