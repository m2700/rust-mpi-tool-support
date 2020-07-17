use std::os::raw::c_int;

use rmpi::{
    request::{Request, RequestSlice},
    Buffer, MpiDatatype, MpiOp, Process, RmpiResult, Status, Tag,
};

pub trait MpiInterceptionLayer {
    trait_layer_function! {
        #[inline]
        fn finalize() -> RmpiResult;

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
        fn recv<Buf: ?Sized>( buf: &mut Buf, src: Process, tag: Tag) -> RmpiResult<Status>
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
        fn free(request: Request) -> RmpiResult;

        // should I really use Buffers of different datatypes??
        #[inline]
        fn gather<SendBuf: ?Sized, RecvBuf: ?Sized>(
            sendbuf: &SendBuf,
            recvbuf: &mut RecvBuf,
            root: Process,
        ) -> RmpiResult where SendBuf: Buffer, RecvBuf: Buffer;
        #[inline]
        fn gatherv<SendBuf: ?Sized, RecvBuf: ?Sized>(
            sendbuf: &SendBuf,
            recvbuf: &mut RecvBuf,
            displs: &[c_int],
            root: Process,
        ) -> RmpiResult where SendBuf: Buffer, RecvBuf: Buffer;
        #[inline]
        fn scatter<SendBuf: ?Sized, RecvBuf: ?Sized>(
            sendbuf: &SendBuf,
            recvbuf: &mut RecvBuf,
            root: Process,
        ) -> RmpiResult where SendBuf: Buffer, RecvBuf: Buffer;
        #[inline]
        fn scatterv<SendBuf: ?Sized, RecvBuf: ?Sized>(
            sendbuf: &SendBuf,
            displs: &[c_int],
            recvbuf: &mut RecvBuf,
            root: Process,
        ) -> RmpiResult where SendBuf: Buffer, RecvBuf: Buffer;

        // even in MPI there is only one Datatype
        #[inline]
        fn reduce<Buf: ?Sized>(
            sendbuf: &Buf,
            recvbuf: &mut Buf,
            op: MpiOp,
            root: Process,
        ) -> c_int where Buf: Buffer;
    }
}
