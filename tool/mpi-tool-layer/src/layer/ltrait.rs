use std::{
    mem::MaybeUninit,
    os::raw::{c_double, c_int},
};

use rmpi::pmpi_mode as rmpi;

use self::rmpi::{
    datatype::RawDatatype,
    request::{Request, RequestSlice},
    CStrMutPtr, Communicator, Error, Group, MpiOp, Process, RmpiContext, RmpiResult, Status, Tag,
    TypeDynamicBufferMut, TypeDynamicBufferRef,
};

use crate::RawMpiInterceptionLayer;

pub trait MpiInterceptionLayer: RawMpiInterceptionLayer {
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
        fn comm_create<'ctx>(
            rmpi_ctx: &RmpiContext,
            comm: &Communicator<'ctx>,
            group: &Group,
        ) -> RmpiResult<Communicator<'ctx>>;
        #[inline]
        fn comm_free(rmpi_ctx: &RmpiContext, comm: Communicator) -> RmpiResult;
        #[inline]
        fn abort(rmpi_ctx: &RmpiContext, comm: Communicator, error: Error) -> RmpiResult;

        #[inline]
        fn send(
            rmpi_ctx: &RmpiContext,
            buf: TypeDynamicBufferRef,
            dest: Process,
            tag: Tag,
        ) -> RmpiResult;
        #[inline]
        fn bsend(
            rmpi_ctx: &RmpiContext,
            buf: TypeDynamicBufferRef,
            dest: Process,
            tag: Tag,
        ) -> RmpiResult;
        #[inline]
        fn ssend(
            rmpi_ctx: &RmpiContext,
            buf: TypeDynamicBufferRef,
            dest: Process,
            tag: Tag,
        ) -> RmpiResult;
        #[inline]
        fn rsend(
            rmpi_ctx: &RmpiContext,
            buf: TypeDynamicBufferRef,
            dest: Process,
            tag: Tag,
        ) -> RmpiResult;

        #[inline]
        fn isend<'b>(
            rmpi_ctx: &RmpiContext,
            buf: TypeDynamicBufferRef<'b>,
            dest: Process,
            tag: Tag,
        ) -> RmpiResult<Request<'b>>;
        #[inline]
        fn ibsend<'b>(
            rmpi_ctx: &RmpiContext,
            buf: TypeDynamicBufferRef<'b>,
            dest: Process,
            tag: Tag,
        ) -> RmpiResult<Request<'b>>;
        #[inline]
        fn issend<'b>(
            rmpi_ctx: &RmpiContext,
            buf: TypeDynamicBufferRef<'b>,
            dest: Process,
            tag: Tag,
        ) -> RmpiResult<Request<'b>>;
        #[inline]
        fn irsend<'b>(
            rmpi_ctx: &RmpiContext,
            buf: TypeDynamicBufferRef<'b>,
            dest: Process,
            tag: Tag,
        ) -> RmpiResult<Request<'b>>;

        #[inline]
        fn recv(
            rmpi_ctx: &RmpiContext,
            buf: TypeDynamicBufferMut,
            src: Process,
            tag: Tag,
            status_ignore: bool,
        ) -> RmpiResult<Option<Status>>;
        #[inline]
        fn irecv<'b>(
            rmpi_ctx: &RmpiContext,
            buf: TypeDynamicBufferMut<'b>,
            dest: Process,
            tag: Tag,
        ) -> RmpiResult<Request<'b>>;

        #[inline]
        fn sendrecv(
            rmpi_ctx: &RmpiContext,
            sendbuf: TypeDynamicBufferRef,
            dest: Process,
            sendtag: Tag,
            recvbuf: TypeDynamicBufferMut,
            src: Process,
            recvtag: Tag,
            status_ignore: bool,
        ) -> RmpiResult<Option<Status>>;

        #[inline]
        fn bcast(rmpi_ctx: &RmpiContext, buf: TypeDynamicBufferMut, root: Process) -> RmpiResult;

        #[inline]
        fn get_count(
            rmpi_ctx: &RmpiContext,
            status: &Status,
            datatype: &RawDatatype,
        ) -> RmpiResult<Option<c_int>>;

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
        ) -> RmpiResult<Option<&'statuses mut [Status]>>;

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
        fn testall<'statuses>(
            rmpi_ctx: &RmpiContext,
            requests: &mut RequestSlice,
            responses: &'statuses mut [MaybeUninit<Status>],
        ) -> RmpiResult<Option<&'statuses mut [Status]>>;

        #[inline]
        unsafe fn request_free(rmpi_ctx: &RmpiContext, request: Request) -> RmpiResult;
        #[inline]
        fn cancel(rmpi_ctx: &RmpiContext, request: Request) -> RmpiResult;

        // should I really use Buffers of different datatypes??
        #[inline]
        fn gather(
            rmpi_ctx: &RmpiContext,
            sendbuf: TypeDynamicBufferRef,
            recvbuf: TypeDynamicBufferMut,
            root: Process,
        ) -> RmpiResult;
        #[inline]
        fn gatherv(
            rmpi_ctx: &RmpiContext,
            sendbuf: TypeDynamicBufferRef,
            recvbufs: &mut [TypeDynamicBufferMut],
            root: Process,
        ) -> RmpiResult;
        #[inline]
        fn allgather(
            rmpi_ctx: &RmpiContext,
            sendbuf: TypeDynamicBufferRef,
            recvbuf: TypeDynamicBufferMut,
            comm: &Communicator,
        ) -> RmpiResult;
        #[inline]
        fn allgatherv(
            rmpi_ctx: &RmpiContext,
            sendbuf: TypeDynamicBufferRef,
            recvbufs: &mut [TypeDynamicBufferMut],
            comm: &Communicator,
        ) -> RmpiResult;

        #[inline]
        fn alltoall(
            rmpi_ctx: &RmpiContext,
            sendbuf: TypeDynamicBufferRef,
            recvbuf: TypeDynamicBufferMut,
            comm: &Communicator,
        ) -> RmpiResult;
        #[inline]
        fn alltoallv(
            rmpi_ctx: &RmpiContext,
            sendbufs: &[TypeDynamicBufferRef],
            recvbufs: &mut [TypeDynamicBufferMut],
            comm: &Communicator,
        ) -> RmpiResult;

        #[inline]
        fn reduce(
            rmpi_ctx: &RmpiContext,
            sendbuf: TypeDynamicBufferRef,
            recvbuf: TypeDynamicBufferMut,
            op: MpiOp,
            root: Process,
        ) -> RmpiResult;
        #[inline]
        fn allreduce(
            rmpi_ctx: &RmpiContext,
            sendbuf: TypeDynamicBufferRef,
            recvbuf: TypeDynamicBufferMut,
            op: MpiOp,
            comm: &Communicator,
        ) -> RmpiResult;

        #[inline]
        fn scan(
            rmpi_ctx: &RmpiContext,
            sendbuf: TypeDynamicBufferRef,
            recvbuf: TypeDynamicBufferMut,
            op: MpiOp,
            comm: &Communicator,
        ) -> RmpiResult;

        #[inline]
        fn scatter(
            rmpi_ctx: &RmpiContext,
            sendbuf: TypeDynamicBufferRef,
            recvbuf: TypeDynamicBufferMut,
            root: Process,
        ) -> RmpiResult;
        #[inline]
        fn scatterv(
            rmpi_ctx: &RmpiContext,
            sendbufs: &[TypeDynamicBufferRef],
            recvbuf: TypeDynamicBufferMut,
            root: Process,
        ) -> RmpiResult;
    );
}
