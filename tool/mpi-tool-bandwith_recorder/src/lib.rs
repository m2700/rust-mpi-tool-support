use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};

use self::rmpi::{
    BufferRef, BufferRefKind, Process, RmpiContext, RmpiResult, Status, Tag, TypeDynamicBufferMut,
    TypeDynamicBufferRef,
};
use mpi_tool_layer::MpiInterceptionLayer;
use qmpi_tool_creator::install_qmpi_layer as install_mpi_layer;
use rmpi::pmpi_mode as rmpi;

pub mod tool {
    use super::install_mpi_layer;
    install_mpi_layer!(super::MyQmpiLayer);
}

const ATC0: AtomicUsize = AtomicUsize::new(0);

static COUNTERS: [AtomicUsize; 12] = [
    ATC0, ATC0, ATC0, ATC0, ATC0, ATC0, ATC0, ATC0, ATC0, ATC0, ATC0, ATC0,
];

static I8_SEND_COUNT: &AtomicUsize = &COUNTERS[0];
static I16_SEND_COUNT: &AtomicUsize = &COUNTERS[1];
static I32_SEND_COUNT: &AtomicUsize = &COUNTERS[2];
static I64_SEND_COUNT: &AtomicUsize = &COUNTERS[3];
static FLOAT_SEND_COUNT: &AtomicUsize = &COUNTERS[4];
static DOUBLE_SEND_COUNT: &AtomicUsize = &COUNTERS[5];

static I8_RECV_COUNT: &AtomicUsize = &COUNTERS[6];
static I16_RECV_COUNT: &AtomicUsize = &COUNTERS[7];
static I32_RECV_COUNT: &AtomicUsize = &COUNTERS[8];
static I64_RECV_COUNT: &AtomicUsize = &COUNTERS[9];
static FLOAT_RECV_COUNT: &AtomicUsize = &COUNTERS[10];
static DOUBLE_RECV_COUNT: &AtomicUsize = &COUNTERS[11];

fn send_data_record(buf: TypeDynamicBufferRef) {
    match buf.kind_ref() {
        BufferRefKind::U8(buf) => I8_SEND_COUNT.fetch_add(buf.len(), Relaxed),
        BufferRefKind::I8(buf) => I8_SEND_COUNT.fetch_add(buf.len(), Relaxed),

        BufferRefKind::U16(buf) => I16_SEND_COUNT.fetch_add(buf.len(), Relaxed),
        BufferRefKind::I16(buf) => I16_SEND_COUNT.fetch_add(buf.len(), Relaxed),

        BufferRefKind::U32(buf) => I32_SEND_COUNT.fetch_add(buf.len(), Relaxed),
        BufferRefKind::I32(buf) => I32_SEND_COUNT.fetch_add(buf.len(), Relaxed),

        BufferRefKind::U64(buf) => I64_SEND_COUNT.fetch_add(buf.len(), Relaxed),
        BufferRefKind::I64(buf) => I64_SEND_COUNT.fetch_add(buf.len(), Relaxed),

        BufferRefKind::Float(buf) => FLOAT_SEND_COUNT.fetch_add(buf.len(), Relaxed),
        BufferRefKind::Double(buf) => DOUBLE_SEND_COUNT.fetch_add(buf.len(), Relaxed),

        _ => 0, /*ignored*/
    };
}
fn recv_data_record(buf: TypeDynamicBufferMut) {
    match buf.kind_ref() {
        BufferRefKind::U8(buf) => I8_RECV_COUNT.fetch_add(buf.len(), Relaxed),
        BufferRefKind::I8(buf) => I8_RECV_COUNT.fetch_add(buf.len(), Relaxed),

        BufferRefKind::U16(buf) => I16_RECV_COUNT.fetch_add(buf.len(), Relaxed),
        BufferRefKind::I16(buf) => I16_RECV_COUNT.fetch_add(buf.len(), Relaxed),

        BufferRefKind::U32(buf) => I32_RECV_COUNT.fetch_add(buf.len(), Relaxed),
        BufferRefKind::I32(buf) => I32_RECV_COUNT.fetch_add(buf.len(), Relaxed),

        BufferRefKind::U64(buf) => I64_RECV_COUNT.fetch_add(buf.len(), Relaxed),
        BufferRefKind::I64(buf) => I64_RECV_COUNT.fetch_add(buf.len(), Relaxed),

        BufferRefKind::Float(buf) => FLOAT_RECV_COUNT.fetch_add(buf.len(), Relaxed),
        BufferRefKind::Double(buf) => DOUBLE_RECV_COUNT.fetch_add(buf.len(), Relaxed),

        _ => 0, /*ignored*/
    };
}

struct MyQmpiLayer;
impl MpiInterceptionLayer for MyQmpiLayer {
    fn finalize<F>(next_f: F, rmpi_ctx: RmpiContext) -> RmpiResult
    where
        F: FnOnce(RmpiContext) -> RmpiResult,
    {
        next_f(rmpi_ctx)
    }

    fn send<F>(
        next_f: F,
        rmpi_ctx: &RmpiContext,
        buf: TypeDynamicBufferRef,
        dest: Process,
        tag: Tag,
    ) -> RmpiResult
    where
        F: FnOnce(&RmpiContext, TypeDynamicBufferRef, Process, Tag) -> RmpiResult,
    {
        let res = next_f(rmpi_ctx, buf, dest, tag);
        if res.is_ok() {
            send_data_record(buf);
        }
        res
    }
    fn bsend<F>(
        next_f: F,
        rmpi_ctx: &RmpiContext,
        buf: TypeDynamicBufferRef,
        dest: Process,
        tag: Tag,
    ) -> RmpiResult
    where
        F: FnOnce(&RmpiContext, TypeDynamicBufferRef, Process, Tag) -> RmpiResult,
    {
        let res = next_f(rmpi_ctx, buf, dest, tag);
        if res.is_ok() {
            send_data_record(buf);
        }
        res
    }
    fn ssend<F>(
        next_f: F,
        rmpi_ctx: &RmpiContext,
        buf: TypeDynamicBufferRef,
        dest: Process,
        tag: Tag,
    ) -> RmpiResult
    where
        F: FnOnce(&RmpiContext, TypeDynamicBufferRef, Process, Tag) -> RmpiResult,
    {
        let res = next_f(rmpi_ctx, buf, dest, tag);
        if res.is_ok() {
            send_data_record(buf);
        }
        res
    }
    fn rsend<F>(
        next_f: F,
        rmpi_ctx: &RmpiContext,
        buf: TypeDynamicBufferRef,
        dest: Process,
        tag: Tag,
    ) -> RmpiResult
    where
        F: FnOnce(&RmpiContext, TypeDynamicBufferRef, Process, Tag) -> RmpiResult,
    {
        let res = next_f(rmpi_ctx, buf, dest, tag);
        if res.is_ok() {
            send_data_record(buf);
        }
        res
    }

    fn recv<F>(
        next_f: F,
        rmpi_ctx: &RmpiContext,
        mut buf: TypeDynamicBufferMut,
        src: Process,
        tag: Tag,
        status_ignore: bool,
    ) -> RmpiResult<Option<Status>>
    where
        F: FnOnce(
            &RmpiContext,
            TypeDynamicBufferMut,
            Process,
            Tag,
            bool,
        ) -> RmpiResult<Option<Status>>,
    {
        let res = next_f(rmpi_ctx, buf.as_mut(), src, tag, status_ignore);
        if res.is_ok() {
            recv_data_record(buf);
        }
        res
    }

    // #[inline]
    // fn sendrecv<SendBuf, RecvBuf>(
    //     sendbuf: SendBuf,
    //     dest: Process,
    //     sendtag: Tag,
    //     recvbuf: RecvBuf,
    //     src: Process,
    //     recvtag: Tag,
    //     status_ignore: bool,
    // ) -> RmpiResult<Option<Status>>
    // where
    //     SendBuf: BufferRef,
    //     RecvBuf: BufferMut;

    // #[inline]
    // fn bcast<Buf>(buf: Buf, root: Process) -> RmpiResult
    // where
    //     Buf: BufferMut;

    // #[inline]
    // fn get_count(status: &Status, datatype: &RawDatatype) -> RmpiResult<c_int>;

    // #[doc = "unsafe, because lifetime is unknown"]
    // #[inline]
    // unsafe fn buffer_attach(buffer: &'static mut [u8]) -> RmpiResult;
    // #[doc = "unsafe, because lifetime is unknown"]
    // #[inline]
    // unsafe fn buffer_detach() -> RmpiResult<&'static mut [u8]>;

    // #[inline]
    // fn wait(
    //     request: Request,
    //     status_ignore: bool,
    // ) -> RmpiResult<(Option<Status>, Option<Request>)>;
    // #[inline]
    // fn waitany(
    //     requests: &mut RequestSlice,
    //     status_ignore: bool,
    // ) -> RmpiResult<(usize, Option<Status>)>;
    // #[inline]
    // fn waitall<'statuses>(
    //     requests: &mut RequestSlice,
    //     responses: &'statuses mut [MaybeUninit<Status>],
    // ) -> RmpiResult<&'statuses mut [Status]>;

    // #[inline]
    // fn test(
    //     request: Request,
    //     status_ignore: bool,
    // ) -> RmpiResult<Result<(Option<Status>, Option<Request>), Request>>;
    // #[inline]
    // fn testany(
    //     request: &mut RequestSlice,
    //     status_ignore: bool,
    // ) -> RmpiResult<Option<(usize, Option<Status>)>>;

    // #[inline]
    // fn request_free(request: Request) -> RmpiResult;
    // #[inline]
    // fn cancel(request: Request) -> RmpiResult;

    // // should I really use Buffers of different datatypes??
    // #[inline]
    // fn gather<SendBuf, RecvBuf>(
    //     sendbuf: SendBuf,
    //     recvbuf: RecvBuf,
    //     root: Process,
    // ) -> RmpiResult
    // where
    //     SendBuf: BufferRef,
    //     RecvBuf: BufferMut;
    // #[inline]
    // fn gatherv<SendBuf, RecvBuf>(
    //     sendbuf: SendBuf,
    //     recvbufs: &mut [RecvBuf],
    //     root: Process,
    // ) -> RmpiResult
    // where
    //     SendBuf: BufferRef,
    //     RecvBuf: BufferMut;
    // #[inline]
    // fn allgather<SendBuf, RecvBuf>(
    //     sendbuf: SendBuf,
    //     recvbuf: RecvBuf,
    //     comm: &Communicator,
    // ) -> RmpiResult
    // where
    //     SendBuf: BufferRef,
    //     RecvBuf: BufferMut;
    // #[inline]
    // fn allgatherv<SendBuf, RecvBuf>(
    //     sendbuf: SendBuf,
    //     recvbufs: &mut [RecvBuf],
    //     comm: &Communicator,
    // ) -> RmpiResult
    // where
    //     SendBuf: BufferRef,
    //     RecvBuf: BufferMut;

    // #[inline]
    // fn alltoall<SendBuf, RecvBuf>(
    //     sendbuf: SendBuf,
    //     recvbuf: RecvBuf,
    //     comm: &Communicator,
    // ) -> RmpiResult
    // where
    //     SendBuf: BufferRef,
    //     RecvBuf: BufferMut;
    // #[inline]
    // fn alltoallv<SendBuf, RecvBuf>(
    //     sendbufs: &[SendBuf],
    //     recvbufs: &mut [RecvBuf],
    //     comm: &Communicator,
    // ) -> RmpiResult
    // where
    //     SendBuf: BufferRef,
    //     RecvBuf: BufferMut;

    // #[inline]
    // fn reduce<Buf>(
    //     sendbuf: Buf,
    //     recvbuf: Buf::Mut,
    //     op: MpiOp,
    //     root: Process,
    // ) -> RmpiResult
    // where
    //     Buf: BufferRef;
    // #[inline]
    // fn allreduce<Buf>(
    //     sendbuf: Buf,
    //     recvbuf: Buf::Mut,
    //     op: MpiOp,
    //     comm: &Communicator,
    // ) -> RmpiResult
    // where
    //     Buf: BufferRef;

    // #[inline]
    // fn scan<Buf>(
    //     sendbuf: Buf,
    //     recvbuf: Buf::Mut,
    //     op: MpiOp,
    //     comm: &Communicator,
    // ) -> RmpiResult
    // where
    //     Buf: BufferRef;

    // #[inline]
    // fn scatter<SendBuf, RecvBuf>(
    //     sendbuf: SendBuf,
    //     recvbuf: RecvBuf,
    //     root: Process,
    // ) -> RmpiResult
    // where
    //     SendBuf: BufferRef,
    //     RecvBuf: BufferMut;
    // #[inline]
    // fn scatterv<SendBuf, RecvBuf>(
    //     sendbufs: &[SendBuf],
    //     recvbuf: RecvBuf,
    //     root: Process,
    // ) -> RmpiResult
    // where
    //     SendBuf: BufferRef,
    //     RecvBuf: BufferMut;
}
