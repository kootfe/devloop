#include <pthread.h>
#include <stdio.h>
#include <sys/stat.h>

#define FIFO_PATH "/tmp/devloop"

void *listenFifo() 
{
    int fifofd;

    if (mkfifo(FIFO_PATH, 0666) != 0) {
        perror(

    return NULL;
}
int main() 
{
    pthread_t fifo_thread;
    
    if(pthread_create(&fifo_thread, NULL, listenFifo, NULL) != 0) {
        perror("pthread_craete\n");
        return -1;
    }

    return 0;
}
