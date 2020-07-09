use std::os::raw::c_int;

use rmpi::{Buffer, MpiDatatype, Process, Request, RmpiResult, Status, Tag};

pub trait MpiInterceptionLayer {
    trait_layer_function! {
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
        fn recv<Buf: ?Sized>( buf: &mut Buf, src: Process, tag: Tag) -> RmpiResult<Status>
        where
            Buf: Buffer;

        #[inline]
        fn get_count<Datatype>(status: &Status) -> RmpiResult<c_int>
        where
            Datatype: MpiDatatype;

        #[inline]
        fn buffer_attach(buffer: &'static mut [u8]) -> RmpiResult;
        #[inline]
        fn buffer_detach() -> RmpiResult<&'static mut [u8]>;

        #[inline]
        fn wait(request: &mut Request) -> RmpiResult<Status>;
        #[inline]
        fn test(request: &mut Request) -> RmpiResult<Option<Status>>;
    }
}
