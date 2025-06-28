#include "semaforo.h"
#include <stdio.h>
#include <unistd.h>

int init_sem(int semid, int value) {
    union semun sem_union;
    sem_union.val = value;
    if (semctl(semid, 0, SETVAL, sem_union) == -1) {
        perror("semctl");
        return -1;
    }
    return 0;
}

int sem_wait(int semid) {
    struct sembuf sem_b;
    sem_b.sem_num = 0;
    sem_b.sem_op = -1;
    sem_b.sem_flg = SEM_UNDO;
    if (semop(semid, &sem_b, 1) == -1) {
        perror("semop wait");
        return -1;
    }
    return 0;
}

int sem_signal(int semid) {
    struct sembuf sem_b;
    sem_b.sem_num = 0;
    sem_b.sem_op = 1;
    sem_b.sem_flg = SEM_UNDO;
    if (semop(semid, &sem_b, 1) == -1) {
        perror("semop signal");
        return -1;
    }
    return 0;
}

void del_sem(int semid) {
    union semun sem_union;
    if (semctl(semid, 0, IPC_RMID, sem_union) == -1) {
        perror("semctl delete");
    }
}
