use std::os::raw::{c_double, c_int};

use rmpi::pmpi_mode as rmpi;

use self::rmpi::{
    request::{Request, RequestSlice},
    Buffer, CStrMutPtr, Communicator, Group, MpiDatatype, MpiOp, Process, RmpiContext, RmpiResult,
    Status, Tag,
};

pub trait MpiInterceptionLayer {
    trait_layer_function!(
        #[inline]
        fn init(args: &mut &mut [CStrMutPtr]) -> RmpiResult<Option<RmpiContext>>;
        #[inline]
        fn initialized() -> RmpiResult<bool>;
        #[inline]
        fn finalize() -> RmpiResult;
        #[inline]
        fn finalized() -> RmpiResult<bool>;

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
        fn comm_create(comm: &Communicator, group: &Group) -> RmpiResult<Communicator>;
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
        fn isend<'b, Buf: ?Sized>(buf: &'b Buf, dest: Process, tag: Tag) -> RmpiResult<Request<'b>>
        where
            Buf: Buffer;
        #[inline]
        fn ibsend<'b, Buf: ?Sized>(
            buf: &'b Buf,
            dest: Process,
            tag: Tag,
        ) -> RmpiResult<Request<'b>>
        where
            Buf: Buffer;
        #[inline]
        fn issend<'b, Buf: ?Sized>(
            buf: &'b Buf,
            dest: Process,
            tag: Tag,
        ) -> RmpiResult<Request<'b>>
        where
            Buf: Buffer;

        #[inline]
        fn irsend<'b, Buf: ?Sized>(
            buf: &'b Buf,
            dest: Process,
            tag: Tag,
        ) -> RmpiResult<Request<'b>>
        where
            Buf: Buffer;
        #[inline]
        fn recv<Buf: ?Sized>(buf: &mut Buf, src: Process, tag: Tag) -> RmpiResult<Status>
        where
            Buf: Buffer;

        #[inline]
        fn sendrecv<SendBuf: ?Sized, RecvBuf: ?Sized>(
            sendbuf: &SendBuf,
            dest: Process,
            sendtag: Tag,
            recvbuf: &mut RecvBuf,
            src: Process,
            recvtag: Tag,
        ) -> RmpiResult<Status>
        where
            SendBuf: Buffer,
            RecvBuf: Buffer;

        #[inline]
        fn bcast<Buf: ?Sized>(buf: &mut Buf, root: Process) -> RmpiResult;

        #[inline]
        fn get_count<Datatype>(status: &Status) -> RmpiResult<c_int>
        where
            Datatype: MpiDatatype;

        #[doc = "unsafe, because lifetime is unknown"]
        #[inline]
        unsafe fn buffer_attach(buffer: &'static mut [u8]) -> RmpiResult;
        #[doc = "unsafe, because lifetime is unknown"]
        #[inline]
        unsafe fn buffer_detach() -> RmpiResult<&'static mut [u8]>;

        #[inline]
        fn wait(request: Request) -> RmpiResult<(Status, Option<Request>)>;
        #[inline]
        fn waitany(requests: &mut RequestSlice) -> RmpiResult<(usize, Status)>;
        #[inline]
        fn waitall(requests: &mut RequestSlice, responses: &mut [Status]) -> RmpiResult;

        #[inline]
        fn test(request: Request) -> RmpiResult<Result<(Status, Option<Request>), Request>>;
        #[inline]
        fn testany(request: &mut RequestSlice) -> RmpiResult<Option<(usize, Status)>>;

        #[inline]
        fn request_free(request: Request) -> RmpiResult;
        #[inline]
        fn cancel(request: Request) -> RmpiResult;

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
        fn alltoall<SendBuf: ?Sized, RecvBuf: ?Sized>(
            sendbuf: &SendBuf,
            recvbuf: &mut RecvBuf,
            comm: &Communicator,
        ) -> RmpiResult
        where
            SendBuf: Buffer,
            RecvBuf: Buffer;
        #[inline]
        fn alltoallv<SendBuf: ?Sized, RecvBuf: ?Sized>(
            sendbufs: &[&SendBuf],
            recvbufs: &mut [&mut RecvBuf],
            comm: &Communicator,
        ) -> RmpiResult
        where
            SendBuf: Buffer,
            RecvBuf: Buffer;

        #[inline]
        fn reduce<Buf: ?Sized>(
            sendbuf: &Buf,
            recvbuf: Option<&mut Buf::Single>,
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
