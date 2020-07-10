#include <stdio.h>
#include<stdlib.h>
#include <mpi.h>

int main(int argc, char const *argv[]) {
    MPI_Init(NULL, NULL);

    int process_count;
    MPI_Comm_size(MPI_COMM_WORLD, &process_count);

    int rank;
    MPI_Comm_rank(MPI_COMM_WORLD, &rank);

    if (rank == 0) {
        int recv_arr[process_count - 1];
        for (int send_rank = 1; send_rank < process_count; send_rank++) {
            MPI_Status status;
            MPI_Recv(&recv_arr[send_rank-1], 1, MPI_INT, send_rank, 0, MPI_COMM_WORLD, &status);
            printf("received: %i from: %i\n", recv_arr[send_rank-1], send_rank);
        }
    }
    else {
        int send_val = rank * 3;
        MPI_Send(&send_val, 1, MPI_INT, 0, 0, MPI_COMM_WORLD);
        printf("sent: %i from: %i\n", send_val, rank);
    }

    MPI_Finalize();

    return 0;
}
