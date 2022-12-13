#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <pthread.h>

pthread_mutex_t mut = PTHREAD_MUTEX_INITIALIZER;
pthread_cond_t cond = PTHREAD_COND_INITIALIZER;

volatile bool ready = false;
char buf[256];

void* producer(void *arg) {
    
    printf("Producer thread: \n");
    fgets(buf, sizeof(buf), stdin);
    printf("Producer: %s", mut);
    pthread_mutex_lock(&mut); // acquire lock
    ready = true;
    
    if(pthread_cond_broadcast(&cond) != 0) {
        perror("Error on pthread_cond_broadcast");
        exit(-1);
    } else {
        printf("Broadcast!");
    }
    pthread_mutex_unlock(&mut);
    return NULL;
}

void* consumer(void *arg) {
    pthread_mutex_lock(&mut); // acquire lock
    printf("Consumer thread \n");
    // Wait until ready
    while(!ready) {
        printf("Not Ready! \n");
        if(pthread_cond_wait(&cond, &mut) != 0) {
            perror("Error on pthread_cond_wait");
            exit(-1);
        } else {
            printf("wait!");
        }
    }
    pthread_mutex_unlock(&mut); // release lock
    printf("Consumer: %s\n", buf);
    return NULL;
}

int main(int argc, char *argv[]) {
    
    pthread_t pr, cn;
    pthread_create(&pr, NULL, producer, NULL);
    pthread_create(&cn, NULL, consumer, NULL);
    
    pthread_join(pr, NULL);
    pthread_join(cn, NULL);
    
    pthread_mutex_destroy(&mut);
    
    if (pthread_cond_destroy(&cond) != 0) {
        perror("Error destroying cond");
        exit(-1);
    }
    
    return 0;
}
