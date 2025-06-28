#ifndef SEMAFORO_H
#define SEMAFORO_H

#include <sys/ipc.h>
#include <sys/sem.h>
#include <sys/types.h>

union semun {
    int val;
    struct semid_ds *buf;
    unsigned short *array;
};

int init_sem(int semid, int value);
int sem_wait(int semid);
int sem_signal(int semid);
void del_sem(int semid);

#endif
