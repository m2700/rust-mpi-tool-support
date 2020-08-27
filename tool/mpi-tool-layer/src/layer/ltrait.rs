use std::{
    mem::MaybeUninit,
    os::raw::{c_double, c_int},
};

use rmpi::pmpi_mode as rmpi;

use self::rmpi::{
    datatype::RawDatatype,
    request::{Request, RequestSlice},
    BufferMut, BufferRef, CStrMutPtr, Communicator, Group, MpiOp, Process, RmpiContext, RmpiResult,
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
        fn send<Buf>(buf: Buf, dest: Process, tag: Tag) -> RmpiResult
        where
            Buf: BufferRef;
        #[inline]
        fn bsend<Buf>(buf: Buf, dest: Process, tag: Tag) -> RmpiResult
        where
            Buf: BufferRef;
        #[inline]
        fn ssend<Buf>(buf: Buf, dest: Process, tag: Tag) -> RmpiResult
        where
            Buf: BufferRef;
        #[inline]
        fn rsend<Buf>(buf: Buf, dest: Process, tag: Tag) -> RmpiResult
        where
            Buf: BufferRef;

        #[inline]
        fn isend<'b, Buf: 'b>(buf: Buf, dest: Process, tag: Tag) -> RmpiResult<Request<'b>>
        where
            Buf: BufferRef;
        #[inline]
        fn ibsend<'b, Buf: 'b>(buf: Buf, dest: Process, tag: Tag) -> RmpiResult<Request<'b>>
        where
            Buf: BufferRef;
        #[inline]
        fn issend<'b, Buf: 'b>(buf: Buf, dest: Process, tag: Tag) -> RmpiResult<Request<'b>>
        where
            Buf: BufferRef;

        #[inline]
        fn irsend<'b, Buf: 'b>(buf: Buf, dest: Process, tag: Tag) -> RmpiResult<Request<'b>>
        where
            Buf: BufferRef;
        #[inline]
        fn recv<Buf>(
            buf: Buf,
            src: Process,
            tag: Tag,
            status_ignore: bool,
        ) -> RmpiResult<Option<Status>>
        where
            Buf: BufferMut;
        #[inline]
        fn irecv<'b, Buf: 'b>(buf: Buf, dest: Process, tag: Tag) -> RmpiResult<Request<'b>>
        where
            Buf: BufferMut;

        #[inline]
        fn sendrecv<SendBuf, RecvBuf>(
            sendbuf: SendBuf,
            dest: Process,
            sendtag: Tag,
            recvbuf: RecvBuf,
            src: Process,
            recvtag: Tag,
            status_ignore: bool,
        ) -> RmpiResult<Option<Status>>
        where
            SendBuf: BufferRef,
            RecvBuf: BufferMut;

        #[inline]
        fn bcast<Buf>(buf: Buf, root: Process) -> RmpiResult
        where
            Buf: BufferMut;

        #[inline]
        fn get_count(status: &Status, datatype: &RawDatatype) -> RmpiResult<c_int>;

        #[doc = "unsafe, because lifetime is unknown"]
        #[inline]
        unsafe fn buffer_attach(buffer: &'static mut [u8]) -> RmpiResult;
        #[doc = "unsafe, because lifetime is unknown"]
        #[inline]
        unsafe fn buffer_detach() -> RmpiResult<&'static mut [u8]>;

        #[inline]
        fn wait(
            request: Request,
            status_ignore: bool,
        ) -> RmpiResult<(Option<Status>, Option<Request>)>;
        #[inline]
        fn waitany(
            requests: &mut RequestSlice,
            status_ignore: bool,
        ) -> RmpiResult<(usize, Option<Status>)>;
        #[inline]
        fn waitall<'statuses>(
            requests: &mut RequestSlice,
            responses: &'statuses mut [MaybeUninit<Status>],
        ) -> RmpiResult<&'statuses mut [Status]>;

        #[inline]
        fn test(
            request: Request,
            status_ignore: bool,
        ) -> RmpiResult<Result<(Option<Status>, Option<Request>), Request>>;
        #[inline]
        fn testany(
            request: &mut RequestSlice,
            status_ignore: bool,
        ) -> RmpiResult<Option<(usize, Option<Status>)>>;

        #[inline]
        fn request_free(request: Request) -> RmpiResult;
        #[inline]
        fn cancel(request: Request) -> RmpiResult;

        // should I really use Buffers of different datatypes??
        #[inline]
        fn gather<SendBuf, RecvBuf>(
            sendbuf: SendBuf,
            recvbuf: RecvBuf,
            root: Process,
        ) -> RmpiResult
        where
            SendBuf: BufferRef,
            RecvBuf: BufferMut;
        #[inline]
        fn gatherv<SendBuf, RecvBuf>(
            sendbuf: SendBuf,
            recvbufs: &mut [RecvBuf],
            root: Process,
        ) -> RmpiResult
        where
            SendBuf: BufferRef,
            RecvBuf: BufferMut;
        #[inline]
        fn allgather<SendBuf, RecvBuf>(
            sendbuf: SendBuf,
            recvbuf: RecvBuf,
            comm: &Communicator,
        ) -> RmpiResult
        where
            SendBuf: BufferRef,
            RecvBuf: BufferMut;
        #[inline]
        fn allgatherv<SendBuf, RecvBuf>(
            sendbuf: SendBuf,
            recvbufs: &mut [RecvBuf],
            comm: &Communicator,
        ) -> RmpiResult
        where
            SendBuf: BufferRef,
            RecvBuf: BufferMut;

        #[inline]
        fn alltoall<SendBuf, RecvBuf>(
            sendbuf: SendBuf,
            recvbuf: RecvBuf,
            comm: &Communicator,
        ) -> RmpiResult
        where
            SendBuf: BufferRef,
            RecvBuf: BufferMut;
        #[inline]
        fn alltoallv<SendBuf, RecvBuf>(
            sendbufs: &[SendBuf],
            recvbufs: &mut [RecvBuf],
            comm: &Communicator,
        ) -> RmpiResult
        where
            SendBuf: BufferRef,
            RecvBuf: BufferMut;

        #[inline]
        fn reduce<Buf>(
            sendbuf: Buf,
            recvbuf: Buf::Mut,
            op: MpiOp,
            root: Process,
        ) -> RmpiResult
        where
            Buf: BufferRef;
        #[inline]
        fn allreduce<Buf>(
            sendbuf: Buf,
            recvbuf: Buf::Mut,
            op: MpiOp,
            comm: &Communicator,
        ) -> RmpiResult
        where
            Buf: BufferRef;

        #[inline]
        fn scan<Buf>(
            sendbuf: Buf,
            recvbuf: Buf::Mut,
            op: MpiOp,
            comm: &Communicator,
        ) -> RmpiResult
        where
            Buf: BufferRef;

        #[inline]
        fn scatter<SendBuf, RecvBuf>(
            sendbuf: SendBuf,
            recvbuf: RecvBuf,
            root: Process,
        ) -> RmpiResult
        where
            SendBuf: BufferRef,
            RecvBuf: BufferMut;
        #[inline]
        fn scatterv<SendBuf, RecvBuf>(
            sendbufs: &[SendBuf],
            recvbuf: RecvBuf,
            root: Process,
        ) -> RmpiResult
        where
            SendBuf: BufferRef,
            RecvBuf: BufferMut;
    );
}
