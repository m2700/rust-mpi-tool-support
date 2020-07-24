use std::os::raw::{c_double, c_int};

use rmpi::pmpi_mode as rmpi;

use self::rmpi::{
    request::{Request, RequestSlice},
    Buffer, Communicator, Group, MpiDatatype, MpiOp, Process, RmpiResult, Status, Tag,
};

pub trait MpiInterceptionLayer {
    trait_layer_function!(
        #[inline]
        fn finalize() -> RmpiResult;

        #[inline]
        fn wtime() -> c_double;
        #[inline]
        fn wtick() -> c_double;

        #[inline]
        fn barrier(comm: &Communicator) -> RmpiResult;

        #[inline]
        fn group_incl(group: &Group, ranks: &[c_int]) -> RmpiResult<Group>;
        #[inline]
        fn group_free(group: Group) -> RmpiResult;

        #[inline]
        fn comm_size(comm: &Communicator) -> RmpiResult<c_int>;
        #[inline]
        fn comm_rank(comm: &Communicator) -> RmpiResult<c_int>;
        #[inline]
        fn comm_create(
            comm: &Communicator,
            group: &Group,
        ) -> RmpiResult<Communicator>;
        #[inline]
        fn comm_free(comm: Communicator) -> RmpiResult;

        #[inline]
        fn send<Buf: ?Sized>(buf: &Buf, dest: Process, tag: Tag) -> RmpiResult
        where
            Buf: Buffer;
        #[inline]
        fn bsend<Buf: ?Sized>(buf: &Buf, dest: Process, tag: Tag) -> RmpiResult
        where
            Buf: Buffer;
        #[inline]
        fn ssend<Buf: ?Sized>(buf: &Buf, dest: Process, tag: Tag) -> RmpiResult
        where
            Buf: Buffer;
        #[inline]
        fn rsend<Buf: ?Sized>(buf: &Buf, dest: Process, tag: Tag) -> RmpiResult
        where
            Buf: Buffer;

        #[inline]
        fn isend<Buf: ?Sized>(buf: &Buf, dest: Process, tag: Tag) -> RmpiResult<Request>
        where
            Buf: Buffer;
        #[inline]
        fn ibsend<Buf: ?Sized>(buf: &Buf, dest: Process, tag: Tag) -> RmpiResult<Request>
        where
            Buf: Buffer;
        #[inline]
        fn issend<Buf: ?Sized>(buf: &Buf, dest: Process, tag: Tag) -> RmpiResult<Request>
        where
            Buf: Buffer;
        #[inline]
        fn irsend<Buf: ?Sized>(buf: &Buf, dest: Process, tag: Tag) -> RmpiResult<Request>
        where
            Buf: Buffer;

        #[inline]
        fn bcast<Buf: ?Sized>(buf: &mut Buf, root: Process) -> RmpiResult;

        #[inline]
        fn recv<Buf: ?Sized>(buf: &mut Buf, src: Process, tag: Tag) -> RmpiResult<Status>
        where
            Buf: Buffer;

        #[inline]
        fn get_count<Datatype>(status: &Status) -> RmpiResult<c_int>
        where
            Datatype: MpiDatatype;

        #[inline]
        fn buffer_attach(buffer: &'static mut [u8]) -> RmpiResult; //FIXME
        #[inline]
        fn buffer_detach() -> RmpiResult<&'static mut [u8]>;

        #[inline]
        fn wait(request: &mut Request) -> RmpiResult<Status>;
        #[inline]
        fn waitany(requests: &mut RequestSlice) -> RmpiResult<(usize, Status)>;
        #[inline]
        fn waitall(requests: &mut RequestSlice, responses: &mut [Status]) -> RmpiResult;

        #[inline]
        fn test(request: &mut Request) -> RmpiResult<Option<Status>>;
        #[inline]
        fn testany(request: &mut RequestSlice) -> RmpiResult<Option<(usize, Status)>>;

        #[inline]
        fn request_free(request: Request) -> RmpiResult;

        // should I really use Buffers of different datatypes??
        #[inline]
        fn gather<SendBuf: ?Sized, RecvBuf: ?Sized>(
            sendbuf: &SendBuf,
            recvbuf: &mut RecvBuf,
            root: Process,
        ) -> RmpiResult
        where
            SendBuf: Buffer,
            RecvBuf: Buffer;
        #[inline]
        fn gatherv<SendBuf: ?Sized, RecvBuf: ?Sized>(
            sendbuf: &SendBuf,
            recvbufs: &mut [&mut RecvBuf],
            root: Process,
        ) -> RmpiResult
        where
            SendBuf: Buffer,
            RecvBuf: Buffer;
        #[inline]
        fn allgather<SendBuf: ?Sized, RecvBuf: ?Sized>(
            sendbuf: &SendBuf,
            recvbuf: &mut RecvBuf,
            comm: &Communicator,
        ) -> RmpiResult
        where
            SendBuf: Buffer,
            RecvBuf: Buffer;
        #[inline]
        fn allgatherv<SendBuf: ?Sized, RecvBuf: ?Sized>(
            sendbuf: &SendBuf,
            recvbufs: &mut [&mut RecvBuf],
            comm: &Communicator,
        ) -> RmpiResult
        where
            SendBuf: Buffer,
            RecvBuf: Buffer;

        #[inline]
        fn reduce<Buf: ?Sized>(
            sendbuf: &Buf,
            recvbuf: &mut Buf::Single,
            op: MpiOp,
            root: Process,
        ) -> RmpiResult
        where
            Buf: Buffer;
        #[inline]
        fn allreduce<Buf: ?Sized>(
            sendbuf: &Buf,
            recvbuf: &mut Buf::Single,
            op: MpiOp,
            comm: &Communicator,
        ) -> RmpiResult
        where
            Buf: Buffer;

        #[inline]
        fn scan<Buf: ?Sized>(
            sendbuf: &Buf,
            recvbuf: &mut Buf::Single,
            op: MpiOp,
            comm: &Communicator,
        ) -> RmpiResult
        where
            Buf: Buffer;

        //TODO
        #[inline]
        fn scatter<SendBuf: ?Sized, RecvBuf: ?Sized>(
            sendbuf: &SendBuf,
            recvbuf: &mut RecvBuf,
            root: Process,
        ) -> RmpiResult
        where
            SendBuf: Buffer,
            RecvBuf: Buffer;
        #[inline]
        fn scatterv<SendBuf: ?Sized, RecvBuf: ?Sized>(
            sendbuf: &SendBuf,
            displs: &[c_int],
            recvbuf: &mut RecvBuf,
            root: Process,
        ) -> RmpiResult
        where
            SendBuf: Buffer,
            RecvBuf: Buffer;
    );
}
