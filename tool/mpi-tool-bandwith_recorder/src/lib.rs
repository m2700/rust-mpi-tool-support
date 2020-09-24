use std::{
    env,
    sync::atomic::{AtomicUsize, Ordering::Relaxed},
};

use self::rmpi::{
    BufferRef, BufferRefKind, Communicator, MpiOp, Process, RmpiContext, RmpiResult, Status, Tag,
    TypeDynamicBufferMut, TypeDynamicBufferRef,
};
use mpi_tool_layer::MpiInterceptionLayer;
use qmpi_tool_creator::install_qmpi_layer as install_mpi_layer;
use rmpi::pmpi_mode as rmpi;

pub mod tool {
    use super::install_mpi_layer;
    install_mpi_layer!(super::MyQmpiLayer);
}

const ATC0: AtomicUsize = AtomicUsize::new(0);

const COUNTERS_LEN: usize = 18;
static COUNTERS: [AtomicUsize; COUNTERS_LEN] = [
    ATC0, ATC0, ATC0, ATC0, ATC0, ATC0, ATC0, ATC0, ATC0, ATC0, ATC0, ATC0, ATC0, ATC0, ATC0, ATC0,
    ATC0, ATC0,
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

static I8_RDC_COUNT: &AtomicUsize = &COUNTERS[12];
static I16_RDC_COUNT: &AtomicUsize = &COUNTERS[13];
static I32_RDC_COUNT: &AtomicUsize = &COUNTERS[14];
static I64_RDC_COUNT: &AtomicUsize = &COUNTERS[15];
static FLOAT_RDC_COUNT: &AtomicUsize = &COUNTERS[16];
static DOUBLE_RDC_COUNT: &AtomicUsize = &COUNTERS[17];

const COUNTER_NAMES: [&str; COUNTERS_LEN] = [
    "8-bit Intergers sent ",
    "16-bit Intergers sent",
    "32-bit Intergers sent",
    "64-bit Intergers sent",
    "Floats sent          ",
    "Doubles sent         ",
    "8-bit Intergers received ",
    "16-bit Intergers received",
    "32-bit Intergers received",
    "64-bit Intergers received",
    "Floats received          ",
    "Doubles received         ",
    "8-bit Intergers reduced ",
    "16-bit Intergers reduced",
    "32-bit Intergers reduced",
    "64-bit Intergers reduced",
    "Floats reduced          ",
    "Doubles reduced         ",
];

fn send_data_record(buf: TypeDynamicBufferRef, recv_count: usize) {
    let send_len = buf.len() * recv_count;
    match buf.kind_ref() {
        BufferRefKind::U8(_) => I8_SEND_COUNT.fetch_add(send_len, Relaxed),
        BufferRefKind::I8(_) => I8_SEND_COUNT.fetch_add(send_len, Relaxed),

        BufferRefKind::U16(_) => I16_SEND_COUNT.fetch_add(send_len, Relaxed),
        BufferRefKind::I16(_) => I16_SEND_COUNT.fetch_add(send_len, Relaxed),

        BufferRefKind::U32(_) => I32_SEND_COUNT.fetch_add(send_len, Relaxed),
        BufferRefKind::I32(_) => I32_SEND_COUNT.fetch_add(send_len, Relaxed),

        BufferRefKind::U64(_) => I64_SEND_COUNT.fetch_add(send_len, Relaxed),
        BufferRefKind::I64(_) => I64_SEND_COUNT.fetch_add(send_len, Relaxed),

        BufferRefKind::Float(_) => FLOAT_SEND_COUNT.fetch_add(send_len, Relaxed),
        BufferRefKind::Double(_) => DOUBLE_SEND_COUNT.fetch_add(send_len, Relaxed),

        _ => 0, /*ignored*/
    };
}
fn recv_data_record(buf: TypeDynamicBufferRef) {
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
fn reduce_data_record(buf: TypeDynamicBufferRef) {
    match buf.kind_ref() {
        BufferRefKind::U8(buf) => I8_RDC_COUNT.fetch_add(buf.len(), Relaxed),
        BufferRefKind::I8(buf) => I8_RDC_COUNT.fetch_add(buf.len(), Relaxed),

        BufferRefKind::U16(buf) => I16_RDC_COUNT.fetch_add(buf.len(), Relaxed),
        BufferRefKind::I16(buf) => I16_RDC_COUNT.fetch_add(buf.len(), Relaxed),

        BufferRefKind::U32(buf) => I32_RDC_COUNT.fetch_add(buf.len(), Relaxed),
        BufferRefKind::I32(buf) => I32_RDC_COUNT.fetch_add(buf.len(), Relaxed),

        BufferRefKind::U64(buf) => I64_RDC_COUNT.fetch_add(buf.len(), Relaxed),
        BufferRefKind::I64(buf) => I64_RDC_COUNT.fetch_add(buf.len(), Relaxed),

        BufferRefKind::Float(buf) => FLOAT_RDC_COUNT.fetch_add(buf.len(), Relaxed),
        BufferRefKind::Double(buf) => DOUBLE_RDC_COUNT.fetch_add(buf.len(), Relaxed),

        _ => 0, /*ignored*/
    };
}

struct MyQmpiLayer;
impl MpiInterceptionLayer for MyQmpiLayer {
    fn finalize<F>(next_f: F, rmpi_ctx: RmpiContext) -> RmpiResult
    where
        F: FnOnce(RmpiContext) -> RmpiResult,
    {
        let comm_world = rmpi_ctx.comm_world();

        let mut counters = [0; COUNTERS_LEN];
        for cnt_idx in 0..counters.len() {
            counters[cnt_idx] = COUNTERS[cnt_idx].load(Relaxed);
        }

        let mut smmd_counters = [0; COUNTERS_LEN];
        comm_world
            .get_process(0)
            .reduce(&counters[..], &mut smmd_counters[..], MpiOp::Sum)
            .unwrap();
        let current_rank = comm_world.current_process()?.rank();
        drop(comm_world);

        next_f(rmpi_ctx)?;

        let fin_dbg_cnf = env::var("FINALIZE_DEBUG_CONFIRM");
        if current_rank == 0
            && (fin_dbg_cnf.is_err() || fin_dbg_cnf.as_ref().map(|s| &**s) == Ok("1"))
        {
            for counter_id in 0..COUNTERS_LEN {
                let count = smmd_counters[counter_id];
                if count != 0 {
                    println!("{}: {}", COUNTER_NAMES[counter_id], count);
                }
            }
        }

        Ok(())
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
            send_data_record(buf, 1);
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
            send_data_record(buf, 1);
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
            send_data_record(buf, 1);
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
            send_data_record(buf, 1);
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
            recv_data_record(buf.as_ref());
        }
        res
    }

    fn sendrecv<F>(
        next_f: F,
        rmpi_ctx: &RmpiContext,
        sendbuf: TypeDynamicBufferRef,
        dest: Process,
        sendtag: Tag,
        mut recvbuf: TypeDynamicBufferMut,
        src: Process,
        recvtag: Tag,
        status_ignore: bool,
    ) -> RmpiResult<Option<Status>>
    where
        F: FnOnce(
            &RmpiContext,
            TypeDynamicBufferRef,
            Process,
            Tag,
            TypeDynamicBufferMut,
            Process,
            Tag,
            bool,
        ) -> RmpiResult<Option<Status>>,
    {
        let res = next_f(
            rmpi_ctx,
            sendbuf,
            dest,
            sendtag,
            recvbuf.as_mut(),
            src,
            recvtag,
            status_ignore,
        );
        if res.is_ok() {
            send_data_record(sendbuf, 1);
            recv_data_record(recvbuf.as_ref());
        }
        res
    }

    fn bcast<F>(
        next_f: F,
        rmpi_ctx: &RmpiContext,
        mut buf: TypeDynamicBufferMut,
        root: Process,
    ) -> RmpiResult
    where
        F: FnOnce(&RmpiContext, TypeDynamicBufferMut, Process) -> RmpiResult,
    {
        let res = next_f(rmpi_ctx, buf.as_mut(), root);
        if res.is_ok() {
            let comm_world = rmpi_ctx.comm_world();
            if comm_world.current_process()?.rank() == root.rank() {
                send_data_record(buf.as_ref(), comm_world.size()? as usize - 1);
            } else {
                recv_data_record(buf.as_ref());
            }
        }
        res
    }

    fn allreduce<F>(
        next_f: F,
        rmpi_ctx: &RmpiContext,
        sendbuf: TypeDynamicBufferRef,
        mut recvbuf: TypeDynamicBufferMut,
        op: MpiOp,
        comm: &Communicator,
    ) -> RmpiResult
    where
        F: FnOnce(
            &RmpiContext,
            TypeDynamicBufferRef,
            TypeDynamicBufferMut,
            MpiOp,
            &Communicator,
        ) -> RmpiResult,
    {
        let res = next_f(rmpi_ctx, sendbuf, recvbuf.as_mut(), op, comm);
        if res.is_ok() {
            reduce_data_record(recvbuf.as_ref());
        }
        res
    }
}
