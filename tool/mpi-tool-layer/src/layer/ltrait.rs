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
        fn finalize(rmpi_ctx: RmpiContext) -> RmpiResult;
        #[inline]
        fn finalized() -> RmpiResult<bool>;

        #[inline]
        fn wtime(rmpi_ctx: &RmpiContext) -> c_double;
        #[inline]
        fn wtick(rmpi_ctx: &RmpiContext) -> c_double;

        #[inline]
        fn barrier(rmpi_ctx: &RmpiContext, comm: &Communicator) -> RmpiResult;

        #[inline]
        fn group_incl(rmpi_ctx: &RmpiContext, group: &Group, ranks: &[c_int]) -> RmpiResult<Group>;
        #[inline]
        fn group_free(rmpi_ctx: &RmpiContext, group: Group) -> RmpiResult;

        #[inline]
        fn comm_size(rmpi_ctx: &RmpiContext, comm: &Communicator) -> RmpiResult<c_int>;
        #[inline]
        fn comm_rank(rmpi_ctx: &RmpiContext, comm: &Communicator) -> RmpiResult<c_int>;
        #[inline]
        fn comm_create(
            rmpi_ctx: &RmpiContext,
            comm: &Communicator,
            group: &Group,
        ) -> RmpiResult<Communicator>;
        #[inline]
        fn comm_free(rmpi_ctx: &RmpiContext, comm: Communicator) -> RmpiResult;

        #[inline]
        fn send<Buf>(rmpi_ctx: &RmpiContext, buf: Buf, dest: Process, tag: Tag) -> RmpiResult
        where
            Buf: BufferRef;
        #[inline]
        fn bsend<Buf>(rmpi_ctx: &RmpiContext, buf: Buf, dest: Process, tag: Tag) -> RmpiResult
        where
            Buf: BufferRef;
        #[inline]
        fn ssend<Buf>(rmpi_ctx: &RmpiContext, buf: Buf, dest: Process, tag: Tag) -> RmpiResult
        where
            Buf: BufferRef;
        #[inline]
        fn rsend<Buf>(rmpi_ctx: &RmpiContext, buf: Buf, dest: Process, tag: Tag) -> RmpiResult
        where
            Buf: BufferRef;

        #[inline]
        fn isend<'b, Buf: 'b>(
            rmpi_ctx: &RmpiContext,
            buf: Buf,
            dest: Process,
            tag: Tag,
        ) -> RmpiResult<Request<'b>>
        where
            Buf: BufferRef;
        #[inline]
        fn ibsend<'b, Buf: 'b>(
            rmpi_ctx: &RmpiContext,
            buf: Buf,
            dest: Process,
            tag: Tag,
        ) -> RmpiResult<Request<'b>>
        where
            Buf: BufferRef;
        #[inline]
        fn issend<'b, Buf: 'b>(
            rmpi_ctx: &RmpiContext,
            buf: Buf,
            dest: Process,
            tag: Tag,
        ) -> RmpiResult<Request<'b>>
        where
            Buf: BufferRef;

        #[inline]
        fn irsend<'b, Buf: 'b>(
            rmpi_ctx: &RmpiContext,
            buf: Buf,
            dest: Process,
            tag: Tag,
        ) -> RmpiResult<Request<'b>>
        where
            Buf: BufferRef;
        #[inline]
        fn recv<Buf>(
            rmpi_ctx: &RmpiContext,
            buf: Buf,
            src: Process,
            tag: Tag,
            status_ignore: bool,
        ) -> RmpiResult<Option<Status>>
        where
            Buf: BufferMut;
        #[inline]
        fn irecv<'b, Buf: 'b>(
            rmpi_ctx: &RmpiContext,
            buf: Buf,
            dest: Process,
            tag: Tag,
        ) -> RmpiResult<Request<'b>>
        where
            Buf: BufferMut;

        #[inline]
        fn sendrecv<SendBuf, RecvBuf>(
            rmpi_ctx: &RmpiContext,
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
        fn bcast<Buf>(rmpi_ctx: &RmpiContext, buf: Buf, root: Process) -> RmpiResult
        where
            Buf: BufferMut;

        #[inline]
        fn get_count(
            rmpi_ctx: &RmpiContext,
            status: &Status,
            datatype: &RawDatatype,
        ) -> RmpiResult<c_int>;

        #[doc = "unsafe, because lifetime is unknown"]
        #[inline]
        unsafe fn buffer_attach(rmpi_ctx: &RmpiContext, buffer: &'static mut [u8]) -> RmpiResult;
        #[doc = "unsafe, because lifetime is unknown"]
        #[inline]
        unsafe fn buffer_detach(rmpi_ctx: &RmpiContext) -> RmpiResult<&'static mut [u8]>;

        #[inline]
        fn wait<'r>(
            rmpi_ctx: &RmpiContext,
            request: Request<'r>,
            status_ignore: bool,
        ) -> RmpiResult<(Option<Status>, Option<Request<'r>>)>;
        #[inline]
        fn waitany(
            rmpi_ctx: &RmpiContext,
            requests: &mut RequestSlice,
            status_ignore: bool,
        ) -> RmpiResult<(usize, Option<Status>)>;
        #[inline]
        fn waitall<'statuses>(
            rmpi_ctx: &RmpiContext,
            requests: &mut RequestSlice,
            responses: &'statuses mut [MaybeUninit<Status>],
        ) -> RmpiResult<&'statuses mut [Status]>;

        #[inline]
        fn test<'r>(
            rmpi_ctx: &RmpiContext,
            request: Request<'r>,
            status_ignore: bool,
        ) -> RmpiResult<Result<(Option<Status>, Option<Request<'r>>), Request<'r>>>;
        #[inline]
        fn testany(
            rmpi_ctx: &RmpiContext,
            request: &mut RequestSlice,
            status_ignore: bool,
        ) -> RmpiResult<Option<(usize, Option<Status>)>>;

        #[inline]
        fn request_free(rmpi_ctx: &RmpiContext, request: Request) -> RmpiResult;
        #[inline]
        fn cancel(rmpi_ctx: &RmpiContext, request: Request) -> RmpiResult;

        // should I really use Buffers of different datatypes??
        #[inline]
        fn gather<SendBuf, RecvBuf>(
            rmpi_ctx: &RmpiContext,
            sendbuf: SendBuf,
            recvbuf: RecvBuf,
            root: Process,
        ) -> RmpiResult
        where
            SendBuf: BufferRef,
            RecvBuf: BufferMut;
        #[inline]
        fn gatherv<SendBuf, RecvBuf>(
            rmpi_ctx: &RmpiContext,
            sendbuf: SendBuf,
            recvbufs: &mut [RecvBuf],
            root: Process,
        ) -> RmpiResult
        where
            SendBuf: BufferRef,
            RecvBuf: BufferMut;
        #[inline]
        fn allgather<SendBuf, RecvBuf>(
            rmpi_ctx: &RmpiContext,
            sendbuf: SendBuf,
            recvbuf: RecvBuf,
            comm: &Communicator,
        ) -> RmpiResult
        where
            SendBuf: BufferRef,
            RecvBuf: BufferMut;
        #[inline]
        fn allgatherv<SendBuf, RecvBuf>(
            rmpi_ctx: &RmpiContext,
            sendbuf: SendBuf,
            recvbufs: &mut [RecvBuf],
            comm: &Communicator,
        ) -> RmpiResult
        where
            SendBuf: BufferRef,
            RecvBuf: BufferMut;

        #[inline]
        fn alltoall<SendBuf, RecvBuf>(
            rmpi_ctx: &RmpiContext,
            sendbuf: SendBuf,
            recvbuf: RecvBuf,
            comm: &Communicator,
        ) -> RmpiResult
        where
            SendBuf: BufferRef,
            RecvBuf: BufferMut;
        #[inline]
        fn alltoallv<SendBuf, RecvBuf>(
            rmpi_ctx: &RmpiContext,
            sendbufs: &[SendBuf],
            recvbufs: &mut [RecvBuf],
            comm: &Communicator,
        ) -> RmpiResult
        where
            SendBuf: BufferRef,
            RecvBuf: BufferMut;

        #[inline]
        fn reduce<Buf>(
            rmpi_ctx: &RmpiContext,
            sendbuf: Buf,
            recvbuf: Buf::Mut,
            op: MpiOp,
            root: Process,
        ) -> RmpiResult
        where
            Buf: BufferRef;
        #[inline]
        fn allreduce<Buf>(
            rmpi_ctx: &RmpiContext,
            sendbuf: Buf,
            recvbuf: Buf::Mut,
            op: MpiOp,
            comm: &Communicator,
        ) -> RmpiResult
        where
            Buf: BufferRef;

        #[inline]
        fn scan<Buf>(
            rmpi_ctx: &RmpiContext,
            sendbuf: Buf,
            recvbuf: Buf::Mut,
            op: MpiOp,
            comm: &Communicator,
        ) -> RmpiResult
        where
            Buf: BufferRef;

        #[inline]
        fn scatter<SendBuf, RecvBuf>(
            rmpi_ctx: &RmpiContext,
            sendbuf: SendBuf,
            recvbuf: RecvBuf,
            root: Process,
        ) -> RmpiResult
        where
            SendBuf: BufferRef,
            RecvBuf: BufferMut;
        #[inline]
        fn scatterv<SendBuf, RecvBuf>(
            rmpi_ctx: &RmpiContext,
            sendbufs: &[SendBuf],
            recvbuf: RecvBuf,
            root: Process,
        ) -> RmpiResult
        where
            SendBuf: BufferRef,
            RecvBuf: BufferMut;
    );
}
