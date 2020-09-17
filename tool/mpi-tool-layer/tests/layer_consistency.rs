use self::rmpi::{BufferRef, BufferRefKind, Process, RmpiResult, Tag};
use mpi_sys::*;
use mpi_tool_layer::{MpiInterceptionLayer, RawMpiInterceptionLayer, UnsafeBox};
use rmpi::pmpi_mode as rmpi;
use std::os::raw::*;

#[test]
fn mpi_send() {
    enum EmptyLayer {}
    impl MpiInterceptionLayer for EmptyLayer {
        fn send<F, Buf>(next_f: F, buf: Buf, dest: Process, tag: Tag) -> RmpiResult
        where
            F: FnOnce(Buf, Process, Tag) -> RmpiResult,
            Buf: BufferRef,
        {
            match buf.kind_ref() {
                BufferRefKind::I16(i16buf) => assert_eq!(i16buf, &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]),
                other_buf => panic!("unexpected buffer kind: {:?}", other_buf),
            }
            assert_eq!(dest.rank(), 1);
            assert_eq!(dest.communicator().as_raw(), MPI_COMM_WORLD);
            assert_eq!(tag, 7);
            let res = next_f(buf, dest, tag);
            res
        }
    }

    let buffer: &[i16] = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    let raw_next_f = UnsafeBox::new(|buf_ptr, buf_len, datatype, dst, tag, comm| {
        assert_eq!(buf_ptr, buffer.as_ptr() as *const c_void);
        assert_eq!(buf_len, buffer.len() as c_int);
        assert_eq!(datatype, MPI_INT16_T);
        assert_eq!(dst, 1);
        assert_eq!(tag, 7);
        assert_eq!(comm, MPI_COMM_WORLD);
        MPI_SUCCESS
    });

    <EmptyLayer as RawMpiInterceptionLayer>::send(
        raw_next_f,
        buffer.as_ptr() as *const c_void,
        buffer.len() as c_int,
        MPI_INT16_T,
        1,
        7,
        MPI_COMM_WORLD,
    );
}
