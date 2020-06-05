use std::{
    os::raw::{c_int, c_void},
    ptr,
};

use mpi_sys::{
    MPI_Barrier, MPI_Bcast, MPI_Comm_rank, MPI_Comm_size, MPI_Finalize, MPI_Init, MPI_Pcontrol,
    MPI_COMM_WORLD, MPI_INT,
};

fn main() {
    unsafe {
        MPI_Init(ptr::null_mut(), ptr::null_mut());
        let mut rank: c_int = 0;

        // MPI_Pcontrol(5); //not supported

        let mut gsize: c_int = 0;
        MPI_Comm_size(MPI_COMM_WORLD, &mut gsize);

        let mut to_send: c_int = 0;
        let sendarray = &mut to_send;
        MPI_Comm_rank(MPI_COMM_WORLD, &mut rank);

        if rank == 0 {
            *sendarray = 929;
        }
        MPI_Bcast(
            sendarray as *mut c_int as *mut c_void,
            1,
            MPI_INT,
            0,
            MPI_COMM_WORLD,
        );
        if rank != 0 {
            println!("RECEIVED VALUE IS : {}\n", *sendarray);
        }

        if rank == 0 {
            println!("Comm size : {}\n", gsize);
        }
        MPI_Barrier(MPI_COMM_WORLD);

        println!("Hi! from rank: {}\n", rank);
        MPI_Finalize();
    }
}
