use std::os::raw::{c_char, c_double, c_int, c_void};

#[allow(unused_variables)]
pub trait QmpiLayer {
    #[inline]
    fn pre_abort(&mut self, comm: mpi_sys::MPI_Comm, errorcode: c_int) {}
    #[inline]
    fn post_abort(&mut self, output: c_int, comm: mpi_sys::MPI_Comm, errorcode: c_int) {}
    #[inline]
    fn pre_accumulate(
        &mut self,
        origin_addr: *const c_void,
        origin_count: c_int,
        origin_datatype: mpi_sys::MPI_Datatype,
        target_rank: c_int,
        target_disp: mpi_sys::MPI_Aint,
        target_count: c_int,
        target_datatype: mpi_sys::MPI_Datatype,
        op: mpi_sys::MPI_Op,
        win: mpi_sys::MPI_Win,
    ) {
    }
    #[inline]
    fn post_accumulate(
        &mut self,
        output: c_int,
        origin_addr: *const c_void,
        origin_count: c_int,
        origin_datatype: mpi_sys::MPI_Datatype,
        target_rank: c_int,
        target_disp: mpi_sys::MPI_Aint,
        target_count: c_int,
        target_datatype: mpi_sys::MPI_Datatype,
        op: mpi_sys::MPI_Op,
        win: mpi_sys::MPI_Win,
    ) {
    }
    #[inline]
    fn pre_add_error_class(&mut self, errorclass: *mut c_int) {}
    #[inline]
    fn post_add_error_class(&mut self, output: c_int, errorclass: *mut c_int) {}
    #[inline]
    fn pre_add_error_code(&mut self, errorclass: c_int, errorcode: *mut c_int) {}
    #[inline]
    fn post_add_error_code(&mut self, output: c_int, errorclass: c_int, errorcode: *mut c_int) {}
    #[inline]
    fn pre_add_error_string(&mut self, errorcode: c_int, string: *const c_char) {}
    #[inline]
    fn post_add_error_string(&mut self, output: c_int, errorcode: c_int, string: *const c_char) {}
    #[inline]
    fn pre_address(&mut self, location: *mut c_void, address: *mut mpi_sys::MPI_Aint) {}
    #[inline]
    fn post_address(
        &mut self,
        output: c_int,
        location: *mut c_void,
        address: *mut mpi_sys::MPI_Aint,
    ) {
    }
    #[inline]
    fn pre_allgather(
        &mut self,
        sendbuf: *const c_void,
        sendcount: c_int,
        sendtype: mpi_sys::MPI_Datatype,
        recvbuf: *mut c_void,
        recvcount: c_int,
        recvtype: mpi_sys::MPI_Datatype,
        comm: mpi_sys::MPI_Comm,
    ) {
    }
    #[inline]
    fn post_allgather(
        &mut self,
        output: c_int,
        sendbuf: *const c_void,
        sendcount: c_int,
        sendtype: mpi_sys::MPI_Datatype,
        recvbuf: *mut c_void,
        recvcount: c_int,
        recvtype: mpi_sys::MPI_Datatype,
        comm: mpi_sys::MPI_Comm,
    ) {
    }
    #[inline]
    fn pre_allgatherv(
        &mut self,
        sendbuf: *const c_void,
        sendcount: c_int,
        sendtype: mpi_sys::MPI_Datatype,
        recvbuf: *mut c_void,
        recvcounts: *const c_int,
        displs: *const c_int,
        recvtype: mpi_sys::MPI_Datatype,
        comm: mpi_sys::MPI_Comm,
    ) {
    }
    #[inline]
    fn post_allgatherv(
        &mut self,
        output: c_int,
        sendbuf: *const c_void,
        sendcount: c_int,
        sendtype: mpi_sys::MPI_Datatype,
        recvbuf: *mut c_void,
        recvcounts: *const c_int,
        displs: *const c_int,
        recvtype: mpi_sys::MPI_Datatype,
        comm: mpi_sys::MPI_Comm,
    ) {
    }
    #[inline]
    fn pre_alloc_mem(
        &mut self,
        size: mpi_sys::MPI_Aint,
        info: mpi_sys::MPI_Info,
        baseptr: *mut c_void,
    ) {
    }
    #[inline]
    fn post_alloc_mem(
        &mut self,
        output: c_int,
        size: mpi_sys::MPI_Aint,
        info: mpi_sys::MPI_Info,
        baseptr: *mut c_void,
    ) {
    }
    #[inline]
    fn pre_allreduce(
        &mut self,
        sendbuf: *const c_void,
        recvbuf: *mut c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        op: mpi_sys::MPI_Op,
        comm: mpi_sys::MPI_Comm,
    ) {
    }
    #[inline]
    fn post_allreduce(
        &mut self,
        output: c_int,
        sendbuf: *const c_void,
        recvbuf: *mut c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        op: mpi_sys::MPI_Op,
        comm: mpi_sys::MPI_Comm,
    ) {
    }
    #[inline]
    fn pre_alltoall(
        &mut self,
        sendbuf: *const c_void,
        sendcount: c_int,
        sendtype: mpi_sys::MPI_Datatype,
        recvbuf: *mut c_void,
        recvcount: c_int,
        recvtype: mpi_sys::MPI_Datatype,
        comm: mpi_sys::MPI_Comm,
    ) {
    }
    #[inline]
    fn post_alltoall(
        &mut self,
        output: c_int,
        sendbuf: *const c_void,
        sendcount: c_int,
        sendtype: mpi_sys::MPI_Datatype,
        recvbuf: *mut c_void,
        recvcount: c_int,
        recvtype: mpi_sys::MPI_Datatype,
        comm: mpi_sys::MPI_Comm,
    ) {
    }
    #[inline]
    fn pre_alltoallv(
        &mut self,
        sendbuf: *const c_void,
        sendcounts: *const c_int,
        sdispls: *const c_int,
        sendtype: mpi_sys::MPI_Datatype,
        recvbuf: *mut c_void,
        recvcounts: *const c_int,
        rdispls: *const c_int,
        recvtype: mpi_sys::MPI_Datatype,
        comm: mpi_sys::MPI_Comm,
    ) {
    }
    #[inline]
    fn post_alltoallv(
        &mut self,
        output: c_int,
        sendbuf: *const c_void,
        sendcounts: *const c_int,
        sdispls: *const c_int,
        sendtype: mpi_sys::MPI_Datatype,
        recvbuf: *mut c_void,
        recvcounts: *const c_int,
        rdispls: *const c_int,
        recvtype: mpi_sys::MPI_Datatype,
        comm: mpi_sys::MPI_Comm,
    ) {
    }
    #[inline]
    fn pre_alltoallw(
        &mut self,
        sendbuf: *const c_void,
        sendcounts: *const c_int,
        sdispls: *const c_int,
        sendtypes: *const mpi_sys::MPI_Datatype,
        recvbuf: *mut c_void,
        recvcounts: *const c_int,
        rdispls: *const c_int,
        recvtypes: *const mpi_sys::MPI_Datatype,
        comm: mpi_sys::MPI_Comm,
    ) {
    }
    #[inline]
    fn post_alltoallw(
        &mut self,
        output: c_int,
        sendbuf: *const c_void,
        sendcounts: *const c_int,
        sdispls: *const c_int,
        sendtypes: *const mpi_sys::MPI_Datatype,
        recvbuf: *mut c_void,
        recvcounts: *const c_int,
        rdispls: *const c_int,
        recvtypes: *const mpi_sys::MPI_Datatype,
        comm: mpi_sys::MPI_Comm,
    ) {
    }
    #[inline]
    fn pre_attr_delete(&mut self, comm: mpi_sys::MPI_Comm, keyval: c_int) {}
    #[inline]
    fn post_attr_delete(&mut self, output: c_int, comm: mpi_sys::MPI_Comm, keyval: c_int) {}
    #[inline]
    fn pre_attr_get(
        &mut self,
        comm: mpi_sys::MPI_Comm,
        keyval: c_int,
        attribute_val: *mut c_void,
        flag: *mut c_int,
    ) {
    }
    #[inline]
    fn post_attr_get(
        &mut self,
        output: c_int,
        comm: mpi_sys::MPI_Comm,
        keyval: c_int,
        attribute_val: *mut c_void,
        flag: *mut c_int,
    ) {
    }
    #[inline]
    fn pre_attr_put(&mut self, comm: mpi_sys::MPI_Comm, keyval: c_int, attribute_val: *mut c_void) {
    }
    #[inline]
    fn post_attr_put(
        &mut self,
        output: c_int,
        comm: mpi_sys::MPI_Comm,
        keyval: c_int,
        attribute_val: *mut c_void,
    ) {
    }
    #[inline]
    fn pre_barrier(&mut self, comm: mpi_sys::MPI_Comm) {}
    #[inline]
    fn post_barrier(&mut self, output: c_int, comm: mpi_sys::MPI_Comm) {}
    #[inline]
    fn pre_bcast(
        &mut self,
        buffer: *mut c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        root: c_int,
        comm: mpi_sys::MPI_Comm,
    ) {
    }
    #[inline]
    fn post_bcast(
        &mut self,
        output: c_int,
        buffer: *mut c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        root: c_int,
        comm: mpi_sys::MPI_Comm,
    ) {
    }
    #[inline]
    fn pre_bsend(
        &mut self,
        buf: *const c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        dest: c_int,
        tag: c_int,
        comm: mpi_sys::MPI_Comm,
    ) {
    }
    #[inline]
    fn post_bsend(
        &mut self,
        output: c_int,
        buf: *const c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        dest: c_int,
        tag: c_int,
        comm: mpi_sys::MPI_Comm,
    ) {
    }
    #[inline]
    fn pre_bsend_init(
        &mut self,
        buf: *const c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        dest: c_int,
        tag: c_int,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn post_bsend_init(
        &mut self,
        output: c_int,
        buf: *const c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        dest: c_int,
        tag: c_int,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn pre_buffer_attach(&mut self, buffer: *mut c_void, size: c_int) {}
    #[inline]
    fn post_buffer_attach(&mut self, output: c_int, buffer: *mut c_void, size: c_int) {}
    #[inline]
    fn pre_buffer_detach(&mut self, buffer: *mut c_void, size: *mut c_int) {}
    #[inline]
    fn post_buffer_detach(&mut self, output: c_int, buffer: *mut c_void, size: *mut c_int) {}
    #[inline]
    fn pre_cancel(&mut self, request: *mut mpi_sys::MPI_Request) {}
    #[inline]
    fn post_cancel(&mut self, output: c_int, request: *mut mpi_sys::MPI_Request) {}
    #[inline]
    fn pre_cart_coords(
        &mut self,
        comm: mpi_sys::MPI_Comm,
        rank: c_int,
        maxdims: c_int,
        coords: *mut c_int,
    ) {
    }
    #[inline]
    fn post_cart_coords(
        &mut self,
        output: c_int,
        comm: mpi_sys::MPI_Comm,
        rank: c_int,
        maxdims: c_int,
        coords: *mut c_int,
    ) {
    }
    #[inline]
    fn pre_cart_create(
        &mut self,
        old_comm: mpi_sys::MPI_Comm,
        ndims: c_int,
        dims: *const c_int,
        periods: *const c_int,
        reorder: c_int,
        comm_cart: *mut mpi_sys::MPI_Comm,
    ) {
    }
    #[inline]
    fn post_cart_create(
        &mut self,
        output: c_int,
        old_comm: mpi_sys::MPI_Comm,
        ndims: c_int,
        dims: *const c_int,
        periods: *const c_int,
        reorder: c_int,
        comm_cart: *mut mpi_sys::MPI_Comm,
    ) {
    }
    #[inline]
    fn pre_cart_get(
        &mut self,
        comm: mpi_sys::MPI_Comm,
        maxdims: c_int,
        dims: *mut c_int,
        periods: *mut c_int,
        coords: *mut c_int,
    ) {
    }
    #[inline]
    fn post_cart_get(
        &mut self,
        output: c_int,
        comm: mpi_sys::MPI_Comm,
        maxdims: c_int,
        dims: *mut c_int,
        periods: *mut c_int,
        coords: *mut c_int,
    ) {
    }
    #[inline]
    fn pre_cart_map(
        &mut self,
        comm: mpi_sys::MPI_Comm,
        ndims: c_int,
        dims: *const c_int,
        periods: *const c_int,
        newrank: *mut c_int,
    ) {
    }
    #[inline]
    fn post_cart_map(
        &mut self,
        output: c_int,
        comm: mpi_sys::MPI_Comm,
        ndims: c_int,
        dims: *const c_int,
        periods: *const c_int,
        newrank: *mut c_int,
    ) {
    }
    #[inline]
    fn pre_cart_rank(&mut self, comm: mpi_sys::MPI_Comm, coords: *const c_int, rank: *mut c_int) {}
    #[inline]
    fn post_cart_rank(
        &mut self,
        output: c_int,
        comm: mpi_sys::MPI_Comm,
        coords: *const c_int,
        rank: *mut c_int,
    ) {
    }
    #[inline]
    fn pre_cart_shift(
        &mut self,
        comm: mpi_sys::MPI_Comm,
        direction: c_int,
        disp: c_int,
        rank_source: *mut c_int,
        rank_dest: *mut c_int,
    ) {
    }
    #[inline]
    fn post_cart_shift(
        &mut self,
        output: c_int,
        comm: mpi_sys::MPI_Comm,
        direction: c_int,
        disp: c_int,
        rank_source: *mut c_int,
        rank_dest: *mut c_int,
    ) {
    }
    #[inline]
    fn pre_cart_sub(
        &mut self,
        comm: mpi_sys::MPI_Comm,
        remain_dims: *const c_int,
        new_comm: *mut mpi_sys::MPI_Comm,
    ) {
    }
    #[inline]
    fn post_cart_sub(
        &mut self,
        output: c_int,
        comm: mpi_sys::MPI_Comm,
        remain_dims: *const c_int,
        new_comm: *mut mpi_sys::MPI_Comm,
    ) {
    }
    #[inline]
    fn pre_cartdim_get(&mut self, comm: mpi_sys::MPI_Comm, ndims: *mut c_int) {}
    #[inline]
    fn post_cartdim_get(&mut self, output: c_int, comm: mpi_sys::MPI_Comm, ndims: *mut c_int) {}
    #[inline]
    fn pre_close_port(&mut self, port_name: *const c_char) {}
    #[inline]
    fn post_close_port(&mut self, output: c_int, port_name: *const c_char) {}
    #[inline]
    fn pre_comm_accept(
        &mut self,
        port_name: *const c_char,
        info: mpi_sys::MPI_Info,
        root: c_int,
        comm: mpi_sys::MPI_Comm,
        newcomm: *mut mpi_sys::MPI_Comm,
    ) {
    }
    #[inline]
    fn post_comm_accept(
        &mut self,
        output: c_int,
        port_name: *const c_char,
        info: mpi_sys::MPI_Info,
        root: c_int,
        comm: mpi_sys::MPI_Comm,
        newcomm: *mut mpi_sys::MPI_Comm,
    ) {
    }
    #[inline]
    fn pre_comm_call_errhandler(&mut self, comm: mpi_sys::MPI_Comm, errorcode: c_int) {}
    #[inline]
    fn post_comm_call_errhandler(
        &mut self,
        output: c_int,
        comm: mpi_sys::MPI_Comm,
        errorcode: c_int,
    ) {
    }
    #[inline]
    fn pre_comm_compare(
        &mut self,
        comm1: mpi_sys::MPI_Comm,
        comm2: mpi_sys::MPI_Comm,
        result: *mut c_int,
    ) {
    }
    #[inline]
    fn post_comm_compare(
        &mut self,
        output: c_int,
        comm1: mpi_sys::MPI_Comm,
        comm2: mpi_sys::MPI_Comm,
        result: *mut c_int,
    ) {
    }
    #[inline]
    fn pre_comm_connect(
        &mut self,
        port_name: *const c_char,
        info: mpi_sys::MPI_Info,
        root: c_int,
        comm: mpi_sys::MPI_Comm,
        newcomm: *mut mpi_sys::MPI_Comm,
    ) {
    }
    #[inline]
    fn post_comm_connect(
        &mut self,
        output: c_int,
        port_name: *const c_char,
        info: mpi_sys::MPI_Info,
        root: c_int,
        comm: mpi_sys::MPI_Comm,
        newcomm: *mut mpi_sys::MPI_Comm,
    ) {
    }
    #[inline]
    fn pre_comm_create(
        &mut self,
        comm: mpi_sys::MPI_Comm,
        group: mpi_sys::MPI_Group,
        newcomm: *mut mpi_sys::MPI_Comm,
    ) {
    }
    #[inline]
    fn post_comm_create(
        &mut self,
        output: c_int,
        comm: mpi_sys::MPI_Comm,
        group: mpi_sys::MPI_Group,
        newcomm: *mut mpi_sys::MPI_Comm,
    ) {
    }
    #[inline]
    fn pre_comm_create_errhandler(
        &mut self,
        function: *mut mpi_sys::MPI_Comm_errhandler_function,
        errhandler: *mut mpi_sys::MPI_Errhandler,
    ) {
    }
    #[inline]
    fn post_comm_create_errhandler(
        &mut self,
        output: c_int,
        function: *mut mpi_sys::MPI_Comm_errhandler_function,
        errhandler: *mut mpi_sys::MPI_Errhandler,
    ) {
    }
    #[inline]
    fn pre_comm_create_group(
        &mut self,
        comm: mpi_sys::MPI_Comm,
        group: mpi_sys::MPI_Group,
        tag: c_int,
        newcomm: *mut mpi_sys::MPI_Comm,
    ) {
    }
    #[inline]
    fn post_comm_create_group(
        &mut self,
        output: c_int,
        comm: mpi_sys::MPI_Comm,
        group: mpi_sys::MPI_Group,
        tag: c_int,
        newcomm: *mut mpi_sys::MPI_Comm,
    ) {
    }
    #[inline]
    fn pre_comm_create_keyval(
        &mut self,
        comm_copy_attr_fn: *mut mpi_sys::MPI_Comm_copy_attr_function,
        comm_delete_attr_fn: *mut mpi_sys::MPI_Comm_delete_attr_function,
        comm_keyval: *mut c_int,
        extra_state: *mut c_void,
    ) {
    }
    #[inline]
    fn post_comm_create_keyval(
        &mut self,
        output: c_int,
        comm_copy_attr_fn: *mut mpi_sys::MPI_Comm_copy_attr_function,
        comm_delete_attr_fn: *mut mpi_sys::MPI_Comm_delete_attr_function,
        comm_keyval: *mut c_int,
        extra_state: *mut c_void,
    ) {
    }
    #[inline]
    fn pre_comm_delete_attr(&mut self, comm: mpi_sys::MPI_Comm, comm_keyval: c_int) {}
    #[inline]
    fn post_comm_delete_attr(
        &mut self,
        output: c_int,
        comm: mpi_sys::MPI_Comm,
        comm_keyval: c_int,
    ) {
    }
    #[inline]
    fn pre_comm_disconnect(&mut self, comm: *mut mpi_sys::MPI_Comm) {}
    #[inline]
    fn post_comm_disconnect(&mut self, output: c_int, comm: *mut mpi_sys::MPI_Comm) {}
    #[inline]
    fn pre_comm_dup(&mut self, comm: mpi_sys::MPI_Comm, newcomm: *mut mpi_sys::MPI_Comm) {}
    #[inline]
    fn post_comm_dup(
        &mut self,
        output: c_int,
        comm: mpi_sys::MPI_Comm,
        newcomm: *mut mpi_sys::MPI_Comm,
    ) {
    }
    #[inline]
    fn pre_comm_dup_with_info(
        &mut self,
        comm: mpi_sys::MPI_Comm,
        info: mpi_sys::MPI_Info,
        newcomm: *mut mpi_sys::MPI_Comm,
    ) {
    }
    #[inline]
    fn post_comm_dup_with_info(
        &mut self,
        output: c_int,
        comm: mpi_sys::MPI_Comm,
        info: mpi_sys::MPI_Info,
        newcomm: *mut mpi_sys::MPI_Comm,
    ) {
    }
    #[inline]
    fn pre_comm_free(&mut self, comm: *mut mpi_sys::MPI_Comm) {}
    #[inline]
    fn post_comm_free(&mut self, output: c_int, comm: *mut mpi_sys::MPI_Comm) {}
    #[inline]
    fn pre_comm_free_keyval(&mut self, comm_keyval: *mut c_int) {}
    #[inline]
    fn post_comm_free_keyval(&mut self, output: c_int, comm_keyval: *mut c_int) {}
    #[inline]
    fn pre_comm_get_attr(
        &mut self,
        comm: mpi_sys::MPI_Comm,
        comm_keyval: c_int,
        attribute_val: *mut c_void,
        flag: *mut c_int,
    ) {
    }
    #[inline]
    fn post_comm_get_attr(
        &mut self,
        output: c_int,
        comm: mpi_sys::MPI_Comm,
        comm_keyval: c_int,
        attribute_val: *mut c_void,
        flag: *mut c_int,
    ) {
    }
    #[inline]
    fn pre_comm_get_errhandler(
        &mut self,
        comm: mpi_sys::MPI_Comm,
        erhandler: *mut mpi_sys::MPI_Errhandler,
    ) {
    }
    #[inline]
    fn post_comm_get_errhandler(
        &mut self,
        output: c_int,
        comm: mpi_sys::MPI_Comm,
        erhandler: *mut mpi_sys::MPI_Errhandler,
    ) {
    }
    #[inline]
    fn pre_comm_get_info(&mut self, comm: mpi_sys::MPI_Comm, info_used: *mut mpi_sys::MPI_Info) {}
    #[inline]
    fn post_comm_get_info(
        &mut self,
        output: c_int,
        comm: mpi_sys::MPI_Comm,
        info_used: *mut mpi_sys::MPI_Info,
    ) {
    }
    #[inline]
    fn pre_comm_get_name(
        &mut self,
        comm: mpi_sys::MPI_Comm,
        comm_name: *mut c_char,
        resultlen: *mut c_int,
    ) {
    }
    #[inline]
    fn post_comm_get_name(
        &mut self,
        output: c_int,
        comm: mpi_sys::MPI_Comm,
        comm_name: *mut c_char,
        resultlen: *mut c_int,
    ) {
    }
    #[inline]
    fn pre_comm_get_parent(&mut self, parent: *mut mpi_sys::MPI_Comm) {}
    #[inline]
    fn post_comm_get_parent(&mut self, output: c_int, parent: *mut mpi_sys::MPI_Comm) {}
    #[inline]
    fn pre_comm_group(&mut self, comm: mpi_sys::MPI_Comm, group: *mut mpi_sys::MPI_Group) {}
    #[inline]
    fn post_comm_group(
        &mut self,
        output: c_int,
        comm: mpi_sys::MPI_Comm,
        group: *mut mpi_sys::MPI_Group,
    ) {
    }
    #[inline]
    fn pre_comm_idup(
        &mut self,
        comm: mpi_sys::MPI_Comm,
        newcomm: *mut mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn post_comm_idup(
        &mut self,
        output: c_int,
        comm: mpi_sys::MPI_Comm,
        newcomm: *mut mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn pre_comm_join(&mut self, fd: c_int, intercomm: *mut mpi_sys::MPI_Comm) {}
    #[inline]
    fn post_comm_join(&mut self, output: c_int, fd: c_int, intercomm: *mut mpi_sys::MPI_Comm) {}
    #[inline]
    fn pre_comm_rank(&mut self, comm: mpi_sys::MPI_Comm, rank: *mut c_int) {}
    #[inline]
    fn post_comm_rank(&mut self, output: c_int, comm: mpi_sys::MPI_Comm, rank: *mut c_int) {}
    #[inline]
    fn pre_comm_remote_group(&mut self, comm: mpi_sys::MPI_Comm, group: *mut mpi_sys::MPI_Group) {}
    #[inline]
    fn post_comm_remote_group(
        &mut self,
        output: c_int,
        comm: mpi_sys::MPI_Comm,
        group: *mut mpi_sys::MPI_Group,
    ) {
    }
    #[inline]
    fn pre_comm_remote_size(&mut self, comm: mpi_sys::MPI_Comm, size: *mut c_int) {}
    #[inline]
    fn post_comm_remote_size(&mut self, output: c_int, comm: mpi_sys::MPI_Comm, size: *mut c_int) {}
    #[inline]
    fn pre_comm_set_attr(
        &mut self,
        comm: mpi_sys::MPI_Comm,
        comm_keyval: c_int,
        attribute_val: *mut c_void,
    ) {
    }
    #[inline]
    fn post_comm_set_attr(
        &mut self,
        output: c_int,
        comm: mpi_sys::MPI_Comm,
        comm_keyval: c_int,
        attribute_val: *mut c_void,
    ) {
    }
    #[inline]
    fn pre_comm_set_errhandler(
        &mut self,
        comm: mpi_sys::MPI_Comm,
        errhandler: mpi_sys::MPI_Errhandler,
    ) {
    }
    #[inline]
    fn post_comm_set_errhandler(
        &mut self,
        output: c_int,
        comm: mpi_sys::MPI_Comm,
        errhandler: mpi_sys::MPI_Errhandler,
    ) {
    }
    #[inline]
    fn pre_comm_set_info(&mut self, comm: mpi_sys::MPI_Comm, info: mpi_sys::MPI_Info) {}
    #[inline]
    fn post_comm_set_info(
        &mut self,
        output: c_int,
        comm: mpi_sys::MPI_Comm,
        info: mpi_sys::MPI_Info,
    ) {
    }
    #[inline]
    fn pre_comm_set_name(&mut self, comm: mpi_sys::MPI_Comm, comm_name: *const c_char) {}
    #[inline]
    fn post_comm_set_name(
        &mut self,
        output: c_int,
        comm: mpi_sys::MPI_Comm,
        comm_name: *const c_char,
    ) {
    }
    #[inline]
    fn pre_comm_size(&mut self, comm: mpi_sys::MPI_Comm, size: *mut c_int) {}
    #[inline]
    fn post_comm_size(&mut self, output: c_int, comm: mpi_sys::MPI_Comm, size: *mut c_int) {}
    #[inline]
    fn pre_comm_split(
        &mut self,
        comm: mpi_sys::MPI_Comm,
        color: c_int,
        key: c_int,
        newcomm: *mut mpi_sys::MPI_Comm,
    ) {
    }
    #[inline]
    fn post_comm_split(
        &mut self,
        output: c_int,
        comm: mpi_sys::MPI_Comm,
        color: c_int,
        key: c_int,
        newcomm: *mut mpi_sys::MPI_Comm,
    ) {
    }
    #[inline]
    fn pre_comm_split_type(
        &mut self,
        comm: mpi_sys::MPI_Comm,
        split_type: c_int,
        key: c_int,
        info: mpi_sys::MPI_Info,
        newcomm: *mut mpi_sys::MPI_Comm,
    ) {
    }
    #[inline]
    fn post_comm_split_type(
        &mut self,
        output: c_int,
        comm: mpi_sys::MPI_Comm,
        split_type: c_int,
        key: c_int,
        info: mpi_sys::MPI_Info,
        newcomm: *mut mpi_sys::MPI_Comm,
    ) {
    }
    #[inline]
    fn pre_comm_test_inter(&mut self, comm: mpi_sys::MPI_Comm, flag: *mut c_int) {}
    #[inline]
    fn post_comm_test_inter(&mut self, output: c_int, comm: mpi_sys::MPI_Comm, flag: *mut c_int) {}
    #[inline]
    fn pre_compare_and_swap(
        &mut self,
        origin_addr: *const c_void,
        compare_addr: *const c_void,
        result_addr: *mut c_void,
        datatype: mpi_sys::MPI_Datatype,
        target_rank: c_int,
        target_disp: mpi_sys::MPI_Aint,
        win: mpi_sys::MPI_Win,
    ) {
    }
    #[inline]
    fn post_compare_and_swap(
        &mut self,
        output: c_int,
        origin_addr: *const c_void,
        compare_addr: *const c_void,
        result_addr: *mut c_void,
        datatype: mpi_sys::MPI_Datatype,
        target_rank: c_int,
        target_disp: mpi_sys::MPI_Aint,
        win: mpi_sys::MPI_Win,
    ) {
    }
    #[inline]
    fn pre_dims_create(&mut self, nnodes: c_int, ndims: c_int, dims: *mut c_int) {}
    #[inline]
    fn post_dims_create(&mut self, output: c_int, nnodes: c_int, ndims: c_int, dims: *mut c_int) {}
    #[inline]
    fn pre_dist_graph_create(
        &mut self,
        comm_old: mpi_sys::MPI_Comm,
        n: c_int,
        nodes: *const c_int,
        degrees: *const c_int,
        targets: *const c_int,
        weights: *const c_int,
        info: mpi_sys::MPI_Info,
        reorder: c_int,
        newcomm: *mut mpi_sys::MPI_Comm,
    ) {
    }
    #[inline]
    fn post_dist_graph_create(
        &mut self,
        output: c_int,
        comm_old: mpi_sys::MPI_Comm,
        n: c_int,
        nodes: *const c_int,
        degrees: *const c_int,
        targets: *const c_int,
        weights: *const c_int,
        info: mpi_sys::MPI_Info,
        reorder: c_int,
        newcomm: *mut mpi_sys::MPI_Comm,
    ) {
    }
    #[inline]
    fn pre_dist_graph_create_adjacent(
        &mut self,
        comm_old: mpi_sys::MPI_Comm,
        indegree: c_int,
        sources: *const c_int,
        sourceweights: *const c_int,
        outdegree: c_int,
        destinations: *const c_int,
        destweights: *const c_int,
        info: mpi_sys::MPI_Info,
        reorder: c_int,
        comm_dist_graph: *mut mpi_sys::MPI_Comm,
    ) {
    }
    #[inline]
    fn post_dist_graph_create_adjacent(
        &mut self,
        output: c_int,
        comm_old: mpi_sys::MPI_Comm,
        indegree: c_int,
        sources: *const c_int,
        sourceweights: *const c_int,
        outdegree: c_int,
        destinations: *const c_int,
        destweights: *const c_int,
        info: mpi_sys::MPI_Info,
        reorder: c_int,
        comm_dist_graph: *mut mpi_sys::MPI_Comm,
    ) {
    }
    #[inline]
    fn pre_dist_graph_neighbors(
        &mut self,
        comm: mpi_sys::MPI_Comm,
        maxindegree: c_int,
        sources: *mut c_int,
        sourceweights: *mut c_int,
        maxoutdegree: c_int,
        destinations: *mut c_int,
        destweights: *mut c_int,
    ) {
    }
    #[inline]
    fn post_dist_graph_neighbors(
        &mut self,
        output: c_int,
        comm: mpi_sys::MPI_Comm,
        maxindegree: c_int,
        sources: *mut c_int,
        sourceweights: *mut c_int,
        maxoutdegree: c_int,
        destinations: *mut c_int,
        destweights: *mut c_int,
    ) {
    }
    #[inline]
    fn pre_dist_graph_neighbors_count(
        &mut self,
        comm: mpi_sys::MPI_Comm,
        inneighbors: *mut c_int,
        outneighbors: *mut c_int,
        weighted: *mut c_int,
    ) {
    }
    #[inline]
    fn post_dist_graph_neighbors_count(
        &mut self,
        output: c_int,
        comm: mpi_sys::MPI_Comm,
        inneighbors: *mut c_int,
        outneighbors: *mut c_int,
        weighted: *mut c_int,
    ) {
    }
    #[inline]
    fn pre_errhandler_create(
        &mut self,
        function: *mut mpi_sys::MPI_Handler_function,
        errhandler: *mut mpi_sys::MPI_Errhandler,
    ) {
    }
    #[inline]
    fn post_errhandler_create(
        &mut self,
        output: c_int,
        function: *mut mpi_sys::MPI_Handler_function,
        errhandler: *mut mpi_sys::MPI_Errhandler,
    ) {
    }
    #[inline]
    fn pre_errhandler_free(&mut self, errhandler: *mut mpi_sys::MPI_Errhandler) {}
    #[inline]
    fn post_errhandler_free(&mut self, output: c_int, errhandler: *mut mpi_sys::MPI_Errhandler) {}
    #[inline]
    fn pre_errhandler_get(
        &mut self,
        comm: mpi_sys::MPI_Comm,
        errhandler: *mut mpi_sys::MPI_Errhandler,
    ) {
    }
    #[inline]
    fn post_errhandler_get(
        &mut self,
        output: c_int,
        comm: mpi_sys::MPI_Comm,
        errhandler: *mut mpi_sys::MPI_Errhandler,
    ) {
    }
    #[inline]
    fn pre_errhandler_set(&mut self, comm: mpi_sys::MPI_Comm, errhandler: mpi_sys::MPI_Errhandler) {
    }
    #[inline]
    fn post_errhandler_set(
        &mut self,
        output: c_int,
        comm: mpi_sys::MPI_Comm,
        errhandler: mpi_sys::MPI_Errhandler,
    ) {
    }
    #[inline]
    fn pre_error_class(&mut self, errorcode: c_int, errorclass: *mut c_int) {}
    #[inline]
    fn post_error_class(&mut self, output: c_int, errorcode: c_int, errorclass: *mut c_int) {}
    #[inline]
    fn pre_error_string(&mut self, errorcode: c_int, string: *mut c_char, resultlen: *mut c_int) {}
    #[inline]
    fn post_error_string(
        &mut self,
        output: c_int,
        errorcode: c_int,
        string: *mut c_char,
        resultlen: *mut c_int,
    ) {
    }
    #[inline]
    fn pre_exscan(
        &mut self,
        sendbuf: *const c_void,
        recvbuf: *mut c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        op: mpi_sys::MPI_Op,
        comm: mpi_sys::MPI_Comm,
    ) {
    }
    #[inline]
    fn post_exscan(
        &mut self,
        output: c_int,
        sendbuf: *const c_void,
        recvbuf: *mut c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        op: mpi_sys::MPI_Op,
        comm: mpi_sys::MPI_Comm,
    ) {
    }
    #[inline]
    fn pre_fetch_and_op(
        &mut self,
        origin_addr: *const c_void,
        result_addr: *mut c_void,
        datatype: mpi_sys::MPI_Datatype,
        target_rank: c_int,
        target_disp: mpi_sys::MPI_Aint,
        op: mpi_sys::MPI_Op,
        win: mpi_sys::MPI_Win,
    ) {
    }
    #[inline]
    fn post_fetch_and_op(
        &mut self,
        output: c_int,
        origin_addr: *const c_void,
        result_addr: *mut c_void,
        datatype: mpi_sys::MPI_Datatype,
        target_rank: c_int,
        target_disp: mpi_sys::MPI_Aint,
        op: mpi_sys::MPI_Op,
        win: mpi_sys::MPI_Win,
    ) {
    }
    #[inline]
    fn pre_file_call_errhandler(&mut self, fh: mpi_sys::MPI_File, errorcode: c_int) {}
    #[inline]
    fn post_file_call_errhandler(
        &mut self,
        output: c_int,
        fh: mpi_sys::MPI_File,
        errorcode: c_int,
    ) {
    }
    #[inline]
    fn pre_file_close(&mut self, fh: *mut mpi_sys::MPI_File) {}
    #[inline]
    fn post_file_close(&mut self, output: c_int, fh: *mut mpi_sys::MPI_File) {}
    #[inline]
    fn pre_file_create_errhandler(
        &mut self,
        function: *mut mpi_sys::MPI_File_errhandler_function,
        errhandler: *mut mpi_sys::MPI_Errhandler,
    ) {
    }
    #[inline]
    fn post_file_create_errhandler(
        &mut self,
        output: c_int,
        function: *mut mpi_sys::MPI_File_errhandler_function,
        errhandler: *mut mpi_sys::MPI_Errhandler,
    ) {
    }
    #[inline]
    fn pre_file_delete(&mut self, filename: *const c_char, info: mpi_sys::MPI_Info) {}
    #[inline]
    fn post_file_delete(
        &mut self,
        output: c_int,
        filename: *const c_char,
        info: mpi_sys::MPI_Info,
    ) {
    }
    #[inline]
    fn pre_file_get_amode(&mut self, fh: mpi_sys::MPI_File, amode: *mut c_int) {}
    #[inline]
    fn post_file_get_amode(&mut self, output: c_int, fh: mpi_sys::MPI_File, amode: *mut c_int) {}
    #[inline]
    fn pre_file_get_atomicity(&mut self, fh: mpi_sys::MPI_File, flag: *mut c_int) {}
    #[inline]
    fn post_file_get_atomicity(&mut self, output: c_int, fh: mpi_sys::MPI_File, flag: *mut c_int) {}
    #[inline]
    fn pre_file_get_byte_offset(
        &mut self,
        fh: mpi_sys::MPI_File,
        offset: mpi_sys::MPI_Offset,
        disp: *mut mpi_sys::MPI_Offset,
    ) {
    }
    #[inline]
    fn post_file_get_byte_offset(
        &mut self,
        output: c_int,
        fh: mpi_sys::MPI_File,
        offset: mpi_sys::MPI_Offset,
        disp: *mut mpi_sys::MPI_Offset,
    ) {
    }
    #[inline]
    fn pre_file_get_errhandler(
        &mut self,
        file: mpi_sys::MPI_File,
        errhandler: *mut mpi_sys::MPI_Errhandler,
    ) {
    }
    #[inline]
    fn post_file_get_errhandler(
        &mut self,
        output: c_int,
        file: mpi_sys::MPI_File,
        errhandler: *mut mpi_sys::MPI_Errhandler,
    ) {
    }
    #[inline]
    fn pre_file_get_group(&mut self, fh: mpi_sys::MPI_File, group: *mut mpi_sys::MPI_Group) {}
    #[inline]
    fn post_file_get_group(
        &mut self,
        output: c_int,
        fh: mpi_sys::MPI_File,
        group: *mut mpi_sys::MPI_Group,
    ) {
    }
    #[inline]
    fn pre_file_get_info(&mut self, fh: mpi_sys::MPI_File, info_used: *mut mpi_sys::MPI_Info) {}
    #[inline]
    fn post_file_get_info(
        &mut self,
        output: c_int,
        fh: mpi_sys::MPI_File,
        info_used: *mut mpi_sys::MPI_Info,
    ) {
    }
    #[inline]
    fn pre_file_get_position(&mut self, fh: mpi_sys::MPI_File, offset: *mut mpi_sys::MPI_Offset) {}
    #[inline]
    fn post_file_get_position(
        &mut self,
        output: c_int,
        fh: mpi_sys::MPI_File,
        offset: *mut mpi_sys::MPI_Offset,
    ) {
    }
    #[inline]
    fn pre_file_get_position_shared(
        &mut self,
        fh: mpi_sys::MPI_File,
        offset: *mut mpi_sys::MPI_Offset,
    ) {
    }
    #[inline]
    fn post_file_get_position_shared(
        &mut self,
        output: c_int,
        fh: mpi_sys::MPI_File,
        offset: *mut mpi_sys::MPI_Offset,
    ) {
    }
    #[inline]
    fn pre_file_get_size(&mut self, fh: mpi_sys::MPI_File, size: *mut mpi_sys::MPI_Offset) {}
    #[inline]
    fn post_file_get_size(
        &mut self,
        output: c_int,
        fh: mpi_sys::MPI_File,
        size: *mut mpi_sys::MPI_Offset,
    ) {
    }
    #[inline]
    fn pre_file_get_type_extent(
        &mut self,
        fh: mpi_sys::MPI_File,
        datatype: mpi_sys::MPI_Datatype,
        extent: *mut mpi_sys::MPI_Aint,
    ) {
    }
    #[inline]
    fn post_file_get_type_extent(
        &mut self,
        output: c_int,
        fh: mpi_sys::MPI_File,
        datatype: mpi_sys::MPI_Datatype,
        extent: *mut mpi_sys::MPI_Aint,
    ) {
    }
    #[inline]
    fn pre_file_get_view(
        &mut self,
        fh: mpi_sys::MPI_File,
        disp: *mut mpi_sys::MPI_Offset,
        etype: *mut mpi_sys::MPI_Datatype,
        filetype: *mut mpi_sys::MPI_Datatype,
        datarep: *mut c_char,
    ) {
    }
    #[inline]
    fn post_file_get_view(
        &mut self,
        output: c_int,
        fh: mpi_sys::MPI_File,
        disp: *mut mpi_sys::MPI_Offset,
        etype: *mut mpi_sys::MPI_Datatype,
        filetype: *mut mpi_sys::MPI_Datatype,
        datarep: *mut c_char,
    ) {
    }
    #[inline]
    fn pre_file_iread(
        &mut self,
        fh: mpi_sys::MPI_File,
        buf: *mut c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        request: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn post_file_iread(
        &mut self,
        output: c_int,
        fh: mpi_sys::MPI_File,
        buf: *mut c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        request: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn pre_file_iread_all(
        &mut self,
        fh: mpi_sys::MPI_File,
        buf: *mut c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        request: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn post_file_iread_all(
        &mut self,
        output: c_int,
        fh: mpi_sys::MPI_File,
        buf: *mut c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        request: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn pre_file_iread_at(
        &mut self,
        fh: mpi_sys::MPI_File,
        offset: mpi_sys::MPI_Offset,
        buf: *mut c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        request: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn post_file_iread_at(
        &mut self,
        output: c_int,
        fh: mpi_sys::MPI_File,
        offset: mpi_sys::MPI_Offset,
        buf: *mut c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        request: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn pre_file_iread_at_all(
        &mut self,
        fh: mpi_sys::MPI_File,
        offset: mpi_sys::MPI_Offset,
        buf: *mut c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        request: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn post_file_iread_at_all(
        &mut self,
        output: c_int,
        fh: mpi_sys::MPI_File,
        offset: mpi_sys::MPI_Offset,
        buf: *mut c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        request: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn pre_file_iread_shared(
        &mut self,
        fh: mpi_sys::MPI_File,
        buf: *mut c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        request: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn post_file_iread_shared(
        &mut self,
        output: c_int,
        fh: mpi_sys::MPI_File,
        buf: *mut c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        request: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn pre_file_iwrite(
        &mut self,
        fh: mpi_sys::MPI_File,
        buf: *const c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        request: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn post_file_iwrite(
        &mut self,
        output: c_int,
        fh: mpi_sys::MPI_File,
        buf: *const c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        request: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn pre_file_iwrite_all(
        &mut self,
        fh: mpi_sys::MPI_File,
        buf: *const c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        request: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn post_file_iwrite_all(
        &mut self,
        output: c_int,
        fh: mpi_sys::MPI_File,
        buf: *const c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        request: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn pre_file_iwrite_at(
        &mut self,
        fh: mpi_sys::MPI_File,
        offset: mpi_sys::MPI_Offset,
        buf: *const c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        request: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn post_file_iwrite_at(
        &mut self,
        output: c_int,
        fh: mpi_sys::MPI_File,
        offset: mpi_sys::MPI_Offset,
        buf: *const c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        request: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn pre_file_iwrite_at_all(
        &mut self,
        fh: mpi_sys::MPI_File,
        offset: mpi_sys::MPI_Offset,
        buf: *const c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        request: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn post_file_iwrite_at_all(
        &mut self,
        output: c_int,
        fh: mpi_sys::MPI_File,
        offset: mpi_sys::MPI_Offset,
        buf: *const c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        request: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn pre_file_iwrite_shared(
        &mut self,
        fh: mpi_sys::MPI_File,
        buf: *const c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        request: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn post_file_iwrite_shared(
        &mut self,
        output: c_int,
        fh: mpi_sys::MPI_File,
        buf: *const c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        request: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn pre_file_open(
        &mut self,
        comm: mpi_sys::MPI_Comm,
        filename: *const c_char,
        amode: c_int,
        info: mpi_sys::MPI_Info,
        fh: *mut mpi_sys::MPI_File,
    ) {
    }
    #[inline]
    fn post_file_open(
        &mut self,
        output: c_int,
        comm: mpi_sys::MPI_Comm,
        filename: *const c_char,
        amode: c_int,
        info: mpi_sys::MPI_Info,
        fh: *mut mpi_sys::MPI_File,
    ) {
    }
    #[inline]
    fn pre_file_preallocate(&mut self, fh: mpi_sys::MPI_File, size: mpi_sys::MPI_Offset) {}
    #[inline]
    fn post_file_preallocate(
        &mut self,
        output: c_int,
        fh: mpi_sys::MPI_File,
        size: mpi_sys::MPI_Offset,
    ) {
    }
    #[inline]
    fn pre_file_read(
        &mut self,
        fh: mpi_sys::MPI_File,
        buf: *mut c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        status: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn post_file_read(
        &mut self,
        output: c_int,
        fh: mpi_sys::MPI_File,
        buf: *mut c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        status: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn pre_file_read_all(
        &mut self,
        fh: mpi_sys::MPI_File,
        buf: *mut c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        status: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn post_file_read_all(
        &mut self,
        output: c_int,
        fh: mpi_sys::MPI_File,
        buf: *mut c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        status: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn pre_file_read_all_begin(
        &mut self,
        fh: mpi_sys::MPI_File,
        buf: *mut c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
    ) {
    }
    #[inline]
    fn post_file_read_all_begin(
        &mut self,
        output: c_int,
        fh: mpi_sys::MPI_File,
        buf: *mut c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
    ) {
    }
    #[inline]
    fn pre_file_read_all_end(
        &mut self,
        fh: mpi_sys::MPI_File,
        buf: *mut c_void,
        status: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn post_file_read_all_end(
        &mut self,
        output: c_int,
        fh: mpi_sys::MPI_File,
        buf: *mut c_void,
        status: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn pre_file_read_at(
        &mut self,
        fh: mpi_sys::MPI_File,
        offset: mpi_sys::MPI_Offset,
        buf: *mut c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        status: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn post_file_read_at(
        &mut self,
        output: c_int,
        fh: mpi_sys::MPI_File,
        offset: mpi_sys::MPI_Offset,
        buf: *mut c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        status: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn pre_file_read_at_all(
        &mut self,
        fh: mpi_sys::MPI_File,
        offset: mpi_sys::MPI_Offset,
        buf: *mut c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        status: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn post_file_read_at_all(
        &mut self,
        output: c_int,
        fh: mpi_sys::MPI_File,
        offset: mpi_sys::MPI_Offset,
        buf: *mut c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        status: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn pre_file_read_at_all_begin(
        &mut self,
        fh: mpi_sys::MPI_File,
        offset: mpi_sys::MPI_Offset,
        buf: *mut c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
    ) {
    }
    #[inline]
    fn post_file_read_at_all_begin(
        &mut self,
        output: c_int,
        fh: mpi_sys::MPI_File,
        offset: mpi_sys::MPI_Offset,
        buf: *mut c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
    ) {
    }
    #[inline]
    fn pre_file_read_at_all_end(
        &mut self,
        fh: mpi_sys::MPI_File,
        buf: *mut c_void,
        status: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn post_file_read_at_all_end(
        &mut self,
        output: c_int,
        fh: mpi_sys::MPI_File,
        buf: *mut c_void,
        status: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn pre_file_read_ordered(
        &mut self,
        fh: mpi_sys::MPI_File,
        buf: *mut c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        status: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn post_file_read_ordered(
        &mut self,
        output: c_int,
        fh: mpi_sys::MPI_File,
        buf: *mut c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        status: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn pre_file_read_ordered_begin(
        &mut self,
        fh: mpi_sys::MPI_File,
        buf: *mut c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
    ) {
    }
    #[inline]
    fn post_file_read_ordered_begin(
        &mut self,
        output: c_int,
        fh: mpi_sys::MPI_File,
        buf: *mut c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
    ) {
    }
    #[inline]
    fn pre_file_read_ordered_end(
        &mut self,
        fh: mpi_sys::MPI_File,
        buf: *mut c_void,
        status: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn post_file_read_ordered_end(
        &mut self,
        output: c_int,
        fh: mpi_sys::MPI_File,
        buf: *mut c_void,
        status: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn pre_file_read_shared(
        &mut self,
        fh: mpi_sys::MPI_File,
        buf: *mut c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        status: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn post_file_read_shared(
        &mut self,
        output: c_int,
        fh: mpi_sys::MPI_File,
        buf: *mut c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        status: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn pre_file_seek(&mut self, fh: mpi_sys::MPI_File, offset: mpi_sys::MPI_Offset, whence: c_int) {
    }
    #[inline]
    fn post_file_seek(
        &mut self,
        output: c_int,
        fh: mpi_sys::MPI_File,
        offset: mpi_sys::MPI_Offset,
        whence: c_int,
    ) {
    }
    #[inline]
    fn pre_file_seek_shared(
        &mut self,
        fh: mpi_sys::MPI_File,
        offset: mpi_sys::MPI_Offset,
        whence: c_int,
    ) {
    }
    #[inline]
    fn post_file_seek_shared(
        &mut self,
        output: c_int,
        fh: mpi_sys::MPI_File,
        offset: mpi_sys::MPI_Offset,
        whence: c_int,
    ) {
    }
    #[inline]
    fn pre_file_set_atomicity(&mut self, fh: mpi_sys::MPI_File, flag: c_int) {}
    #[inline]
    fn post_file_set_atomicity(&mut self, output: c_int, fh: mpi_sys::MPI_File, flag: c_int) {}
    #[inline]
    fn pre_file_set_errhandler(
        &mut self,
        file: mpi_sys::MPI_File,
        errhandler: mpi_sys::MPI_Errhandler,
    ) {
    }
    #[inline]
    fn post_file_set_errhandler(
        &mut self,
        output: c_int,
        file: mpi_sys::MPI_File,
        errhandler: mpi_sys::MPI_Errhandler,
    ) {
    }
    #[inline]
    fn pre_file_set_info(&mut self, fh: mpi_sys::MPI_File, info: mpi_sys::MPI_Info) {}
    #[inline]
    fn post_file_set_info(
        &mut self,
        output: c_int,
        fh: mpi_sys::MPI_File,
        info: mpi_sys::MPI_Info,
    ) {
    }
    #[inline]
    fn pre_file_set_size(&mut self, fh: mpi_sys::MPI_File, size: mpi_sys::MPI_Offset) {}
    #[inline]
    fn post_file_set_size(
        &mut self,
        output: c_int,
        fh: mpi_sys::MPI_File,
        size: mpi_sys::MPI_Offset,
    ) {
    }
    #[inline]
    fn pre_file_set_view(
        &mut self,
        fh: mpi_sys::MPI_File,
        disp: mpi_sys::MPI_Offset,
        etype: mpi_sys::MPI_Datatype,
        filetype: mpi_sys::MPI_Datatype,
        datarep: *const c_char,
        info: mpi_sys::MPI_Info,
    ) {
    }
    #[inline]
    fn post_file_set_view(
        &mut self,
        output: c_int,
        fh: mpi_sys::MPI_File,
        disp: mpi_sys::MPI_Offset,
        etype: mpi_sys::MPI_Datatype,
        filetype: mpi_sys::MPI_Datatype,
        datarep: *const c_char,
        info: mpi_sys::MPI_Info,
    ) {
    }
    #[inline]
    fn pre_file_sync(&mut self, fh: mpi_sys::MPI_File) {}
    #[inline]
    fn post_file_sync(&mut self, output: c_int, fh: mpi_sys::MPI_File) {}
    #[inline]
    fn pre_file_write(
        &mut self,
        fh: mpi_sys::MPI_File,
        buf: *const c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        status: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn post_file_write(
        &mut self,
        output: c_int,
        fh: mpi_sys::MPI_File,
        buf: *const c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        status: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn pre_file_write_all(
        &mut self,
        fh: mpi_sys::MPI_File,
        buf: *const c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        status: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn post_file_write_all(
        &mut self,
        output: c_int,
        fh: mpi_sys::MPI_File,
        buf: *const c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        status: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn pre_file_write_all_begin(
        &mut self,
        fh: mpi_sys::MPI_File,
        buf: *const c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
    ) {
    }
    #[inline]
    fn post_file_write_all_begin(
        &mut self,
        output: c_int,
        fh: mpi_sys::MPI_File,
        buf: *const c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
    ) {
    }
    #[inline]
    fn pre_file_write_all_end(
        &mut self,
        fh: mpi_sys::MPI_File,
        buf: *const c_void,
        status: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn post_file_write_all_end(
        &mut self,
        output: c_int,
        fh: mpi_sys::MPI_File,
        buf: *const c_void,
        status: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn pre_file_write_at(
        &mut self,
        fh: mpi_sys::MPI_File,
        offset: mpi_sys::MPI_Offset,
        buf: *const c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        status: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn post_file_write_at(
        &mut self,
        output: c_int,
        fh: mpi_sys::MPI_File,
        offset: mpi_sys::MPI_Offset,
        buf: *const c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        status: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn pre_file_write_at_all(
        &mut self,
        fh: mpi_sys::MPI_File,
        offset: mpi_sys::MPI_Offset,
        buf: *const c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        status: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn post_file_write_at_all(
        &mut self,
        output: c_int,
        fh: mpi_sys::MPI_File,
        offset: mpi_sys::MPI_Offset,
        buf: *const c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        status: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn pre_file_write_at_all_begin(
        &mut self,
        fh: mpi_sys::MPI_File,
        offset: mpi_sys::MPI_Offset,
        buf: *const c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
    ) {
    }
    #[inline]
    fn post_file_write_at_all_begin(
        &mut self,
        output: c_int,
        fh: mpi_sys::MPI_File,
        offset: mpi_sys::MPI_Offset,
        buf: *const c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
    ) {
    }
    #[inline]
    fn pre_file_write_at_all_end(
        &mut self,
        fh: mpi_sys::MPI_File,
        buf: *const c_void,
        status: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn post_file_write_at_all_end(
        &mut self,
        output: c_int,
        fh: mpi_sys::MPI_File,
        buf: *const c_void,
        status: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn pre_file_write_ordered(
        &mut self,
        fh: mpi_sys::MPI_File,
        buf: *const c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        status: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn post_file_write_ordered(
        &mut self,
        output: c_int,
        fh: mpi_sys::MPI_File,
        buf: *const c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        status: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn pre_file_write_ordered_begin(
        &mut self,
        fh: mpi_sys::MPI_File,
        buf: *const c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
    ) {
    }
    #[inline]
    fn post_file_write_ordered_begin(
        &mut self,
        output: c_int,
        fh: mpi_sys::MPI_File,
        buf: *const c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
    ) {
    }
    #[inline]
    fn pre_file_write_ordered_end(
        &mut self,
        fh: mpi_sys::MPI_File,
        buf: *const c_void,
        status: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn post_file_write_ordered_end(
        &mut self,
        output: c_int,
        fh: mpi_sys::MPI_File,
        buf: *const c_void,
        status: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn pre_file_write_shared(
        &mut self,
        fh: mpi_sys::MPI_File,
        buf: *const c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        status: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn post_file_write_shared(
        &mut self,
        output: c_int,
        fh: mpi_sys::MPI_File,
        buf: *const c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        status: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn pre_finalize(&mut self) {}
    #[inline]
    fn post_finalize(&mut self, output: c_int) {}
    #[inline]
    fn pre_finalized(&mut self, flag: *mut c_int) {}
    #[inline]
    fn post_finalized(&mut self, output: c_int, flag: *mut c_int) {}
    #[inline]
    fn pre_free_mem(&mut self, base: *mut c_void) {}
    #[inline]
    fn post_free_mem(&mut self, output: c_int, base: *mut c_void) {}
    #[inline]
    fn pre_gather(
        &mut self,
        sendbuf: *const c_void,
        sendcount: c_int,
        sendtype: mpi_sys::MPI_Datatype,
        recvbuf: *mut c_void,
        recvcount: c_int,
        recvtype: mpi_sys::MPI_Datatype,
        root: c_int,
        comm: mpi_sys::MPI_Comm,
    ) {
    }
    #[inline]
    fn post_gather(
        &mut self,
        output: c_int,
        sendbuf: *const c_void,
        sendcount: c_int,
        sendtype: mpi_sys::MPI_Datatype,
        recvbuf: *mut c_void,
        recvcount: c_int,
        recvtype: mpi_sys::MPI_Datatype,
        root: c_int,
        comm: mpi_sys::MPI_Comm,
    ) {
    }
    #[inline]
    fn pre_gatherv(
        &mut self,
        sendbuf: *const c_void,
        sendcount: c_int,
        sendtype: mpi_sys::MPI_Datatype,
        recvbuf: *mut c_void,
        recvcounts: *const c_int,
        displs: *const c_int,
        recvtype: mpi_sys::MPI_Datatype,
        root: c_int,
        comm: mpi_sys::MPI_Comm,
    ) {
    }
    #[inline]
    fn post_gatherv(
        &mut self,
        output: c_int,
        sendbuf: *const c_void,
        sendcount: c_int,
        sendtype: mpi_sys::MPI_Datatype,
        recvbuf: *mut c_void,
        recvcounts: *const c_int,
        displs: *const c_int,
        recvtype: mpi_sys::MPI_Datatype,
        root: c_int,
        comm: mpi_sys::MPI_Comm,
    ) {
    }
    #[inline]
    fn pre_get(
        &mut self,
        origin_addr: *mut c_void,
        origin_count: c_int,
        origin_datatype: mpi_sys::MPI_Datatype,
        target_rank: c_int,
        target_disp: mpi_sys::MPI_Aint,
        target_count: c_int,
        target_datatype: mpi_sys::MPI_Datatype,
        win: mpi_sys::MPI_Win,
    ) {
    }
    #[inline]
    fn post_get(
        &mut self,
        output: c_int,
        origin_addr: *mut c_void,
        origin_count: c_int,
        origin_datatype: mpi_sys::MPI_Datatype,
        target_rank: c_int,
        target_disp: mpi_sys::MPI_Aint,
        target_count: c_int,
        target_datatype: mpi_sys::MPI_Datatype,
        win: mpi_sys::MPI_Win,
    ) {
    }
    #[inline]
    fn pre_get_accumulate(
        &mut self,
        origin_addr: *const c_void,
        origin_count: c_int,
        origin_datatype: mpi_sys::MPI_Datatype,
        result_addr: *mut c_void,
        result_count: c_int,
        result_datatype: mpi_sys::MPI_Datatype,
        target_rank: c_int,
        target_disp: mpi_sys::MPI_Aint,
        target_count: c_int,
        target_datatype: mpi_sys::MPI_Datatype,
        op: mpi_sys::MPI_Op,
        win: mpi_sys::MPI_Win,
    ) {
    }
    #[inline]
    fn post_get_accumulate(
        &mut self,
        output: c_int,
        origin_addr: *const c_void,
        origin_count: c_int,
        origin_datatype: mpi_sys::MPI_Datatype,
        result_addr: *mut c_void,
        result_count: c_int,
        result_datatype: mpi_sys::MPI_Datatype,
        target_rank: c_int,
        target_disp: mpi_sys::MPI_Aint,
        target_count: c_int,
        target_datatype: mpi_sys::MPI_Datatype,
        op: mpi_sys::MPI_Op,
        win: mpi_sys::MPI_Win,
    ) {
    }
    #[inline]
    fn pre_get_address(&mut self, location: *const c_void, address: *mut mpi_sys::MPI_Aint) {}
    #[inline]
    fn post_get_address(
        &mut self,
        output: c_int,
        location: *const c_void,
        address: *mut mpi_sys::MPI_Aint,
    ) {
    }
    #[inline]
    fn pre_get_count(
        &mut self,
        status: *const mpi_sys::MPI_Status,
        datatype: mpi_sys::MPI_Datatype,
        count: *mut c_int,
    ) {
    }
    #[inline]
    fn post_get_count(
        &mut self,
        output: c_int,
        status: *const mpi_sys::MPI_Status,
        datatype: mpi_sys::MPI_Datatype,
        count: *mut c_int,
    ) {
    }
    #[inline]
    fn pre_get_elements(
        &mut self,
        status: *const mpi_sys::MPI_Status,
        datatype: mpi_sys::MPI_Datatype,
        count: *mut c_int,
    ) {
    }
    #[inline]
    fn post_get_elements(
        &mut self,
        output: c_int,
        status: *const mpi_sys::MPI_Status,
        datatype: mpi_sys::MPI_Datatype,
        count: *mut c_int,
    ) {
    }
    #[inline]
    fn pre_get_elements_x(
        &mut self,
        status: *const mpi_sys::MPI_Status,
        datatype: mpi_sys::MPI_Datatype,
        count: *mut mpi_sys::MPI_Count,
    ) {
    }
    #[inline]
    fn post_get_elements_x(
        &mut self,
        output: c_int,
        status: *const mpi_sys::MPI_Status,
        datatype: mpi_sys::MPI_Datatype,
        count: *mut mpi_sys::MPI_Count,
    ) {
    }
    #[inline]
    fn pre_get_library_version(&mut self, version: *mut c_char, resultlen: *mut c_int) {}
    #[inline]
    fn post_get_library_version(
        &mut self,
        output: c_int,
        version: *mut c_char,
        resultlen: *mut c_int,
    ) {
    }
    #[inline]
    fn pre_get_processor_name(&mut self, name: *mut c_char, resultlen: *mut c_int) {}
    #[inline]
    fn post_get_processor_name(&mut self, output: c_int, name: *mut c_char, resultlen: *mut c_int) {
    }
    #[inline]
    fn pre_get_version(&mut self, version: *mut c_int, subversion: *mut c_int) {}
    #[inline]
    fn post_get_version(&mut self, output: c_int, version: *mut c_int, subversion: *mut c_int) {}
    #[inline]
    fn pre_graph_create(
        &mut self,
        comm_old: mpi_sys::MPI_Comm,
        nnodes: c_int,
        index: *const c_int,
        edges: *const c_int,
        reorder: c_int,
        comm_graph: *mut mpi_sys::MPI_Comm,
    ) {
    }
    #[inline]
    fn post_graph_create(
        &mut self,
        output: c_int,
        comm_old: mpi_sys::MPI_Comm,
        nnodes: c_int,
        index: *const c_int,
        edges: *const c_int,
        reorder: c_int,
        comm_graph: *mut mpi_sys::MPI_Comm,
    ) {
    }
    #[inline]
    fn pre_graph_get(
        &mut self,
        comm: mpi_sys::MPI_Comm,
        maxindex: c_int,
        maxedges: c_int,
        index: *mut c_int,
        edges: *mut c_int,
    ) {
    }
    #[inline]
    fn post_graph_get(
        &mut self,
        output: c_int,
        comm: mpi_sys::MPI_Comm,
        maxindex: c_int,
        maxedges: c_int,
        index: *mut c_int,
        edges: *mut c_int,
    ) {
    }
    #[inline]
    fn pre_graph_map(
        &mut self,
        comm: mpi_sys::MPI_Comm,
        nnodes: c_int,
        index: *const c_int,
        edges: *const c_int,
        newrank: *mut c_int,
    ) {
    }
    #[inline]
    fn post_graph_map(
        &mut self,
        output: c_int,
        comm: mpi_sys::MPI_Comm,
        nnodes: c_int,
        index: *const c_int,
        edges: *const c_int,
        newrank: *mut c_int,
    ) {
    }
    #[inline]
    fn pre_graph_neighbors(
        &mut self,
        comm: mpi_sys::MPI_Comm,
        rank: c_int,
        maxneighbors: c_int,
        neighbors: *mut c_int,
    ) {
    }
    #[inline]
    fn post_graph_neighbors(
        &mut self,
        output: c_int,
        comm: mpi_sys::MPI_Comm,
        rank: c_int,
        maxneighbors: c_int,
        neighbors: *mut c_int,
    ) {
    }
    #[inline]
    fn pre_graph_neighbors_count(
        &mut self,
        comm: mpi_sys::MPI_Comm,
        rank: c_int,
        nneighbors: *mut c_int,
    ) {
    }
    #[inline]
    fn post_graph_neighbors_count(
        &mut self,
        output: c_int,
        comm: mpi_sys::MPI_Comm,
        rank: c_int,
        nneighbors: *mut c_int,
    ) {
    }
    #[inline]
    fn pre_graphdims_get(
        &mut self,
        comm: mpi_sys::MPI_Comm,
        nnodes: *mut c_int,
        nedges: *mut c_int,
    ) {
    }
    #[inline]
    fn post_graphdims_get(
        &mut self,
        output: c_int,
        comm: mpi_sys::MPI_Comm,
        nnodes: *mut c_int,
        nedges: *mut c_int,
    ) {
    }
    #[inline]
    fn pre_grequest_complete(&mut self, request: mpi_sys::MPI_Request) {}
    #[inline]
    fn post_grequest_complete(&mut self, output: c_int, request: mpi_sys::MPI_Request) {}
    #[inline]
    fn pre_grequest_start(
        &mut self,
        query_fn: *mut mpi_sys::MPI_Grequest_query_function,
        free_fn: *mut mpi_sys::MPI_Grequest_free_function,
        cancel_fn: *mut mpi_sys::MPI_Grequest_cancel_function,
        extra_state: *mut c_void,
        request: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn post_grequest_start(
        &mut self,
        output: c_int,
        query_fn: *mut mpi_sys::MPI_Grequest_query_function,
        free_fn: *mut mpi_sys::MPI_Grequest_free_function,
        cancel_fn: *mut mpi_sys::MPI_Grequest_cancel_function,
        extra_state: *mut c_void,
        request: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn pre_group_compare(
        &mut self,
        group1: mpi_sys::MPI_Group,
        group2: mpi_sys::MPI_Group,
        result: *mut c_int,
    ) {
    }
    #[inline]
    fn post_group_compare(
        &mut self,
        output: c_int,
        group1: mpi_sys::MPI_Group,
        group2: mpi_sys::MPI_Group,
        result: *mut c_int,
    ) {
    }
    #[inline]
    fn pre_group_difference(
        &mut self,
        group1: mpi_sys::MPI_Group,
        group2: mpi_sys::MPI_Group,
        newgroup: *mut mpi_sys::MPI_Group,
    ) {
    }
    #[inline]
    fn post_group_difference(
        &mut self,
        output: c_int,
        group1: mpi_sys::MPI_Group,
        group2: mpi_sys::MPI_Group,
        newgroup: *mut mpi_sys::MPI_Group,
    ) {
    }
    #[inline]
    fn pre_group_excl(
        &mut self,
        group: mpi_sys::MPI_Group,
        n: c_int,
        ranks: *const c_int,
        newgroup: *mut mpi_sys::MPI_Group,
    ) {
    }
    #[inline]
    fn post_group_excl(
        &mut self,
        output: c_int,
        group: mpi_sys::MPI_Group,
        n: c_int,
        ranks: *const c_int,
        newgroup: *mut mpi_sys::MPI_Group,
    ) {
    }
    #[inline]
    fn pre_group_free(&mut self, group: *mut mpi_sys::MPI_Group) {}
    #[inline]
    fn post_group_free(&mut self, output: c_int, group: *mut mpi_sys::MPI_Group) {}
    #[inline]
    fn pre_group_incl(
        &mut self,
        group: mpi_sys::MPI_Group,
        n: c_int,
        ranks: *const c_int,
        newgroup: *mut mpi_sys::MPI_Group,
    ) {
    }
    #[inline]
    fn post_group_incl(
        &mut self,
        output: c_int,
        group: mpi_sys::MPI_Group,
        n: c_int,
        ranks: *const c_int,
        newgroup: *mut mpi_sys::MPI_Group,
    ) {
    }
    #[inline]
    fn pre_group_intersection(
        &mut self,
        group1: mpi_sys::MPI_Group,
        group2: mpi_sys::MPI_Group,
        newgroup: *mut mpi_sys::MPI_Group,
    ) {
    }
    #[inline]
    fn post_group_intersection(
        &mut self,
        output: c_int,
        group1: mpi_sys::MPI_Group,
        group2: mpi_sys::MPI_Group,
        newgroup: *mut mpi_sys::MPI_Group,
    ) {
    }
    #[inline]
    fn pre_group_range_excl(
        &mut self,
        group: mpi_sys::MPI_Group,
        n: c_int,
        ranges: *mut [c_int; 3],
        newgroup: *mut mpi_sys::MPI_Group,
    ) {
    }
    #[inline]
    fn post_group_range_excl(
        &mut self,
        output: c_int,
        group: mpi_sys::MPI_Group,
        n: c_int,
        ranges: *mut [c_int; 3],
        newgroup: *mut mpi_sys::MPI_Group,
    ) {
    }
    #[inline]
    fn pre_group_range_incl(
        &mut self,
        group: mpi_sys::MPI_Group,
        n: c_int,
        ranges: *mut [c_int; 3],
        newgroup: *mut mpi_sys::MPI_Group,
    ) {
    }
    #[inline]
    fn post_group_range_incl(
        &mut self,
        output: c_int,
        group: mpi_sys::MPI_Group,
        n: c_int,
        ranges: *mut [c_int; 3],
        newgroup: *mut mpi_sys::MPI_Group,
    ) {
    }
    #[inline]
    fn pre_group_rank(&mut self, group: mpi_sys::MPI_Group, rank: *mut c_int) {}
    #[inline]
    fn post_group_rank(&mut self, output: c_int, group: mpi_sys::MPI_Group, rank: *mut c_int) {}
    #[inline]
    fn pre_group_size(&mut self, group: mpi_sys::MPI_Group, size: *mut c_int) {}
    #[inline]
    fn post_group_size(&mut self, output: c_int, group: mpi_sys::MPI_Group, size: *mut c_int) {}
    #[inline]
    fn pre_group_translate_ranks(
        &mut self,
        group1: mpi_sys::MPI_Group,
        n: c_int,
        ranks1: *const c_int,
        group2: mpi_sys::MPI_Group,
        ranks2: *mut c_int,
    ) {
    }
    #[inline]
    fn post_group_translate_ranks(
        &mut self,
        output: c_int,
        group1: mpi_sys::MPI_Group,
        n: c_int,
        ranks1: *const c_int,
        group2: mpi_sys::MPI_Group,
        ranks2: *mut c_int,
    ) {
    }
    #[inline]
    fn pre_group_union(
        &mut self,
        group1: mpi_sys::MPI_Group,
        group2: mpi_sys::MPI_Group,
        newgroup: *mut mpi_sys::MPI_Group,
    ) {
    }
    #[inline]
    fn post_group_union(
        &mut self,
        output: c_int,
        group1: mpi_sys::MPI_Group,
        group2: mpi_sys::MPI_Group,
        newgroup: *mut mpi_sys::MPI_Group,
    ) {
    }
    #[inline]
    fn pre_iallgather(
        &mut self,
        sendbuf: *const c_void,
        sendcount: c_int,
        sendtype: mpi_sys::MPI_Datatype,
        recvbuf: *mut c_void,
        recvcount: c_int,
        recvtype: mpi_sys::MPI_Datatype,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn post_iallgather(
        &mut self,
        output: c_int,
        sendbuf: *const c_void,
        sendcount: c_int,
        sendtype: mpi_sys::MPI_Datatype,
        recvbuf: *mut c_void,
        recvcount: c_int,
        recvtype: mpi_sys::MPI_Datatype,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn pre_iallgatherv(
        &mut self,
        sendbuf: *const c_void,
        sendcount: c_int,
        sendtype: mpi_sys::MPI_Datatype,
        recvbuf: *mut c_void,
        recvcounts: *const c_int,
        displs: *const c_int,
        recvtype: mpi_sys::MPI_Datatype,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn post_iallgatherv(
        &mut self,
        output: c_int,
        sendbuf: *const c_void,
        sendcount: c_int,
        sendtype: mpi_sys::MPI_Datatype,
        recvbuf: *mut c_void,
        recvcounts: *const c_int,
        displs: *const c_int,
        recvtype: mpi_sys::MPI_Datatype,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn pre_iallreduce(
        &mut self,
        sendbuf: *const c_void,
        recvbuf: *mut c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        op: mpi_sys::MPI_Op,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn post_iallreduce(
        &mut self,
        output: c_int,
        sendbuf: *const c_void,
        recvbuf: *mut c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        op: mpi_sys::MPI_Op,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn pre_ialltoall(
        &mut self,
        sendbuf: *const c_void,
        sendcount: c_int,
        sendtype: mpi_sys::MPI_Datatype,
        recvbuf: *mut c_void,
        recvcount: c_int,
        recvtype: mpi_sys::MPI_Datatype,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn post_ialltoall(
        &mut self,
        output: c_int,
        sendbuf: *const c_void,
        sendcount: c_int,
        sendtype: mpi_sys::MPI_Datatype,
        recvbuf: *mut c_void,
        recvcount: c_int,
        recvtype: mpi_sys::MPI_Datatype,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn pre_ialltoallv(
        &mut self,
        sendbuf: *const c_void,
        sendcounts: *const c_int,
        sdispls: *const c_int,
        sendtype: mpi_sys::MPI_Datatype,
        recvbuf: *mut c_void,
        recvcounts: *const c_int,
        rdispls: *const c_int,
        recvtype: mpi_sys::MPI_Datatype,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn post_ialltoallv(
        &mut self,
        output: c_int,
        sendbuf: *const c_void,
        sendcounts: *const c_int,
        sdispls: *const c_int,
        sendtype: mpi_sys::MPI_Datatype,
        recvbuf: *mut c_void,
        recvcounts: *const c_int,
        rdispls: *const c_int,
        recvtype: mpi_sys::MPI_Datatype,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn pre_ialltoallw(
        &mut self,
        sendbuf: *const c_void,
        sendcounts: *const c_int,
        sdispls: *const c_int,
        sendtypes: *const mpi_sys::MPI_Datatype,
        recvbuf: *mut c_void,
        recvcounts: *const c_int,
        rdispls: *const c_int,
        recvtypes: *const mpi_sys::MPI_Datatype,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn post_ialltoallw(
        &mut self,
        output: c_int,
        sendbuf: *const c_void,
        sendcounts: *const c_int,
        sdispls: *const c_int,
        sendtypes: *const mpi_sys::MPI_Datatype,
        recvbuf: *mut c_void,
        recvcounts: *const c_int,
        rdispls: *const c_int,
        recvtypes: *const mpi_sys::MPI_Datatype,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn pre_ibarrier(&mut self, comm: mpi_sys::MPI_Comm, request: *mut mpi_sys::MPI_Request) {}
    #[inline]
    fn post_ibarrier(
        &mut self,
        output: c_int,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn pre_ibcast(
        &mut self,
        buffer: *mut c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        root: c_int,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn post_ibcast(
        &mut self,
        output: c_int,
        buffer: *mut c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        root: c_int,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn pre_ibsend(
        &mut self,
        buf: *const c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        dest: c_int,
        tag: c_int,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn post_ibsend(
        &mut self,
        output: c_int,
        buf: *const c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        dest: c_int,
        tag: c_int,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn pre_iexscan(
        &mut self,
        sendbuf: *const c_void,
        recvbuf: *mut c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        op: mpi_sys::MPI_Op,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn post_iexscan(
        &mut self,
        output: c_int,
        sendbuf: *const c_void,
        recvbuf: *mut c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        op: mpi_sys::MPI_Op,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn pre_igather(
        &mut self,
        sendbuf: *const c_void,
        sendcount: c_int,
        sendtype: mpi_sys::MPI_Datatype,
        recvbuf: *mut c_void,
        recvcount: c_int,
        recvtype: mpi_sys::MPI_Datatype,
        root: c_int,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn post_igather(
        &mut self,
        output: c_int,
        sendbuf: *const c_void,
        sendcount: c_int,
        sendtype: mpi_sys::MPI_Datatype,
        recvbuf: *mut c_void,
        recvcount: c_int,
        recvtype: mpi_sys::MPI_Datatype,
        root: c_int,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn pre_igatherv(
        &mut self,
        sendbuf: *const c_void,
        sendcount: c_int,
        sendtype: mpi_sys::MPI_Datatype,
        recvbuf: *mut c_void,
        recvcounts: *const c_int,
        displs: *const c_int,
        recvtype: mpi_sys::MPI_Datatype,
        root: c_int,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn post_igatherv(
        &mut self,
        output: c_int,
        sendbuf: *const c_void,
        sendcount: c_int,
        sendtype: mpi_sys::MPI_Datatype,
        recvbuf: *mut c_void,
        recvcounts: *const c_int,
        displs: *const c_int,
        recvtype: mpi_sys::MPI_Datatype,
        root: c_int,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn pre_improbe(
        &mut self,
        source: c_int,
        tag: c_int,
        comm: mpi_sys::MPI_Comm,
        flag: *mut c_int,
        message: *mut mpi_sys::MPI_Message,
        status: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn post_improbe(
        &mut self,
        output: c_int,
        source: c_int,
        tag: c_int,
        comm: mpi_sys::MPI_Comm,
        flag: *mut c_int,
        message: *mut mpi_sys::MPI_Message,
        status: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn pre_imrecv(
        &mut self,
        buf: *mut c_void,
        count: c_int,
        r#type: mpi_sys::MPI_Datatype,
        message: *mut mpi_sys::MPI_Message,
        request: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn post_imrecv(
        &mut self,
        output: c_int,
        buf: *mut c_void,
        count: c_int,
        r#type: mpi_sys::MPI_Datatype,
        message: *mut mpi_sys::MPI_Message,
        request: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn pre_ineighbor_allgather(
        &mut self,
        sendbuf: *const c_void,
        sendcount: c_int,
        sendtype: mpi_sys::MPI_Datatype,
        recvbuf: *mut c_void,
        recvcount: c_int,
        recvtype: mpi_sys::MPI_Datatype,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn post_ineighbor_allgather(
        &mut self,
        output: c_int,
        sendbuf: *const c_void,
        sendcount: c_int,
        sendtype: mpi_sys::MPI_Datatype,
        recvbuf: *mut c_void,
        recvcount: c_int,
        recvtype: mpi_sys::MPI_Datatype,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn pre_ineighbor_allgatherv(
        &mut self,
        sendbuf: *const c_void,
        sendcount: c_int,
        sendtype: mpi_sys::MPI_Datatype,
        recvbuf: *mut c_void,
        recvcounts: *const c_int,
        displs: *const c_int,
        recvtype: mpi_sys::MPI_Datatype,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn post_ineighbor_allgatherv(
        &mut self,
        output: c_int,
        sendbuf: *const c_void,
        sendcount: c_int,
        sendtype: mpi_sys::MPI_Datatype,
        recvbuf: *mut c_void,
        recvcounts: *const c_int,
        displs: *const c_int,
        recvtype: mpi_sys::MPI_Datatype,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn pre_ineighbor_alltoall(
        &mut self,
        sendbuf: *const c_void,
        sendcount: c_int,
        sendtype: mpi_sys::MPI_Datatype,
        recvbuf: *mut c_void,
        recvcount: c_int,
        recvtype: mpi_sys::MPI_Datatype,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn post_ineighbor_alltoall(
        &mut self,
        output: c_int,
        sendbuf: *const c_void,
        sendcount: c_int,
        sendtype: mpi_sys::MPI_Datatype,
        recvbuf: *mut c_void,
        recvcount: c_int,
        recvtype: mpi_sys::MPI_Datatype,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn pre_ineighbor_alltoallv(
        &mut self,
        sendbuf: *const c_void,
        sendcounts: *const c_int,
        sdispls: *const c_int,
        sendtype: mpi_sys::MPI_Datatype,
        recvbuf: *mut c_void,
        recvcounts: *const c_int,
        rdispls: *const c_int,
        recvtype: mpi_sys::MPI_Datatype,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn post_ineighbor_alltoallv(
        &mut self,
        output: c_int,
        sendbuf: *const c_void,
        sendcounts: *const c_int,
        sdispls: *const c_int,
        sendtype: mpi_sys::MPI_Datatype,
        recvbuf: *mut c_void,
        recvcounts: *const c_int,
        rdispls: *const c_int,
        recvtype: mpi_sys::MPI_Datatype,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn pre_ineighbor_alltoallw(
        &mut self,
        sendbuf: *const c_void,
        sendcounts: *const c_int,
        sdispls: *const mpi_sys::MPI_Aint,
        sendtypes: *const mpi_sys::MPI_Datatype,
        recvbuf: *mut c_void,
        recvcounts: *const c_int,
        rdispls: *const mpi_sys::MPI_Aint,
        recvtypes: *const mpi_sys::MPI_Datatype,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn post_ineighbor_alltoallw(
        &mut self,
        output: c_int,
        sendbuf: *const c_void,
        sendcounts: *const c_int,
        sdispls: *const mpi_sys::MPI_Aint,
        sendtypes: *const mpi_sys::MPI_Datatype,
        recvbuf: *mut c_void,
        recvcounts: *const c_int,
        rdispls: *const mpi_sys::MPI_Aint,
        recvtypes: *const mpi_sys::MPI_Datatype,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn pre_info_create(&mut self, info: *mut mpi_sys::MPI_Info) {}
    #[inline]
    fn post_info_create(&mut self, output: c_int, info: *mut mpi_sys::MPI_Info) {}
    #[inline]
    fn pre_info_delete(&mut self, info: mpi_sys::MPI_Info, key: *const c_char) {}
    #[inline]
    fn post_info_delete(&mut self, output: c_int, info: mpi_sys::MPI_Info, key: *const c_char) {}
    #[inline]
    fn pre_info_dup(&mut self, info: mpi_sys::MPI_Info, newinfo: *mut mpi_sys::MPI_Info) {}
    #[inline]
    fn post_info_dup(
        &mut self,
        output: c_int,
        info: mpi_sys::MPI_Info,
        newinfo: *mut mpi_sys::MPI_Info,
    ) {
    }
    #[inline]
    fn pre_info_free(&mut self, info: *mut mpi_sys::MPI_Info) {}
    #[inline]
    fn post_info_free(&mut self, output: c_int, info: *mut mpi_sys::MPI_Info) {}
    #[inline]
    fn pre_info_get(
        &mut self,
        info: mpi_sys::MPI_Info,
        key: *const c_char,
        valuelen: c_int,
        value: *mut c_char,
        flag: *mut c_int,
    ) {
    }
    #[inline]
    fn post_info_get(
        &mut self,
        output: c_int,
        info: mpi_sys::MPI_Info,
        key: *const c_char,
        valuelen: c_int,
        value: *mut c_char,
        flag: *mut c_int,
    ) {
    }
    #[inline]
    fn pre_info_get_nkeys(&mut self, info: mpi_sys::MPI_Info, nkeys: *mut c_int) {}
    #[inline]
    fn post_info_get_nkeys(&mut self, output: c_int, info: mpi_sys::MPI_Info, nkeys: *mut c_int) {}
    #[inline]
    fn pre_info_get_nthkey(&mut self, info: mpi_sys::MPI_Info, n: c_int, key: *mut c_char) {}
    #[inline]
    fn post_info_get_nthkey(
        &mut self,
        output: c_int,
        info: mpi_sys::MPI_Info,
        n: c_int,
        key: *mut c_char,
    ) {
    }
    #[inline]
    fn pre_info_get_valuelen(
        &mut self,
        info: mpi_sys::MPI_Info,
        key: *const c_char,
        valuelen: *mut c_int,
        flag: *mut c_int,
        i: c_int,
        v: *mut qmpi_sys::vector,
    ) {
    }
    #[inline]
    fn post_info_get_valuelen(
        &mut self,
        output: c_int,
        info: mpi_sys::MPI_Info,
        key: *const c_char,
        valuelen: *mut c_int,
        flag: *mut c_int,
        i: c_int,
        v: *mut qmpi_sys::vector,
    ) {
    }
    #[inline]
    fn pre_info_set(&mut self, info: mpi_sys::MPI_Info, key: *const c_char, value: *const c_char) {}
    #[inline]
    fn post_info_set(
        &mut self,
        output: c_int,
        info: mpi_sys::MPI_Info,
        key: *const c_char,
        value: *const c_char,
    ) {
    }
    #[inline]
    fn pre_init(&mut self, argc: *mut c_int, argv: *mut *mut *mut c_char) {}
    #[inline]
    fn post_init(&mut self, output: c_int, argc: *mut c_int, argv: *mut *mut *mut c_char) {}
    #[inline]
    fn pre_init_thread(
        &mut self,
        argc: *mut c_int,
        argv: *mut *mut *mut c_char,
        required: c_int,
        provided: *mut c_int,
    ) {
    }
    #[inline]
    fn post_init_thread(
        &mut self,
        output: c_int,
        argc: *mut c_int,
        argv: *mut *mut *mut c_char,
        required: c_int,
        provided: *mut c_int,
    ) {
    }
    #[inline]
    fn pre_initialized(&mut self, flag: *mut c_int) {}
    #[inline]
    fn post_initialized(&mut self, output: c_int, flag: *mut c_int) {}
    #[inline]
    fn pre_intercomm_create(
        &mut self,
        local_comm: mpi_sys::MPI_Comm,
        local_leader: c_int,
        bridge_comm: mpi_sys::MPI_Comm,
        remote_leader: c_int,
        tag: c_int,
        newintercomm: *mut mpi_sys::MPI_Comm,
    ) {
    }
    #[inline]
    fn post_intercomm_create(
        &mut self,
        output: c_int,
        local_comm: mpi_sys::MPI_Comm,
        local_leader: c_int,
        bridge_comm: mpi_sys::MPI_Comm,
        remote_leader: c_int,
        tag: c_int,
        newintercomm: *mut mpi_sys::MPI_Comm,
    ) {
    }
    #[inline]
    fn pre_intercomm_merge(
        &mut self,
        intercomm: mpi_sys::MPI_Comm,
        high: c_int,
        newintercomm: *mut mpi_sys::MPI_Comm,
    ) {
    }
    #[inline]
    fn post_intercomm_merge(
        &mut self,
        output: c_int,
        intercomm: mpi_sys::MPI_Comm,
        high: c_int,
        newintercomm: *mut mpi_sys::MPI_Comm,
    ) {
    }
    #[inline]
    fn pre_iprobe(
        &mut self,
        source: c_int,
        tag: c_int,
        comm: mpi_sys::MPI_Comm,
        flag: *mut c_int,
        status: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn post_iprobe(
        &mut self,
        output: c_int,
        source: c_int,
        tag: c_int,
        comm: mpi_sys::MPI_Comm,
        flag: *mut c_int,
        status: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn pre_irecv(
        &mut self,
        buf: *mut c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        source: c_int,
        tag: c_int,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn post_irecv(
        &mut self,
        output: c_int,
        buf: *mut c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        source: c_int,
        tag: c_int,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn pre_ireduce(
        &mut self,
        sendbuf: *const c_void,
        recvbuf: *mut c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        op: mpi_sys::MPI_Op,
        root: c_int,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn post_ireduce(
        &mut self,
        output: c_int,
        sendbuf: *const c_void,
        recvbuf: *mut c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        op: mpi_sys::MPI_Op,
        root: c_int,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn pre_ireduce_scatter(
        &mut self,
        sendbuf: *const c_void,
        recvbuf: *mut c_void,
        recvcounts: *const c_int,
        datatype: mpi_sys::MPI_Datatype,
        op: mpi_sys::MPI_Op,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn post_ireduce_scatter(
        &mut self,
        output: c_int,
        sendbuf: *const c_void,
        recvbuf: *mut c_void,
        recvcounts: *const c_int,
        datatype: mpi_sys::MPI_Datatype,
        op: mpi_sys::MPI_Op,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn pre_ireduce_scatter_block(
        &mut self,
        sendbuf: *const c_void,
        recvbuf: *mut c_void,
        recvcount: c_int,
        datatype: mpi_sys::MPI_Datatype,
        op: mpi_sys::MPI_Op,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn post_ireduce_scatter_block(
        &mut self,
        output: c_int,
        sendbuf: *const c_void,
        recvbuf: *mut c_void,
        recvcount: c_int,
        datatype: mpi_sys::MPI_Datatype,
        op: mpi_sys::MPI_Op,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn pre_irsend(
        &mut self,
        buf: *const c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        dest: c_int,
        tag: c_int,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn post_irsend(
        &mut self,
        output: c_int,
        buf: *const c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        dest: c_int,
        tag: c_int,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn pre_is_thread_main(&mut self, flag: *mut c_int) {}
    #[inline]
    fn post_is_thread_main(&mut self, output: c_int, flag: *mut c_int) {}
    #[inline]
    fn pre_iscan(
        &mut self,
        sendbuf: *const c_void,
        recvbuf: *mut c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        op: mpi_sys::MPI_Op,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn post_iscan(
        &mut self,
        output: c_int,
        sendbuf: *const c_void,
        recvbuf: *mut c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        op: mpi_sys::MPI_Op,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn pre_iscatter(
        &mut self,
        sendbuf: *const c_void,
        sendcount: c_int,
        sendtype: mpi_sys::MPI_Datatype,
        recvbuf: *mut c_void,
        recvcount: c_int,
        recvtype: mpi_sys::MPI_Datatype,
        root: c_int,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn post_iscatter(
        &mut self,
        output: c_int,
        sendbuf: *const c_void,
        sendcount: c_int,
        sendtype: mpi_sys::MPI_Datatype,
        recvbuf: *mut c_void,
        recvcount: c_int,
        recvtype: mpi_sys::MPI_Datatype,
        root: c_int,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn pre_iscatterv(
        &mut self,
        sendbuf: *const c_void,
        sendcounts: *const c_int,
        displs: *const c_int,
        sendtype: mpi_sys::MPI_Datatype,
        recvbuf: *mut c_void,
        recvcount: c_int,
        recvtype: mpi_sys::MPI_Datatype,
        root: c_int,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn post_iscatterv(
        &mut self,
        output: c_int,
        sendbuf: *const c_void,
        sendcounts: *const c_int,
        displs: *const c_int,
        sendtype: mpi_sys::MPI_Datatype,
        recvbuf: *mut c_void,
        recvcount: c_int,
        recvtype: mpi_sys::MPI_Datatype,
        root: c_int,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn pre_isend(
        &mut self,
        buf: *const c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        dest: c_int,
        tag: c_int,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn post_isend(
        &mut self,
        output: c_int,
        buf: *const c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        dest: c_int,
        tag: c_int,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn pre_issend(
        &mut self,
        buf: *const c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        dest: c_int,
        tag: c_int,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn post_issend(
        &mut self,
        output: c_int,
        buf: *const c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        dest: c_int,
        tag: c_int,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn pre_keyval_create(
        &mut self,
        copy_fn: *mut mpi_sys::MPI_Copy_function,
        delete_fn: *mut mpi_sys::MPI_Delete_function,
        keyval: *mut c_int,
        extra_state: *mut c_void,
    ) {
    }
    #[inline]
    fn post_keyval_create(
        &mut self,
        output: c_int,
        copy_fn: *mut mpi_sys::MPI_Copy_function,
        delete_fn: *mut mpi_sys::MPI_Delete_function,
        keyval: *mut c_int,
        extra_state: *mut c_void,
    ) {
    }
    #[inline]
    fn pre_keyval_free(&mut self, keyval: *mut c_int) {}
    #[inline]
    fn post_keyval_free(&mut self, output: c_int, keyval: *mut c_int) {}
    #[inline]
    fn pre_lookup_name(
        &mut self,
        service_name: *const c_char,
        info: mpi_sys::MPI_Info,
        port_name: *mut c_char,
    ) {
    }
    #[inline]
    fn post_lookup_name(
        &mut self,
        output: c_int,
        service_name: *const c_char,
        info: mpi_sys::MPI_Info,
        port_name: *mut c_char,
    ) {
    }
    #[inline]
    fn pre_mprobe(
        &mut self,
        source: c_int,
        tag: c_int,
        comm: mpi_sys::MPI_Comm,
        message: *mut mpi_sys::MPI_Message,
        status: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn post_mprobe(
        &mut self,
        output: c_int,
        source: c_int,
        tag: c_int,
        comm: mpi_sys::MPI_Comm,
        message: *mut mpi_sys::MPI_Message,
        status: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn pre_mrecv(
        &mut self,
        buf: *mut c_void,
        count: c_int,
        r#type: mpi_sys::MPI_Datatype,
        message: *mut mpi_sys::MPI_Message,
        status: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn post_mrecv(
        &mut self,
        output: c_int,
        buf: *mut c_void,
        count: c_int,
        r#type: mpi_sys::MPI_Datatype,
        message: *mut mpi_sys::MPI_Message,
        status: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn pre_neighbor_allgather(
        &mut self,
        sendbuf: *const c_void,
        sendcount: c_int,
        sendtype: mpi_sys::MPI_Datatype,
        recvbuf: *mut c_void,
        recvcount: c_int,
        recvtype: mpi_sys::MPI_Datatype,
        comm: mpi_sys::MPI_Comm,
    ) {
    }
    #[inline]
    fn post_neighbor_allgather(
        &mut self,
        output: c_int,
        sendbuf: *const c_void,
        sendcount: c_int,
        sendtype: mpi_sys::MPI_Datatype,
        recvbuf: *mut c_void,
        recvcount: c_int,
        recvtype: mpi_sys::MPI_Datatype,
        comm: mpi_sys::MPI_Comm,
    ) {
    }
    #[inline]
    fn pre_neighbor_allgatherv(
        &mut self,
        sendbuf: *const c_void,
        sendcount: c_int,
        sendtype: mpi_sys::MPI_Datatype,
        recvbuf: *mut c_void,
        recvcounts: *const c_int,
        displs: *const c_int,
        recvtype: mpi_sys::MPI_Datatype,
        comm: mpi_sys::MPI_Comm,
    ) {
    }
    #[inline]
    fn post_neighbor_allgatherv(
        &mut self,
        output: c_int,
        sendbuf: *const c_void,
        sendcount: c_int,
        sendtype: mpi_sys::MPI_Datatype,
        recvbuf: *mut c_void,
        recvcounts: *const c_int,
        displs: *const c_int,
        recvtype: mpi_sys::MPI_Datatype,
        comm: mpi_sys::MPI_Comm,
    ) {
    }
    #[inline]
    fn pre_neighbor_alltoall(
        &mut self,
        sendbuf: *const c_void,
        sendcount: c_int,
        sendtype: mpi_sys::MPI_Datatype,
        recvbuf: *mut c_void,
        recvcount: c_int,
        recvtype: mpi_sys::MPI_Datatype,
        comm: mpi_sys::MPI_Comm,
    ) {
    }
    #[inline]
    fn post_neighbor_alltoall(
        &mut self,
        output: c_int,
        sendbuf: *const c_void,
        sendcount: c_int,
        sendtype: mpi_sys::MPI_Datatype,
        recvbuf: *mut c_void,
        recvcount: c_int,
        recvtype: mpi_sys::MPI_Datatype,
        comm: mpi_sys::MPI_Comm,
    ) {
    }
    #[inline]
    fn pre_neighbor_alltoallv(
        &mut self,
        sendbuf: *const c_void,
        sendcounts: *const c_int,
        sdispls: *const c_int,
        sendtype: mpi_sys::MPI_Datatype,
        recvbuf: *mut c_void,
        recvcounts: *const c_int,
        rdispls: *const c_int,
        recvtype: mpi_sys::MPI_Datatype,
        comm: mpi_sys::MPI_Comm,
    ) {
    }
    #[inline]
    fn post_neighbor_alltoallv(
        &mut self,
        output: c_int,
        sendbuf: *const c_void,
        sendcounts: *const c_int,
        sdispls: *const c_int,
        sendtype: mpi_sys::MPI_Datatype,
        recvbuf: *mut c_void,
        recvcounts: *const c_int,
        rdispls: *const c_int,
        recvtype: mpi_sys::MPI_Datatype,
        comm: mpi_sys::MPI_Comm,
    ) {
    }
    #[inline]
    fn pre_neighbor_alltoallw(
        &mut self,
        sendbuf: *const c_void,
        sendcounts: *const c_int,
        sdispls: *const mpi_sys::MPI_Aint,
        sendtypes: *const mpi_sys::MPI_Datatype,
        recvbuf: *mut c_void,
        recvcounts: *const c_int,
        rdispls: *const mpi_sys::MPI_Aint,
        recvtypes: *const mpi_sys::MPI_Datatype,
        comm: mpi_sys::MPI_Comm,
    ) {
    }
    #[inline]
    fn post_neighbor_alltoallw(
        &mut self,
        output: c_int,
        sendbuf: *const c_void,
        sendcounts: *const c_int,
        sdispls: *const mpi_sys::MPI_Aint,
        sendtypes: *const mpi_sys::MPI_Datatype,
        recvbuf: *mut c_void,
        recvcounts: *const c_int,
        rdispls: *const mpi_sys::MPI_Aint,
        recvtypes: *const mpi_sys::MPI_Datatype,
        comm: mpi_sys::MPI_Comm,
    ) {
    }
    #[inline]
    fn pre_op_commutative(&mut self, op: mpi_sys::MPI_Op, commute: *mut c_int) {}
    #[inline]
    fn post_op_commutative(&mut self, output: c_int, op: mpi_sys::MPI_Op, commute: *mut c_int) {}
    #[inline]
    fn pre_op_create(
        &mut self,
        function: *mut mpi_sys::MPI_User_function,
        commute: c_int,
        op: *mut mpi_sys::MPI_Op,
    ) {
    }
    #[inline]
    fn post_op_create(
        &mut self,
        output: c_int,
        function: *mut mpi_sys::MPI_User_function,
        commute: c_int,
        op: *mut mpi_sys::MPI_Op,
    ) {
    }
    #[inline]
    fn pre_op_free(&mut self, op: *mut mpi_sys::MPI_Op) {}
    #[inline]
    fn post_op_free(&mut self, output: c_int, op: *mut mpi_sys::MPI_Op) {}
    #[inline]
    fn pre_open_port(&mut self, info: mpi_sys::MPI_Info, port_name: *mut c_char) {}
    #[inline]
    fn post_open_port(&mut self, output: c_int, info: mpi_sys::MPI_Info, port_name: *mut c_char) {}
    #[inline]
    fn pre_pack(
        &mut self,
        inbuf: *const c_void,
        incount: c_int,
        datatype: mpi_sys::MPI_Datatype,
        outbuf: *mut c_void,
        outsize: c_int,
        position: *mut c_int,
        comm: mpi_sys::MPI_Comm,
    ) {
    }
    #[inline]
    fn post_pack(
        &mut self,
        output: c_int,
        inbuf: *const c_void,
        incount: c_int,
        datatype: mpi_sys::MPI_Datatype,
        outbuf: *mut c_void,
        outsize: c_int,
        position: *mut c_int,
        comm: mpi_sys::MPI_Comm,
    ) {
    }
    #[inline]
    fn pre_pack_external(
        &mut self,
        datarep: *const c_char,
        inbuf: *const c_void,
        incount: c_int,
        datatype: mpi_sys::MPI_Datatype,
        outbuf: *mut c_void,
        outsize: mpi_sys::MPI_Aint,
        position: *mut mpi_sys::MPI_Aint,
    ) {
    }
    #[inline]
    fn post_pack_external(
        &mut self,
        output: c_int,
        datarep: *const c_char,
        inbuf: *const c_void,
        incount: c_int,
        datatype: mpi_sys::MPI_Datatype,
        outbuf: *mut c_void,
        outsize: mpi_sys::MPI_Aint,
        position: *mut mpi_sys::MPI_Aint,
    ) {
    }
    #[inline]
    fn pre_pack_external_size(
        &mut self,
        datarep: *const c_char,
        incount: c_int,
        datatype: mpi_sys::MPI_Datatype,
        size: *mut mpi_sys::MPI_Aint,
    ) {
    }
    #[inline]
    fn post_pack_external_size(
        &mut self,
        output: c_int,
        datarep: *const c_char,
        incount: c_int,
        datatype: mpi_sys::MPI_Datatype,
        size: *mut mpi_sys::MPI_Aint,
    ) {
    }
    #[inline]
    fn pre_pack_size(
        &mut self,
        incount: c_int,
        datatype: mpi_sys::MPI_Datatype,
        comm: mpi_sys::MPI_Comm,
        size: *mut c_int,
    ) {
    }
    #[inline]
    fn post_pack_size(
        &mut self,
        output: c_int,
        incount: c_int,
        datatype: mpi_sys::MPI_Datatype,
        comm: mpi_sys::MPI_Comm,
        size: *mut c_int,
    ) {
    }
    #[inline]
    fn pre_pcontrol(&mut self, level: c_int) {}
    #[inline]
    fn post_pcontrol(&mut self, output: c_int, level: c_int) {}
    #[inline]
    fn pre_probe(
        &mut self,
        source: c_int,
        tag: c_int,
        comm: mpi_sys::MPI_Comm,
        status: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn post_probe(
        &mut self,
        output: c_int,
        source: c_int,
        tag: c_int,
        comm: mpi_sys::MPI_Comm,
        status: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn pre_publish_name(
        &mut self,
        service_name: *const c_char,
        info: mpi_sys::MPI_Info,
        port_name: *const c_char,
    ) {
    }
    #[inline]
    fn post_publish_name(
        &mut self,
        output: c_int,
        service_name: *const c_char,
        info: mpi_sys::MPI_Info,
        port_name: *const c_char,
    ) {
    }
    #[inline]
    fn pre_put(
        &mut self,
        origin_addr: *const c_void,
        origin_count: c_int,
        origin_datatype: mpi_sys::MPI_Datatype,
        target_rank: c_int,
        target_disp: mpi_sys::MPI_Aint,
        target_count: c_int,
        target_datatype: mpi_sys::MPI_Datatype,
        win: mpi_sys::MPI_Win,
    ) {
    }
    #[inline]
    fn post_put(
        &mut self,
        output: c_int,
        origin_addr: *const c_void,
        origin_count: c_int,
        origin_datatype: mpi_sys::MPI_Datatype,
        target_rank: c_int,
        target_disp: mpi_sys::MPI_Aint,
        target_count: c_int,
        target_datatype: mpi_sys::MPI_Datatype,
        win: mpi_sys::MPI_Win,
    ) {
    }
    #[inline]
    fn pre_query_thread(&mut self, provided: *mut c_int) {}
    #[inline]
    fn post_query_thread(&mut self, output: c_int, provided: *mut c_int) {}
    #[inline]
    fn pre_raccumulate(
        &mut self,
        origin_addr: *const c_void,
        origin_count: c_int,
        origin_datatype: mpi_sys::MPI_Datatype,
        target_rank: c_int,
        target_disp: mpi_sys::MPI_Aint,
        target_count: c_int,
        target_datatype: mpi_sys::MPI_Datatype,
        op: mpi_sys::MPI_Op,
        win: mpi_sys::MPI_Win,
        request: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn post_raccumulate(
        &mut self,
        output: c_int,
        origin_addr: *const c_void,
        origin_count: c_int,
        origin_datatype: mpi_sys::MPI_Datatype,
        target_rank: c_int,
        target_disp: mpi_sys::MPI_Aint,
        target_count: c_int,
        target_datatype: mpi_sys::MPI_Datatype,
        op: mpi_sys::MPI_Op,
        win: mpi_sys::MPI_Win,
        request: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn pre_recv(
        &mut self,
        buf: *mut c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        source: c_int,
        tag: c_int,
        comm: mpi_sys::MPI_Comm,
        status: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn post_recv(
        &mut self,
        output: c_int,
        buf: *mut c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        source: c_int,
        tag: c_int,
        comm: mpi_sys::MPI_Comm,
        status: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn pre_recv_init(
        &mut self,
        buf: *mut c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        source: c_int,
        tag: c_int,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn post_recv_init(
        &mut self,
        output: c_int,
        buf: *mut c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        source: c_int,
        tag: c_int,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn pre_reduce(
        &mut self,
        sendbuf: *const c_void,
        recvbuf: *mut c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        op: mpi_sys::MPI_Op,
        root: c_int,
        comm: mpi_sys::MPI_Comm,
    ) {
    }
    #[inline]
    fn post_reduce(
        &mut self,
        output: c_int,
        sendbuf: *const c_void,
        recvbuf: *mut c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        op: mpi_sys::MPI_Op,
        root: c_int,
        comm: mpi_sys::MPI_Comm,
    ) {
    }
    #[inline]
    fn pre_reduce_local(
        &mut self,
        inbuf: *const c_void,
        inoutbuf: *mut c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        op: mpi_sys::MPI_Op,
    ) {
    }
    #[inline]
    fn post_reduce_local(
        &mut self,
        output: c_int,
        inbuf: *const c_void,
        inoutbuf: *mut c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        op: mpi_sys::MPI_Op,
    ) {
    }
    #[inline]
    fn pre_reduce_scatter(
        &mut self,
        sendbuf: *const c_void,
        recvbuf: *mut c_void,
        recvcounts: *const c_int,
        datatype: mpi_sys::MPI_Datatype,
        op: mpi_sys::MPI_Op,
        comm: mpi_sys::MPI_Comm,
    ) {
    }
    #[inline]
    fn post_reduce_scatter(
        &mut self,
        output: c_int,
        sendbuf: *const c_void,
        recvbuf: *mut c_void,
        recvcounts: *const c_int,
        datatype: mpi_sys::MPI_Datatype,
        op: mpi_sys::MPI_Op,
        comm: mpi_sys::MPI_Comm,
    ) {
    }
    #[inline]
    fn pre_reduce_scatter_block(
        &mut self,
        sendbuf: *const c_void,
        recvbuf: *mut c_void,
        recvcount: c_int,
        datatype: mpi_sys::MPI_Datatype,
        op: mpi_sys::MPI_Op,
        comm: mpi_sys::MPI_Comm,
    ) {
    }
    #[inline]
    fn post_reduce_scatter_block(
        &mut self,
        output: c_int,
        sendbuf: *const c_void,
        recvbuf: *mut c_void,
        recvcount: c_int,
        datatype: mpi_sys::MPI_Datatype,
        op: mpi_sys::MPI_Op,
        comm: mpi_sys::MPI_Comm,
    ) {
    }
    #[inline]
    fn pre_register_datarep(
        &mut self,
        datarep: *const c_char,
        read_conversion_fn: *mut mpi_sys::MPI_Datarep_conversion_function,
        write_conversion_fn: *mut mpi_sys::MPI_Datarep_conversion_function,
        dtype_file_extent_fn: *mut mpi_sys::MPI_Datarep_extent_function,
        extra_state: *mut c_void,
    ) {
    }
    #[inline]
    fn post_register_datarep(
        &mut self,
        output: c_int,
        datarep: *const c_char,
        read_conversion_fn: *mut mpi_sys::MPI_Datarep_conversion_function,
        write_conversion_fn: *mut mpi_sys::MPI_Datarep_conversion_function,
        dtype_file_extent_fn: *mut mpi_sys::MPI_Datarep_extent_function,
        extra_state: *mut c_void,
    ) {
    }
    #[inline]
    fn pre_request_free(&mut self, request: *mut mpi_sys::MPI_Request) {}
    #[inline]
    fn post_request_free(&mut self, output: c_int, request: *mut mpi_sys::MPI_Request) {}
    #[inline]
    fn pre_request_get_status(
        &mut self,
        request: mpi_sys::MPI_Request,
        flag: *mut c_int,
        status: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn post_request_get_status(
        &mut self,
        output: c_int,
        request: mpi_sys::MPI_Request,
        flag: *mut c_int,
        status: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn pre_rget(
        &mut self,
        origin_addr: *mut c_void,
        origin_count: c_int,
        origin_datatype: mpi_sys::MPI_Datatype,
        target_rank: c_int,
        target_disp: mpi_sys::MPI_Aint,
        target_count: c_int,
        target_datatype: mpi_sys::MPI_Datatype,
        win: mpi_sys::MPI_Win,
        request: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn post_rget(
        &mut self,
        output: c_int,
        origin_addr: *mut c_void,
        origin_count: c_int,
        origin_datatype: mpi_sys::MPI_Datatype,
        target_rank: c_int,
        target_disp: mpi_sys::MPI_Aint,
        target_count: c_int,
        target_datatype: mpi_sys::MPI_Datatype,
        win: mpi_sys::MPI_Win,
        request: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn pre_rget_accumulate(
        &mut self,
        origin_addr: *const c_void,
        origin_count: c_int,
        origin_datatype: mpi_sys::MPI_Datatype,
        result_addr: *mut c_void,
        result_count: c_int,
        result_datatype: mpi_sys::MPI_Datatype,
        target_rank: c_int,
        target_disp: mpi_sys::MPI_Aint,
        target_count: c_int,
        target_datatype: mpi_sys::MPI_Datatype,
        op: mpi_sys::MPI_Op,
        win: mpi_sys::MPI_Win,
        request: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn post_rget_accumulate(
        &mut self,
        output: c_int,
        origin_addr: *const c_void,
        origin_count: c_int,
        origin_datatype: mpi_sys::MPI_Datatype,
        result_addr: *mut c_void,
        result_count: c_int,
        result_datatype: mpi_sys::MPI_Datatype,
        target_rank: c_int,
        target_disp: mpi_sys::MPI_Aint,
        target_count: c_int,
        target_datatype: mpi_sys::MPI_Datatype,
        op: mpi_sys::MPI_Op,
        win: mpi_sys::MPI_Win,
        request: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn pre_rput(
        &mut self,
        origin_addr: *const c_void,
        origin_count: c_int,
        origin_datatype: mpi_sys::MPI_Datatype,
        target_rank: c_int,
        target_disp: mpi_sys::MPI_Aint,
        target_cout: c_int,
        target_datatype: mpi_sys::MPI_Datatype,
        win: mpi_sys::MPI_Win,
        request: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn post_rput(
        &mut self,
        output: c_int,
        origin_addr: *const c_void,
        origin_count: c_int,
        origin_datatype: mpi_sys::MPI_Datatype,
        target_rank: c_int,
        target_disp: mpi_sys::MPI_Aint,
        target_cout: c_int,
        target_datatype: mpi_sys::MPI_Datatype,
        win: mpi_sys::MPI_Win,
        request: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn pre_rsend(
        &mut self,
        ibuf: *const c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        dest: c_int,
        tag: c_int,
        comm: mpi_sys::MPI_Comm,
    ) {
    }
    #[inline]
    fn post_rsend(
        &mut self,
        output: c_int,
        ibuf: *const c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        dest: c_int,
        tag: c_int,
        comm: mpi_sys::MPI_Comm,
    ) {
    }
    #[inline]
    fn pre_rsend_init(
        &mut self,
        buf: *const c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        dest: c_int,
        tag: c_int,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn post_rsend_init(
        &mut self,
        output: c_int,
        buf: *const c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        dest: c_int,
        tag: c_int,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn pre_scan(
        &mut self,
        sendbuf: *const c_void,
        recvbuf: *mut c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        op: mpi_sys::MPI_Op,
        comm: mpi_sys::MPI_Comm,
    ) {
    }
    #[inline]
    fn post_scan(
        &mut self,
        output: c_int,
        sendbuf: *const c_void,
        recvbuf: *mut c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        op: mpi_sys::MPI_Op,
        comm: mpi_sys::MPI_Comm,
    ) {
    }
    #[inline]
    fn pre_scatter(
        &mut self,
        sendbuf: *const c_void,
        sendcount: c_int,
        sendtype: mpi_sys::MPI_Datatype,
        recvbuf: *mut c_void,
        recvcount: c_int,
        recvtype: mpi_sys::MPI_Datatype,
        root: c_int,
        comm: mpi_sys::MPI_Comm,
    ) {
    }
    #[inline]
    fn post_scatter(
        &mut self,
        output: c_int,
        sendbuf: *const c_void,
        sendcount: c_int,
        sendtype: mpi_sys::MPI_Datatype,
        recvbuf: *mut c_void,
        recvcount: c_int,
        recvtype: mpi_sys::MPI_Datatype,
        root: c_int,
        comm: mpi_sys::MPI_Comm,
    ) {
    }
    #[inline]
    fn pre_scatterv(
        &mut self,
        sendbuf: *const c_void,
        sendcounts: *const c_int,
        displs: *const c_int,
        sendtype: mpi_sys::MPI_Datatype,
        recvbuf: *mut c_void,
        recvcount: c_int,
        recvtype: mpi_sys::MPI_Datatype,
        root: c_int,
        comm: mpi_sys::MPI_Comm,
    ) {
    }
    #[inline]
    fn post_scatterv(
        &mut self,
        output: c_int,
        sendbuf: *const c_void,
        sendcounts: *const c_int,
        displs: *const c_int,
        sendtype: mpi_sys::MPI_Datatype,
        recvbuf: *mut c_void,
        recvcount: c_int,
        recvtype: mpi_sys::MPI_Datatype,
        root: c_int,
        comm: mpi_sys::MPI_Comm,
    ) {
    }
    #[inline]
    fn pre_send(
        &mut self,
        buf: *const c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        dest: c_int,
        tag: c_int,
        comm: mpi_sys::MPI_Comm,
    ) {
    }
    #[inline]
    fn post_send(
        &mut self,
        output: c_int,
        buf: *const c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        dest: c_int,
        tag: c_int,
        comm: mpi_sys::MPI_Comm,
    ) {
    }
    #[inline]
    fn pre_send_init(
        &mut self,
        buf: *const c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        dest: c_int,
        tag: c_int,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn post_send_init(
        &mut self,
        output: c_int,
        buf: *const c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        dest: c_int,
        tag: c_int,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn pre_sendrecv(
        &mut self,
        sendbuf: *const c_void,
        sendcount: c_int,
        sendtype: mpi_sys::MPI_Datatype,
        dest: c_int,
        sendtag: c_int,
        recvbuf: *mut c_void,
        recvcount: c_int,
        recvtype: mpi_sys::MPI_Datatype,
        source: c_int,
        recvtag: c_int,
        comm: mpi_sys::MPI_Comm,
        status: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn post_sendrecv(
        &mut self,
        output: c_int,
        sendbuf: *const c_void,
        sendcount: c_int,
        sendtype: mpi_sys::MPI_Datatype,
        dest: c_int,
        sendtag: c_int,
        recvbuf: *mut c_void,
        recvcount: c_int,
        recvtype: mpi_sys::MPI_Datatype,
        source: c_int,
        recvtag: c_int,
        comm: mpi_sys::MPI_Comm,
        status: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn pre_sendrecv_replace(
        &mut self,
        buf: *mut c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        dest: c_int,
        sendtag: c_int,
        source: c_int,
        recvtag: c_int,
        comm: mpi_sys::MPI_Comm,
        status: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn post_sendrecv_replace(
        &mut self,
        output: c_int,
        buf: *mut c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        dest: c_int,
        sendtag: c_int,
        source: c_int,
        recvtag: c_int,
        comm: mpi_sys::MPI_Comm,
        status: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn pre_ssend(
        &mut self,
        buf: *const c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        dest: c_int,
        tag: c_int,
        comm: mpi_sys::MPI_Comm,
    ) {
    }
    #[inline]
    fn post_ssend(
        &mut self,
        output: c_int,
        buf: *const c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        dest: c_int,
        tag: c_int,
        comm: mpi_sys::MPI_Comm,
    ) {
    }
    #[inline]
    fn pre_ssend_init(
        &mut self,
        buf: *const c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        dest: c_int,
        tag: c_int,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn post_ssend_init(
        &mut self,
        output: c_int,
        buf: *const c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        dest: c_int,
        tag: c_int,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn pre_start(&mut self, request: *mut mpi_sys::MPI_Request) {}
    #[inline]
    fn post_start(&mut self, output: c_int, request: *mut mpi_sys::MPI_Request) {}
    #[inline]
    fn pre_startall(&mut self, count: c_int, array_of_requests: *mut mpi_sys::MPI_Request) {}
    #[inline]
    fn post_startall(
        &mut self,
        output: c_int,
        count: c_int,
        array_of_requests: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn pre_status_set_cancelled(&mut self, status: *mut mpi_sys::MPI_Status, flag: c_int) {}
    #[inline]
    fn post_status_set_cancelled(
        &mut self,
        output: c_int,
        status: *mut mpi_sys::MPI_Status,
        flag: c_int,
    ) {
    }
    #[inline]
    fn pre_status_set_elements(
        &mut self,
        status: *mut mpi_sys::MPI_Status,
        datatype: mpi_sys::MPI_Datatype,
        count: c_int,
    ) {
    }
    #[inline]
    fn post_status_set_elements(
        &mut self,
        output: c_int,
        status: *mut mpi_sys::MPI_Status,
        datatype: mpi_sys::MPI_Datatype,
        count: c_int,
    ) {
    }
    #[inline]
    fn pre_status_set_elements_x(
        &mut self,
        status: *mut mpi_sys::MPI_Status,
        datatype: mpi_sys::MPI_Datatype,
        count: mpi_sys::MPI_Count,
    ) {
    }
    #[inline]
    fn post_status_set_elements_x(
        &mut self,
        output: c_int,
        status: *mut mpi_sys::MPI_Status,
        datatype: mpi_sys::MPI_Datatype,
        count: mpi_sys::MPI_Count,
    ) {
    }
    #[inline]
    fn pre_test(
        &mut self,
        request: *mut mpi_sys::MPI_Request,
        flag: *mut c_int,
        status: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn post_test(
        &mut self,
        output: c_int,
        request: *mut mpi_sys::MPI_Request,
        flag: *mut c_int,
        status: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn pre_test_cancelled(&mut self, status: *const mpi_sys::MPI_Status, flag: *mut c_int) {}
    #[inline]
    fn post_test_cancelled(
        &mut self,
        output: c_int,
        status: *const mpi_sys::MPI_Status,
        flag: *mut c_int,
    ) {
    }
    #[inline]
    fn pre_testall(
        &mut self,
        count: c_int,
        array_of_requests: *mut mpi_sys::MPI_Request,
        flag: *mut c_int,
        array_of_statuses: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn post_testall(
        &mut self,
        output: c_int,
        count: c_int,
        array_of_requests: *mut mpi_sys::MPI_Request,
        flag: *mut c_int,
        array_of_statuses: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn pre_testany(
        &mut self,
        count: c_int,
        array_of_requests: *mut mpi_sys::MPI_Request,
        index: *mut c_int,
        flag: *mut c_int,
        status: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn post_testany(
        &mut self,
        output: c_int,
        count: c_int,
        array_of_requests: *mut mpi_sys::MPI_Request,
        index: *mut c_int,
        flag: *mut c_int,
        status: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn pre_testsome(
        &mut self,
        incount: c_int,
        array_of_requests: *mut mpi_sys::MPI_Request,
        outcount: *mut c_int,
        array_of_indices: *mut c_int,
        array_of_statuses: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn post_testsome(
        &mut self,
        output: c_int,
        incount: c_int,
        array_of_requests: *mut mpi_sys::MPI_Request,
        outcount: *mut c_int,
        array_of_indices: *mut c_int,
        array_of_statuses: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn pre_topo_test(&mut self, comm: mpi_sys::MPI_Comm, status: *mut c_int) {}
    #[inline]
    fn post_topo_test(&mut self, output: c_int, comm: mpi_sys::MPI_Comm, status: *mut c_int) {}
    #[inline]
    fn pre_type_commit(&mut self, r#type: *mut mpi_sys::MPI_Datatype) {}
    #[inline]
    fn post_type_commit(&mut self, output: c_int, r#type: *mut mpi_sys::MPI_Datatype) {}
    #[inline]
    fn pre_type_contiguous(
        &mut self,
        count: c_int,
        oldtype: mpi_sys::MPI_Datatype,
        newtype: *mut mpi_sys::MPI_Datatype,
    ) {
    }
    #[inline]
    fn post_type_contiguous(
        &mut self,
        output: c_int,
        count: c_int,
        oldtype: mpi_sys::MPI_Datatype,
        newtype: *mut mpi_sys::MPI_Datatype,
    ) {
    }
    #[inline]
    fn pre_type_create_darray(
        &mut self,
        size: c_int,
        rank: c_int,
        ndims: c_int,
        gsize_array: *const c_int,
        distrib_array: *const c_int,
        darg_array: *const c_int,
        psize_array: *const c_int,
        order: c_int,
        oldtype: mpi_sys::MPI_Datatype,
        newtype: *mut mpi_sys::MPI_Datatype,
    ) {
    }
    #[inline]
    fn post_type_create_darray(
        &mut self,
        output: c_int,
        size: c_int,
        rank: c_int,
        ndims: c_int,
        gsize_array: *const c_int,
        distrib_array: *const c_int,
        darg_array: *const c_int,
        psize_array: *const c_int,
        order: c_int,
        oldtype: mpi_sys::MPI_Datatype,
        newtype: *mut mpi_sys::MPI_Datatype,
    ) {
    }
    #[inline]
    fn pre_type_create_f90_complex(
        &mut self,
        p: c_int,
        r: c_int,
        newtype: *mut mpi_sys::MPI_Datatype,
    ) {
    }
    #[inline]
    fn post_type_create_f90_complex(
        &mut self,
        output: c_int,
        p: c_int,
        r: c_int,
        newtype: *mut mpi_sys::MPI_Datatype,
    ) {
    }
    #[inline]
    fn pre_type_create_f90_integer(&mut self, r: c_int, newtype: *mut mpi_sys::MPI_Datatype) {}
    #[inline]
    fn post_type_create_f90_integer(
        &mut self,
        output: c_int,
        r: c_int,
        newtype: *mut mpi_sys::MPI_Datatype,
    ) {
    }
    #[inline]
    fn pre_type_create_f90_real(
        &mut self,
        p: c_int,
        r: c_int,
        newtype: *mut mpi_sys::MPI_Datatype,
    ) {
    }
    #[inline]
    fn post_type_create_f90_real(
        &mut self,
        output: c_int,
        p: c_int,
        r: c_int,
        newtype: *mut mpi_sys::MPI_Datatype,
    ) {
    }
    #[inline]
    fn pre_type_create_hindexed(
        &mut self,
        count: c_int,
        array_of_blocklengths: *const c_int,
        array_of_displacements: *const mpi_sys::MPI_Aint,
        oldtype: mpi_sys::MPI_Datatype,
        newtype: *mut mpi_sys::MPI_Datatype,
    ) {
    }
    #[inline]
    fn post_type_create_hindexed(
        &mut self,
        output: c_int,
        count: c_int,
        array_of_blocklengths: *const c_int,
        array_of_displacements: *const mpi_sys::MPI_Aint,
        oldtype: mpi_sys::MPI_Datatype,
        newtype: *mut mpi_sys::MPI_Datatype,
    ) {
    }
    #[inline]
    fn pre_type_create_hindexed_block(
        &mut self,
        count: c_int,
        blocklength: c_int,
        array_of_displacements: *const mpi_sys::MPI_Aint,
        oldtype: mpi_sys::MPI_Datatype,
        newtype: *mut mpi_sys::MPI_Datatype,
    ) {
    }
    #[inline]
    fn post_type_create_hindexed_block(
        &mut self,
        output: c_int,
        count: c_int,
        blocklength: c_int,
        array_of_displacements: *const mpi_sys::MPI_Aint,
        oldtype: mpi_sys::MPI_Datatype,
        newtype: *mut mpi_sys::MPI_Datatype,
    ) {
    }
    #[inline]
    fn pre_type_create_hvector(
        &mut self,
        count: c_int,
        blocklength: c_int,
        stride: mpi_sys::MPI_Aint,
        oldtype: mpi_sys::MPI_Datatype,
        newtype: *mut mpi_sys::MPI_Datatype,
    ) {
    }
    #[inline]
    fn post_type_create_hvector(
        &mut self,
        output: c_int,
        count: c_int,
        blocklength: c_int,
        stride: mpi_sys::MPI_Aint,
        oldtype: mpi_sys::MPI_Datatype,
        newtype: *mut mpi_sys::MPI_Datatype,
    ) {
    }
    #[inline]
    fn pre_type_create_indexed_block(
        &mut self,
        count: c_int,
        blocklength: c_int,
        array_of_displacements: *const c_int,
        oldtype: mpi_sys::MPI_Datatype,
        newtype: *mut mpi_sys::MPI_Datatype,
    ) {
    }
    #[inline]
    fn post_type_create_indexed_block(
        &mut self,
        output: c_int,
        count: c_int,
        blocklength: c_int,
        array_of_displacements: *const c_int,
        oldtype: mpi_sys::MPI_Datatype,
        newtype: *mut mpi_sys::MPI_Datatype,
    ) {
    }
    #[inline]
    fn pre_type_create_keyval(
        &mut self,
        type_copy_attr_fn: *mut mpi_sys::MPI_Type_copy_attr_function,
        type_delete_attr_fn: *mut mpi_sys::MPI_Type_delete_attr_function,
        type_keyval: *mut c_int,
        extra_state: *mut c_void,
    ) {
    }
    #[inline]
    fn post_type_create_keyval(
        &mut self,
        output: c_int,
        type_copy_attr_fn: *mut mpi_sys::MPI_Type_copy_attr_function,
        type_delete_attr_fn: *mut mpi_sys::MPI_Type_delete_attr_function,
        type_keyval: *mut c_int,
        extra_state: *mut c_void,
    ) {
    }
    #[inline]
    fn pre_type_create_resized(
        &mut self,
        oldtype: mpi_sys::MPI_Datatype,
        lb: mpi_sys::MPI_Aint,
        extent: mpi_sys::MPI_Aint,
        newtype: *mut mpi_sys::MPI_Datatype,
    ) {
    }
    #[inline]
    fn post_type_create_resized(
        &mut self,
        output: c_int,
        oldtype: mpi_sys::MPI_Datatype,
        lb: mpi_sys::MPI_Aint,
        extent: mpi_sys::MPI_Aint,
        newtype: *mut mpi_sys::MPI_Datatype,
    ) {
    }
    #[inline]
    fn pre_type_create_struct(
        &mut self,
        count: c_int,
        array_of_block_lengths: *const c_int,
        array_of_displacements: *const mpi_sys::MPI_Aint,
        array_of_types: *const mpi_sys::MPI_Datatype,
        newtype: *mut mpi_sys::MPI_Datatype,
    ) {
    }
    #[inline]
    fn post_type_create_struct(
        &mut self,
        output: c_int,
        count: c_int,
        array_of_block_lengths: *const c_int,
        array_of_displacements: *const mpi_sys::MPI_Aint,
        array_of_types: *const mpi_sys::MPI_Datatype,
        newtype: *mut mpi_sys::MPI_Datatype,
    ) {
    }
    #[inline]
    fn pre_type_create_subarray(
        &mut self,
        ndims: c_int,
        size_array: *const c_int,
        subsize_array: *const c_int,
        start_array: *const c_int,
        order: c_int,
        oldtype: mpi_sys::MPI_Datatype,
        newtype: *mut mpi_sys::MPI_Datatype,
    ) {
    }
    #[inline]
    fn post_type_create_subarray(
        &mut self,
        output: c_int,
        ndims: c_int,
        size_array: *const c_int,
        subsize_array: *const c_int,
        start_array: *const c_int,
        order: c_int,
        oldtype: mpi_sys::MPI_Datatype,
        newtype: *mut mpi_sys::MPI_Datatype,
    ) {
    }
    #[inline]
    fn pre_type_delete_attr(&mut self, r#type: mpi_sys::MPI_Datatype, type_keyval: c_int) {}
    #[inline]
    fn post_type_delete_attr(
        &mut self,
        output: c_int,
        r#type: mpi_sys::MPI_Datatype,
        type_keyval: c_int,
    ) {
    }
    #[inline]
    fn pre_type_dup(&mut self, r#type: mpi_sys::MPI_Datatype, newtype: *mut mpi_sys::MPI_Datatype) {
    }
    #[inline]
    fn post_type_dup(
        &mut self,
        output: c_int,
        r#type: mpi_sys::MPI_Datatype,
        newtype: *mut mpi_sys::MPI_Datatype,
    ) {
    }
    #[inline]
    fn pre_type_extent(&mut self, r#type: mpi_sys::MPI_Datatype, extent: *mut mpi_sys::MPI_Aint) {}
    #[inline]
    fn post_type_extent(
        &mut self,
        output: c_int,
        r#type: mpi_sys::MPI_Datatype,
        extent: *mut mpi_sys::MPI_Aint,
    ) {
    }
    #[inline]
    fn pre_type_free(&mut self, r#type: *mut mpi_sys::MPI_Datatype) {}
    #[inline]
    fn post_type_free(&mut self, output: c_int, r#type: *mut mpi_sys::MPI_Datatype) {}
    #[inline]
    fn pre_type_free_keyval(&mut self, type_keyval: *mut c_int) {}
    #[inline]
    fn post_type_free_keyval(&mut self, output: c_int, type_keyval: *mut c_int) {}
    #[inline]
    fn pre_type_get_attr(
        &mut self,
        r#type: mpi_sys::MPI_Datatype,
        type_keyval: c_int,
        attribute_val: *mut c_void,
        flag: *mut c_int,
    ) {
    }
    #[inline]
    fn post_type_get_attr(
        &mut self,
        output: c_int,
        r#type: mpi_sys::MPI_Datatype,
        type_keyval: c_int,
        attribute_val: *mut c_void,
        flag: *mut c_int,
    ) {
    }
    #[inline]
    fn pre_type_get_contents(
        &mut self,
        mtype: mpi_sys::MPI_Datatype,
        max_integers: c_int,
        max_addresses: c_int,
        max_datatypes: c_int,
        array_of_integers: *mut c_int,
        array_of_addresses: *mut mpi_sys::MPI_Aint,
        array_of_datatypes: *mut mpi_sys::MPI_Datatype,
    ) {
    }
    #[inline]
    fn post_type_get_contents(
        &mut self,
        output: c_int,
        mtype: mpi_sys::MPI_Datatype,
        max_integers: c_int,
        max_addresses: c_int,
        max_datatypes: c_int,
        array_of_integers: *mut c_int,
        array_of_addresses: *mut mpi_sys::MPI_Aint,
        array_of_datatypes: *mut mpi_sys::MPI_Datatype,
    ) {
    }
    #[inline]
    fn pre_type_get_envelope(
        &mut self,
        r#type: mpi_sys::MPI_Datatype,
        num_integers: *mut c_int,
        num_addresses: *mut c_int,
        num_datatypes: *mut c_int,
        combiner: *mut c_int,
    ) {
    }
    #[inline]
    fn post_type_get_envelope(
        &mut self,
        output: c_int,
        r#type: mpi_sys::MPI_Datatype,
        num_integers: *mut c_int,
        num_addresses: *mut c_int,
        num_datatypes: *mut c_int,
        combiner: *mut c_int,
    ) {
    }
    #[inline]
    fn pre_type_get_extent(
        &mut self,
        r#type: mpi_sys::MPI_Datatype,
        lb: *mut mpi_sys::MPI_Aint,
        extent: *mut mpi_sys::MPI_Aint,
    ) {
    }
    #[inline]
    fn post_type_get_extent(
        &mut self,
        output: c_int,
        r#type: mpi_sys::MPI_Datatype,
        lb: *mut mpi_sys::MPI_Aint,
        extent: *mut mpi_sys::MPI_Aint,
    ) {
    }
    #[inline]
    fn pre_type_get_extent_x(
        &mut self,
        r#type: mpi_sys::MPI_Datatype,
        lb: *mut mpi_sys::MPI_Count,
        extent: *mut mpi_sys::MPI_Count,
    ) {
    }
    #[inline]
    fn post_type_get_extent_x(
        &mut self,
        output: c_int,
        r#type: mpi_sys::MPI_Datatype,
        lb: *mut mpi_sys::MPI_Count,
        extent: *mut mpi_sys::MPI_Count,
    ) {
    }
    #[inline]
    fn pre_type_get_name(
        &mut self,
        r#type: mpi_sys::MPI_Datatype,
        type_name: *mut c_char,
        resultlen: *mut c_int,
    ) {
    }
    #[inline]
    fn post_type_get_name(
        &mut self,
        output: c_int,
        r#type: mpi_sys::MPI_Datatype,
        type_name: *mut c_char,
        resultlen: *mut c_int,
    ) {
    }
    #[inline]
    fn pre_type_get_true_extent(
        &mut self,
        datatype: mpi_sys::MPI_Datatype,
        true_lb: *mut mpi_sys::MPI_Aint,
        true_extent: *mut mpi_sys::MPI_Aint,
    ) {
    }
    #[inline]
    fn post_type_get_true_extent(
        &mut self,
        output: c_int,
        datatype: mpi_sys::MPI_Datatype,
        true_lb: *mut mpi_sys::MPI_Aint,
        true_extent: *mut mpi_sys::MPI_Aint,
    ) {
    }
    #[inline]
    fn pre_type_get_true_extent_x(
        &mut self,
        datatype: mpi_sys::MPI_Datatype,
        true_lb: *mut mpi_sys::MPI_Count,
        true_extent: *mut mpi_sys::MPI_Count,
    ) {
    }
    #[inline]
    fn post_type_get_true_extent_x(
        &mut self,
        output: c_int,
        datatype: mpi_sys::MPI_Datatype,
        true_lb: *mut mpi_sys::MPI_Count,
        true_extent: *mut mpi_sys::MPI_Count,
    ) {
    }
    #[inline]
    fn pre_type_hindexed(
        &mut self,
        count: c_int,
        array_of_blocklengths: *mut c_int,
        array_of_displacements: *mut mpi_sys::MPI_Aint,
        oldtype: mpi_sys::MPI_Datatype,
        newtype: *mut mpi_sys::MPI_Datatype,
    ) {
    }
    #[inline]
    fn post_type_hindexed(
        &mut self,
        output: c_int,
        count: c_int,
        array_of_blocklengths: *mut c_int,
        array_of_displacements: *mut mpi_sys::MPI_Aint,
        oldtype: mpi_sys::MPI_Datatype,
        newtype: *mut mpi_sys::MPI_Datatype,
    ) {
    }
    #[inline]
    fn pre_type_hvector(
        &mut self,
        count: c_int,
        blocklength: c_int,
        stride: mpi_sys::MPI_Aint,
        oldtype: mpi_sys::MPI_Datatype,
        newtype: *mut mpi_sys::MPI_Datatype,
    ) {
    }
    #[inline]
    fn post_type_hvector(
        &mut self,
        output: c_int,
        count: c_int,
        blocklength: c_int,
        stride: mpi_sys::MPI_Aint,
        oldtype: mpi_sys::MPI_Datatype,
        newtype: *mut mpi_sys::MPI_Datatype,
    ) {
    }
    #[inline]
    fn pre_type_indexed(
        &mut self,
        count: c_int,
        array_of_blocklengths: *const c_int,
        array_of_displacements: *const c_int,
        oldtype: mpi_sys::MPI_Datatype,
        newtype: *mut mpi_sys::MPI_Datatype,
    ) {
    }
    #[inline]
    fn post_type_indexed(
        &mut self,
        output: c_int,
        count: c_int,
        array_of_blocklengths: *const c_int,
        array_of_displacements: *const c_int,
        oldtype: mpi_sys::MPI_Datatype,
        newtype: *mut mpi_sys::MPI_Datatype,
    ) {
    }
    #[inline]
    fn pre_type_lb(&mut self, r#type: mpi_sys::MPI_Datatype, lb: *mut mpi_sys::MPI_Aint) {}
    #[inline]
    fn post_type_lb(
        &mut self,
        output: c_int,
        r#type: mpi_sys::MPI_Datatype,
        lb: *mut mpi_sys::MPI_Aint,
    ) {
    }
    #[inline]
    fn pre_type_match_size(
        &mut self,
        typeclass: c_int,
        size: c_int,
        r#type: *mut mpi_sys::MPI_Datatype,
    ) {
    }
    #[inline]
    fn post_type_match_size(
        &mut self,
        output: c_int,
        typeclass: c_int,
        size: c_int,
        r#type: *mut mpi_sys::MPI_Datatype,
    ) {
    }
    #[inline]
    fn pre_type_set_attr(
        &mut self,
        r#type: mpi_sys::MPI_Datatype,
        type_keyval: c_int,
        attr_val: *mut c_void,
    ) {
    }
    #[inline]
    fn post_type_set_attr(
        &mut self,
        output: c_int,
        r#type: mpi_sys::MPI_Datatype,
        type_keyval: c_int,
        attr_val: *mut c_void,
    ) {
    }
    #[inline]
    fn pre_type_set_name(&mut self, r#type: mpi_sys::MPI_Datatype, type_name: *const c_char) {}
    #[inline]
    fn post_type_set_name(
        &mut self,
        output: c_int,
        r#type: mpi_sys::MPI_Datatype,
        type_name: *const c_char,
    ) {
    }
    #[inline]
    fn pre_type_size(&mut self, r#type: mpi_sys::MPI_Datatype, size: *mut c_int) {}
    #[inline]
    fn post_type_size(&mut self, output: c_int, r#type: mpi_sys::MPI_Datatype, size: *mut c_int) {}
    #[inline]
    fn pre_type_size_x(&mut self, r#type: mpi_sys::MPI_Datatype, size: *mut mpi_sys::MPI_Count) {}
    #[inline]
    fn post_type_size_x(
        &mut self,
        output: c_int,
        r#type: mpi_sys::MPI_Datatype,
        size: *mut mpi_sys::MPI_Count,
    ) {
    }
    #[inline]
    fn pre_type_struct(
        &mut self,
        count: c_int,
        array_of_blocklengths: *mut c_int,
        array_of_displacements: *mut mpi_sys::MPI_Aint,
        array_of_types: *mut mpi_sys::MPI_Datatype,
        newtype: *mut mpi_sys::MPI_Datatype,
    ) {
    }
    #[inline]
    fn post_type_struct(
        &mut self,
        output: c_int,
        count: c_int,
        array_of_blocklengths: *mut c_int,
        array_of_displacements: *mut mpi_sys::MPI_Aint,
        array_of_types: *mut mpi_sys::MPI_Datatype,
        newtype: *mut mpi_sys::MPI_Datatype,
    ) {
    }
    #[inline]
    fn pre_type_ub(&mut self, mtype: mpi_sys::MPI_Datatype, ub: *mut mpi_sys::MPI_Aint) {}
    #[inline]
    fn post_type_ub(
        &mut self,
        output: c_int,
        mtype: mpi_sys::MPI_Datatype,
        ub: *mut mpi_sys::MPI_Aint,
    ) {
    }
    #[inline]
    fn pre_type_vector(
        &mut self,
        count: c_int,
        blocklength: c_int,
        stride: c_int,
        oldtype: mpi_sys::MPI_Datatype,
        newtype: *mut mpi_sys::MPI_Datatype,
    ) {
    }
    #[inline]
    fn post_type_vector(
        &mut self,
        output: c_int,
        count: c_int,
        blocklength: c_int,
        stride: c_int,
        oldtype: mpi_sys::MPI_Datatype,
        newtype: *mut mpi_sys::MPI_Datatype,
    ) {
    }
    #[inline]
    fn pre_unpack(
        &mut self,
        inbuf: *const c_void,
        insize: c_int,
        position: *mut c_int,
        outbuf: *mut c_void,
        outcount: c_int,
        datatype: mpi_sys::MPI_Datatype,
        comm: mpi_sys::MPI_Comm,
    ) {
    }
    #[inline]
    fn post_unpack(
        &mut self,
        output: c_int,
        inbuf: *const c_void,
        insize: c_int,
        position: *mut c_int,
        outbuf: *mut c_void,
        outcount: c_int,
        datatype: mpi_sys::MPI_Datatype,
        comm: mpi_sys::MPI_Comm,
    ) {
    }
    #[inline]
    fn pre_unpack_external(
        &mut self,
        datarep: *const c_char,
        inbuf: *const c_void,
        insize: mpi_sys::MPI_Aint,
        position: *mut mpi_sys::MPI_Aint,
        outbuf: *mut c_void,
        outcount: c_int,
        datatype: mpi_sys::MPI_Datatype,
    ) {
    }
    #[inline]
    fn post_unpack_external(
        &mut self,
        output: c_int,
        datarep: *const c_char,
        inbuf: *const c_void,
        insize: mpi_sys::MPI_Aint,
        position: *mut mpi_sys::MPI_Aint,
        outbuf: *mut c_void,
        outcount: c_int,
        datatype: mpi_sys::MPI_Datatype,
    ) {
    }
    #[inline]
    fn pre_unpublish_name(
        &mut self,
        service_name: *const c_char,
        info: mpi_sys::MPI_Info,
        port_name: *const c_char,
    ) {
    }
    #[inline]
    fn post_unpublish_name(
        &mut self,
        output: c_int,
        service_name: *const c_char,
        info: mpi_sys::MPI_Info,
        port_name: *const c_char,
    ) {
    }
    #[inline]
    fn pre_wait(&mut self, request: *mut mpi_sys::MPI_Request, status: *mut mpi_sys::MPI_Status) {}
    #[inline]
    fn post_wait(
        &mut self,
        output: c_int,
        request: *mut mpi_sys::MPI_Request,
        status: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn pre_waitall(
        &mut self,
        count: c_int,
        array_of_requests: *mut mpi_sys::MPI_Request,
        array_of_statuses: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn post_waitall(
        &mut self,
        output: c_int,
        count: c_int,
        array_of_requests: *mut mpi_sys::MPI_Request,
        array_of_statuses: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn pre_waitany(
        &mut self,
        count: c_int,
        array_of_requests: *mut mpi_sys::MPI_Request,
        index: *mut c_int,
        status: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn post_waitany(
        &mut self,
        output: c_int,
        count: c_int,
        array_of_requests: *mut mpi_sys::MPI_Request,
        index: *mut c_int,
        status: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn pre_waitsome(
        &mut self,
        incount: c_int,
        array_of_requests: *mut mpi_sys::MPI_Request,
        outcount: *mut c_int,
        array_of_indices: *mut c_int,
        array_of_statuses: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn post_waitsome(
        &mut self,
        output: c_int,
        incount: c_int,
        array_of_requests: *mut mpi_sys::MPI_Request,
        outcount: *mut c_int,
        array_of_indices: *mut c_int,
        array_of_statuses: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn pre_win_allocate(
        &mut self,
        size: mpi_sys::MPI_Aint,
        disp_unit: c_int,
        info: mpi_sys::MPI_Info,
        comm: mpi_sys::MPI_Comm,
        baseptr: *mut c_void,
        win: *mut mpi_sys::MPI_Win,
    ) {
    }
    #[inline]
    fn post_win_allocate(
        &mut self,
        output: c_int,
        size: mpi_sys::MPI_Aint,
        disp_unit: c_int,
        info: mpi_sys::MPI_Info,
        comm: mpi_sys::MPI_Comm,
        baseptr: *mut c_void,
        win: *mut mpi_sys::MPI_Win,
    ) {
    }
    #[inline]
    fn pre_win_allocate_shared(
        &mut self,
        size: mpi_sys::MPI_Aint,
        disp_unit: c_int,
        info: mpi_sys::MPI_Info,
        comm: mpi_sys::MPI_Comm,
        baseptr: *mut c_void,
        win: *mut mpi_sys::MPI_Win,
    ) {
    }
    #[inline]
    fn post_win_allocate_shared(
        &mut self,
        output: c_int,
        size: mpi_sys::MPI_Aint,
        disp_unit: c_int,
        info: mpi_sys::MPI_Info,
        comm: mpi_sys::MPI_Comm,
        baseptr: *mut c_void,
        win: *mut mpi_sys::MPI_Win,
    ) {
    }
    #[inline]
    fn pre_win_attach(
        &mut self,
        win: mpi_sys::MPI_Win,
        base: *mut c_void,
        size: mpi_sys::MPI_Aint,
    ) {
    }
    #[inline]
    fn post_win_attach(
        &mut self,
        output: c_int,
        win: mpi_sys::MPI_Win,
        base: *mut c_void,
        size: mpi_sys::MPI_Aint,
    ) {
    }
    #[inline]
    fn pre_win_call_errhandler(&mut self, win: mpi_sys::MPI_Win, errorcode: c_int) {}
    #[inline]
    fn post_win_call_errhandler(&mut self, output: c_int, win: mpi_sys::MPI_Win, errorcode: c_int) {
    }
    #[inline]
    fn pre_win_complete(&mut self, win: mpi_sys::MPI_Win) {}
    #[inline]
    fn post_win_complete(&mut self, output: c_int, win: mpi_sys::MPI_Win) {}
    #[inline]
    fn pre_win_create(
        &mut self,
        base: *mut c_void,
        size: mpi_sys::MPI_Aint,
        disp_unit: c_int,
        info: mpi_sys::MPI_Info,
        comm: mpi_sys::MPI_Comm,
        win: *mut mpi_sys::MPI_Win,
    ) {
    }
    #[inline]
    fn post_win_create(
        &mut self,
        output: c_int,
        base: *mut c_void,
        size: mpi_sys::MPI_Aint,
        disp_unit: c_int,
        info: mpi_sys::MPI_Info,
        comm: mpi_sys::MPI_Comm,
        win: *mut mpi_sys::MPI_Win,
    ) {
    }
    #[inline]
    fn pre_win_create_dynamic(
        &mut self,
        info: mpi_sys::MPI_Info,
        comm: mpi_sys::MPI_Comm,
        win: *mut mpi_sys::MPI_Win,
    ) {
    }
    #[inline]
    fn post_win_create_dynamic(
        &mut self,
        output: c_int,
        info: mpi_sys::MPI_Info,
        comm: mpi_sys::MPI_Comm,
        win: *mut mpi_sys::MPI_Win,
    ) {
    }
    #[inline]
    fn pre_win_create_errhandler(
        &mut self,
        function: *mut mpi_sys::MPI_Win_errhandler_function,
        errhandler: *mut mpi_sys::MPI_Errhandler,
    ) {
    }
    #[inline]
    fn post_win_create_errhandler(
        &mut self,
        output: c_int,
        function: *mut mpi_sys::MPI_Win_errhandler_function,
        errhandler: *mut mpi_sys::MPI_Errhandler,
    ) {
    }
    #[inline]
    fn pre_win_create_keyval(
        &mut self,
        win_copy_attr_fn: *mut mpi_sys::MPI_Win_copy_attr_function,
        win_delete_attr_fn: *mut mpi_sys::MPI_Win_delete_attr_function,
        win_keyval: *mut c_int,
        extra_state: *mut c_void,
    ) {
    }
    #[inline]
    fn post_win_create_keyval(
        &mut self,
        output: c_int,
        win_copy_attr_fn: *mut mpi_sys::MPI_Win_copy_attr_function,
        win_delete_attr_fn: *mut mpi_sys::MPI_Win_delete_attr_function,
        win_keyval: *mut c_int,
        extra_state: *mut c_void,
    ) {
    }
    #[inline]
    fn pre_win_delete_attr(&mut self, win: mpi_sys::MPI_Win, win_keyval: c_int) {}
    #[inline]
    fn post_win_delete_attr(&mut self, output: c_int, win: mpi_sys::MPI_Win, win_keyval: c_int) {}
    #[inline]
    fn pre_win_detach(&mut self, win: mpi_sys::MPI_Win, base: *const c_void) {}
    #[inline]
    fn post_win_detach(&mut self, output: c_int, win: mpi_sys::MPI_Win, base: *const c_void) {}
    #[inline]
    fn pre_win_fence(&mut self, assert: c_int, win: mpi_sys::MPI_Win) {}
    #[inline]
    fn post_win_fence(&mut self, output: c_int, assert: c_int, win: mpi_sys::MPI_Win) {}
    #[inline]
    fn pre_win_flush(&mut self, rank: c_int, win: mpi_sys::MPI_Win) {}
    #[inline]
    fn post_win_flush(&mut self, output: c_int, rank: c_int, win: mpi_sys::MPI_Win) {}
    #[inline]
    fn pre_win_flush_all(&mut self, win: mpi_sys::MPI_Win) {}
    #[inline]
    fn post_win_flush_all(&mut self, output: c_int, win: mpi_sys::MPI_Win) {}
    #[inline]
    fn pre_win_flush_local(&mut self, rank: c_int, win: mpi_sys::MPI_Win) {}
    #[inline]
    fn post_win_flush_local(&mut self, output: c_int, rank: c_int, win: mpi_sys::MPI_Win) {}
    #[inline]
    fn pre_win_flush_local_all(&mut self, win: mpi_sys::MPI_Win) {}
    #[inline]
    fn post_win_flush_local_all(&mut self, output: c_int, win: mpi_sys::MPI_Win) {}
    #[inline]
    fn pre_win_free(&mut self, win: *mut mpi_sys::MPI_Win) {}
    #[inline]
    fn post_win_free(&mut self, output: c_int, win: *mut mpi_sys::MPI_Win) {}
    #[inline]
    fn pre_win_free_keyval(&mut self, win_keyval: *mut c_int) {}
    #[inline]
    fn post_win_free_keyval(&mut self, output: c_int, win_keyval: *mut c_int) {}
    #[inline]
    fn pre_win_get_attr(
        &mut self,
        win: mpi_sys::MPI_Win,
        win_keyval: c_int,
        attribute_val: *mut c_void,
        flag: *mut c_int,
    ) {
    }
    #[inline]
    fn post_win_get_attr(
        &mut self,
        output: c_int,
        win: mpi_sys::MPI_Win,
        win_keyval: c_int,
        attribute_val: *mut c_void,
        flag: *mut c_int,
    ) {
    }
    #[inline]
    fn pre_win_get_errhandler(
        &mut self,
        win: mpi_sys::MPI_Win,
        errhandler: *mut mpi_sys::MPI_Errhandler,
    ) {
    }
    #[inline]
    fn post_win_get_errhandler(
        &mut self,
        output: c_int,
        win: mpi_sys::MPI_Win,
        errhandler: *mut mpi_sys::MPI_Errhandler,
    ) {
    }
    #[inline]
    fn pre_win_get_group(&mut self, win: mpi_sys::MPI_Win, group: *mut mpi_sys::MPI_Group) {}
    #[inline]
    fn post_win_get_group(
        &mut self,
        output: c_int,
        win: mpi_sys::MPI_Win,
        group: *mut mpi_sys::MPI_Group,
    ) {
    }
    #[inline]
    fn pre_win_get_info(&mut self, win: mpi_sys::MPI_Win, info_used: *mut mpi_sys::MPI_Info) {}
    #[inline]
    fn post_win_get_info(
        &mut self,
        output: c_int,
        win: mpi_sys::MPI_Win,
        info_used: *mut mpi_sys::MPI_Info,
    ) {
    }
    #[inline]
    fn pre_win_get_name(
        &mut self,
        win: mpi_sys::MPI_Win,
        win_name: *mut c_char,
        resultlen: *mut c_int,
    ) {
    }
    #[inline]
    fn post_win_get_name(
        &mut self,
        output: c_int,
        win: mpi_sys::MPI_Win,
        win_name: *mut c_char,
        resultlen: *mut c_int,
    ) {
    }
    #[inline]
    fn pre_win_lock(
        &mut self,
        lock_type: c_int,
        rank: c_int,
        assert: c_int,
        win: mpi_sys::MPI_Win,
    ) {
    }
    #[inline]
    fn post_win_lock(
        &mut self,
        output: c_int,
        lock_type: c_int,
        rank: c_int,
        assert: c_int,
        win: mpi_sys::MPI_Win,
    ) {
    }
    #[inline]
    fn pre_win_lock_all(&mut self, assert: c_int, win: mpi_sys::MPI_Win) {}
    #[inline]
    fn post_win_lock_all(&mut self, output: c_int, assert: c_int, win: mpi_sys::MPI_Win) {}
    #[inline]
    fn pre_win_post(&mut self, group: mpi_sys::MPI_Group, assert: c_int, win: mpi_sys::MPI_Win) {}
    #[inline]
    fn post_win_post(
        &mut self,
        output: c_int,
        group: mpi_sys::MPI_Group,
        assert: c_int,
        win: mpi_sys::MPI_Win,
    ) {
    }
    #[inline]
    fn pre_win_set_attr(
        &mut self,
        win: mpi_sys::MPI_Win,
        win_keyval: c_int,
        attribute_val: *mut c_void,
    ) {
    }
    #[inline]
    fn post_win_set_attr(
        &mut self,
        output: c_int,
        win: mpi_sys::MPI_Win,
        win_keyval: c_int,
        attribute_val: *mut c_void,
    ) {
    }
    #[inline]
    fn pre_win_set_errhandler(
        &mut self,
        win: mpi_sys::MPI_Win,
        errhandler: mpi_sys::MPI_Errhandler,
    ) {
    }
    #[inline]
    fn post_win_set_errhandler(
        &mut self,
        output: c_int,
        win: mpi_sys::MPI_Win,
        errhandler: mpi_sys::MPI_Errhandler,
    ) {
    }
    #[inline]
    fn pre_win_set_info(&mut self, win: mpi_sys::MPI_Win, info: mpi_sys::MPI_Info) {}
    #[inline]
    fn post_win_set_info(&mut self, output: c_int, win: mpi_sys::MPI_Win, info: mpi_sys::MPI_Info) {
    }
    #[inline]
    fn pre_win_set_name(&mut self, win: mpi_sys::MPI_Win, win_name: *const c_char) {}
    #[inline]
    fn post_win_set_name(&mut self, output: c_int, win: mpi_sys::MPI_Win, win_name: *const c_char) {
    }
    #[inline]
    fn pre_win_shared_query(
        &mut self,
        win: mpi_sys::MPI_Win,
        rank: c_int,
        size: *mut mpi_sys::MPI_Aint,
        disp_unit: *mut c_int,
        baseptr: *mut c_void,
    ) {
    }
    #[inline]
    fn post_win_shared_query(
        &mut self,
        output: c_int,
        win: mpi_sys::MPI_Win,
        rank: c_int,
        size: *mut mpi_sys::MPI_Aint,
        disp_unit: *mut c_int,
        baseptr: *mut c_void,
    ) {
    }
    #[inline]
    fn pre_win_start(&mut self, group: mpi_sys::MPI_Group, assert: c_int, win: mpi_sys::MPI_Win) {}
    #[inline]
    fn post_win_start(
        &mut self,
        output: c_int,
        group: mpi_sys::MPI_Group,
        assert: c_int,
        win: mpi_sys::MPI_Win,
    ) {
    }
    #[inline]
    fn pre_win_sync(&mut self, win: mpi_sys::MPI_Win) {}
    #[inline]
    fn post_win_sync(&mut self, output: c_int, win: mpi_sys::MPI_Win) {}
    #[inline]
    fn pre_win_test(&mut self, win: mpi_sys::MPI_Win, flag: *mut c_int) {}
    #[inline]
    fn post_win_test(&mut self, output: c_int, win: mpi_sys::MPI_Win, flag: *mut c_int) {}
    #[inline]
    fn pre_win_unlock(&mut self, rank: c_int, win: mpi_sys::MPI_Win) {}
    #[inline]
    fn post_win_unlock(&mut self, output: c_int, rank: c_int, win: mpi_sys::MPI_Win) {}
    #[inline]
    fn pre_win_unlock_all(&mut self, win: mpi_sys::MPI_Win) {}
    #[inline]
    fn post_win_unlock_all(&mut self, output: c_int, win: mpi_sys::MPI_Win) {}
    #[inline]
    fn pre_win_wait(&mut self, win: mpi_sys::MPI_Win) {}
    #[inline]
    fn post_win_wait(&mut self, output: c_int, win: mpi_sys::MPI_Win) {}
    #[inline]
    fn pre_wtick(&mut self) {}
    #[inline]
    fn post_wtick(&mut self, output: c_double) {}
    #[inline]
    fn pre_wtime(&mut self) {}
    #[inline]
    fn post_wtime(&mut self, output: c_double) {}
}
