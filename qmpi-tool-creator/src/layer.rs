use std::os::raw::{c_char, c_double, c_int, c_void};

#[allow(unused_variables)]
pub trait QmpiLayer {
    #[inline]
    fn pre_abort(comm: mpi_sys::MPI_Comm, errorcode: c_int) {}
    #[inline]
    fn post_abort(output: c_int, comm: mpi_sys::MPI_Comm, errorcode: c_int) {}
    #[inline]
    fn pre_accumulate(
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
    fn pre_add_error_class(errorclass: *mut c_int) {}
    #[inline]
    fn post_add_error_class(output: c_int, errorclass: *mut c_int) {}
    #[inline]
    fn pre_add_error_code(errorclass: c_int, errorcode: *mut c_int) {}
    #[inline]
    fn post_add_error_code(output: c_int, errorclass: c_int, errorcode: *mut c_int) {}
    #[inline]
    fn pre_add_error_string(errorcode: c_int, string: *const c_char) {}
    #[inline]
    fn post_add_error_string(output: c_int, errorcode: c_int, string: *const c_char) {}
    #[inline]
    fn pre_address(location: *mut c_void, address: *mut mpi_sys::MPI_Aint) {}
    #[inline]
    fn post_address(output: c_int, location: *mut c_void, address: *mut mpi_sys::MPI_Aint) {}
    #[inline]
    fn pre_allgather(
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
    fn pre_alloc_mem(size: mpi_sys::MPI_Aint, info: mpi_sys::MPI_Info, baseptr: *mut c_void) {}
    #[inline]
    fn post_alloc_mem(
        output: c_int,
        size: mpi_sys::MPI_Aint,
        info: mpi_sys::MPI_Info,
        baseptr: *mut c_void,
    ) {
    }
    #[inline]
    fn pre_allreduce(
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
    fn pre_attr_delete(comm: mpi_sys::MPI_Comm, keyval: c_int) {}
    #[inline]
    fn post_attr_delete(output: c_int, comm: mpi_sys::MPI_Comm, keyval: c_int) {}
    #[inline]
    fn pre_attr_get(
        comm: mpi_sys::MPI_Comm,
        keyval: c_int,
        attribute_val: *mut c_void,
        flag: *mut c_int,
    ) {
    }
    #[inline]
    fn post_attr_get(
        output: c_int,
        comm: mpi_sys::MPI_Comm,
        keyval: c_int,
        attribute_val: *mut c_void,
        flag: *mut c_int,
    ) {
    }
    #[inline]
    fn pre_attr_put(comm: mpi_sys::MPI_Comm, keyval: c_int, attribute_val: *mut c_void) {}
    #[inline]
    fn post_attr_put(
        output: c_int,
        comm: mpi_sys::MPI_Comm,
        keyval: c_int,
        attribute_val: *mut c_void,
    ) {
    }
    #[inline]
    fn pre_barrier(comm: mpi_sys::MPI_Comm) {}
    #[inline]
    fn post_barrier(output: c_int, comm: mpi_sys::MPI_Comm) {}
    #[inline]
    fn pre_bcast(
        buffer: *mut c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        root: c_int,
        comm: mpi_sys::MPI_Comm,
    ) {
    }
    #[inline]
    fn post_bcast(
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
    fn pre_buffer_attach(buffer: *mut c_void, size: c_int) {}
    #[inline]
    fn post_buffer_attach(output: c_int, buffer: *mut c_void, size: c_int) {}
    #[inline]
    fn pre_buffer_detach(buffer: *mut c_void, size: *mut c_int) {}
    #[inline]
    fn post_buffer_detach(output: c_int, buffer: *mut c_void, size: *mut c_int) {}
    #[inline]
    fn pre_cancel(request: *mut mpi_sys::MPI_Request) {}
    #[inline]
    fn post_cancel(output: c_int, request: *mut mpi_sys::MPI_Request) {}
    #[inline]
    fn pre_cart_coords(comm: mpi_sys::MPI_Comm, rank: c_int, maxdims: c_int, coords: *mut c_int) {}
    #[inline]
    fn post_cart_coords(
        output: c_int,
        comm: mpi_sys::MPI_Comm,
        rank: c_int,
        maxdims: c_int,
        coords: *mut c_int,
    ) {
    }
    #[inline]
    fn pre_cart_create(
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
        comm: mpi_sys::MPI_Comm,
        maxdims: c_int,
        dims: *mut c_int,
        periods: *mut c_int,
        coords: *mut c_int,
    ) {
    }
    #[inline]
    fn post_cart_get(
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
        comm: mpi_sys::MPI_Comm,
        ndims: c_int,
        dims: *const c_int,
        periods: *const c_int,
        newrank: *mut c_int,
    ) {
    }
    #[inline]
    fn post_cart_map(
        output: c_int,
        comm: mpi_sys::MPI_Comm,
        ndims: c_int,
        dims: *const c_int,
        periods: *const c_int,
        newrank: *mut c_int,
    ) {
    }
    #[inline]
    fn pre_cart_rank(comm: mpi_sys::MPI_Comm, coords: *const c_int, rank: *mut c_int) {}
    #[inline]
    fn post_cart_rank(
        output: c_int,
        comm: mpi_sys::MPI_Comm,
        coords: *const c_int,
        rank: *mut c_int,
    ) {
    }
    #[inline]
    fn pre_cart_shift(
        comm: mpi_sys::MPI_Comm,
        direction: c_int,
        disp: c_int,
        rank_source: *mut c_int,
        rank_dest: *mut c_int,
    ) {
    }
    #[inline]
    fn post_cart_shift(
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
        comm: mpi_sys::MPI_Comm,
        remain_dims: *const c_int,
        new_comm: *mut mpi_sys::MPI_Comm,
    ) {
    }
    #[inline]
    fn post_cart_sub(
        output: c_int,
        comm: mpi_sys::MPI_Comm,
        remain_dims: *const c_int,
        new_comm: *mut mpi_sys::MPI_Comm,
    ) {
    }
    #[inline]
    fn pre_cartdim_get(comm: mpi_sys::MPI_Comm, ndims: *mut c_int) {}
    #[inline]
    fn post_cartdim_get(output: c_int, comm: mpi_sys::MPI_Comm, ndims: *mut c_int) {}
    #[inline]
    fn pre_close_port(port_name: *const c_char) {}
    #[inline]
    fn post_close_port(output: c_int, port_name: *const c_char) {}
    #[inline]
    fn pre_comm_accept(
        port_name: *const c_char,
        info: mpi_sys::MPI_Info,
        root: c_int,
        comm: mpi_sys::MPI_Comm,
        newcomm: *mut mpi_sys::MPI_Comm,
    ) {
    }
    #[inline]
    fn post_comm_accept(
        output: c_int,
        port_name: *const c_char,
        info: mpi_sys::MPI_Info,
        root: c_int,
        comm: mpi_sys::MPI_Comm,
        newcomm: *mut mpi_sys::MPI_Comm,
    ) {
    }
    #[inline]
    fn pre_comm_call_errhandler(comm: mpi_sys::MPI_Comm, errorcode: c_int) {}
    #[inline]
    fn post_comm_call_errhandler(output: c_int, comm: mpi_sys::MPI_Comm, errorcode: c_int) {}
    #[inline]
    fn pre_comm_compare(comm1: mpi_sys::MPI_Comm, comm2: mpi_sys::MPI_Comm, result: *mut c_int) {}
    #[inline]
    fn post_comm_compare(
        output: c_int,
        comm1: mpi_sys::MPI_Comm,
        comm2: mpi_sys::MPI_Comm,
        result: *mut c_int,
    ) {
    }
    #[inline]
    fn pre_comm_connect(
        port_name: *const c_char,
        info: mpi_sys::MPI_Info,
        root: c_int,
        comm: mpi_sys::MPI_Comm,
        newcomm: *mut mpi_sys::MPI_Comm,
    ) {
    }
    #[inline]
    fn post_comm_connect(
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
        comm: mpi_sys::MPI_Comm,
        group: mpi_sys::MPI_Group,
        newcomm: *mut mpi_sys::MPI_Comm,
    ) {
    }
    #[inline]
    fn post_comm_create(
        output: c_int,
        comm: mpi_sys::MPI_Comm,
        group: mpi_sys::MPI_Group,
        newcomm: *mut mpi_sys::MPI_Comm,
    ) {
    }
    #[inline]
    fn pre_comm_create_errhandler(
        function: *mut mpi_sys::MPI_Comm_errhandler_function,
        errhandler: *mut mpi_sys::MPI_Errhandler,
    ) {
    }
    #[inline]
    fn post_comm_create_errhandler(
        output: c_int,
        function: *mut mpi_sys::MPI_Comm_errhandler_function,
        errhandler: *mut mpi_sys::MPI_Errhandler,
    ) {
    }
    #[inline]
    fn pre_comm_create_group(
        comm: mpi_sys::MPI_Comm,
        group: mpi_sys::MPI_Group,
        tag: c_int,
        newcomm: *mut mpi_sys::MPI_Comm,
    ) {
    }
    #[inline]
    fn post_comm_create_group(
        output: c_int,
        comm: mpi_sys::MPI_Comm,
        group: mpi_sys::MPI_Group,
        tag: c_int,
        newcomm: *mut mpi_sys::MPI_Comm,
    ) {
    }
    #[inline]
    fn pre_comm_create_keyval(
        comm_copy_attr_fn: *mut mpi_sys::MPI_Comm_copy_attr_function,
        comm_delete_attr_fn: *mut mpi_sys::MPI_Comm_delete_attr_function,
        comm_keyval: *mut c_int,
        extra_state: *mut c_void,
    ) {
    }
    #[inline]
    fn post_comm_create_keyval(
        output: c_int,
        comm_copy_attr_fn: *mut mpi_sys::MPI_Comm_copy_attr_function,
        comm_delete_attr_fn: *mut mpi_sys::MPI_Comm_delete_attr_function,
        comm_keyval: *mut c_int,
        extra_state: *mut c_void,
    ) {
    }
    #[inline]
    fn pre_comm_delete_attr(comm: mpi_sys::MPI_Comm, comm_keyval: c_int) {}
    #[inline]
    fn post_comm_delete_attr(output: c_int, comm: mpi_sys::MPI_Comm, comm_keyval: c_int) {}
    #[inline]
    fn pre_comm_disconnect(comm: *mut mpi_sys::MPI_Comm) {}
    #[inline]
    fn post_comm_disconnect(output: c_int, comm: *mut mpi_sys::MPI_Comm) {}
    #[inline]
    fn pre_comm_dup(comm: mpi_sys::MPI_Comm, newcomm: *mut mpi_sys::MPI_Comm) {}
    #[inline]
    fn post_comm_dup(output: c_int, comm: mpi_sys::MPI_Comm, newcomm: *mut mpi_sys::MPI_Comm) {}
    #[inline]
    fn pre_comm_dup_with_info(
        comm: mpi_sys::MPI_Comm,
        info: mpi_sys::MPI_Info,
        newcomm: *mut mpi_sys::MPI_Comm,
    ) {
    }
    #[inline]
    fn post_comm_dup_with_info(
        output: c_int,
        comm: mpi_sys::MPI_Comm,
        info: mpi_sys::MPI_Info,
        newcomm: *mut mpi_sys::MPI_Comm,
    ) {
    }
    #[inline]
    fn pre_comm_free(comm: *mut mpi_sys::MPI_Comm) {}
    #[inline]
    fn post_comm_free(output: c_int, comm: *mut mpi_sys::MPI_Comm) {}
    #[inline]
    fn pre_comm_free_keyval(comm_keyval: *mut c_int) {}
    #[inline]
    fn post_comm_free_keyval(output: c_int, comm_keyval: *mut c_int) {}
    #[inline]
    fn pre_comm_get_attr(
        comm: mpi_sys::MPI_Comm,
        comm_keyval: c_int,
        attribute_val: *mut c_void,
        flag: *mut c_int,
    ) {
    }
    #[inline]
    fn post_comm_get_attr(
        output: c_int,
        comm: mpi_sys::MPI_Comm,
        comm_keyval: c_int,
        attribute_val: *mut c_void,
        flag: *mut c_int,
    ) {
    }
    #[inline]
    fn pre_comm_get_errhandler(comm: mpi_sys::MPI_Comm, erhandler: *mut mpi_sys::MPI_Errhandler) {}
    #[inline]
    fn post_comm_get_errhandler(
        output: c_int,
        comm: mpi_sys::MPI_Comm,
        erhandler: *mut mpi_sys::MPI_Errhandler,
    ) {
    }
    #[inline]
    fn pre_comm_get_info(comm: mpi_sys::MPI_Comm, info_used: *mut mpi_sys::MPI_Info) {}
    #[inline]
    fn post_comm_get_info(
        output: c_int,
        comm: mpi_sys::MPI_Comm,
        info_used: *mut mpi_sys::MPI_Info,
    ) {
    }
    #[inline]
    fn pre_comm_get_name(comm: mpi_sys::MPI_Comm, comm_name: *mut c_char, resultlen: *mut c_int) {}
    #[inline]
    fn post_comm_get_name(
        output: c_int,
        comm: mpi_sys::MPI_Comm,
        comm_name: *mut c_char,
        resultlen: *mut c_int,
    ) {
    }
    #[inline]
    fn pre_comm_get_parent(parent: *mut mpi_sys::MPI_Comm) {}
    #[inline]
    fn post_comm_get_parent(output: c_int, parent: *mut mpi_sys::MPI_Comm) {}
    #[inline]
    fn pre_comm_group(comm: mpi_sys::MPI_Comm, group: *mut mpi_sys::MPI_Group) {}
    #[inline]
    fn post_comm_group(output: c_int, comm: mpi_sys::MPI_Comm, group: *mut mpi_sys::MPI_Group) {}
    #[inline]
    fn pre_comm_idup(
        comm: mpi_sys::MPI_Comm,
        newcomm: *mut mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn post_comm_idup(
        output: c_int,
        comm: mpi_sys::MPI_Comm,
        newcomm: *mut mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn pre_comm_join(fd: c_int, intercomm: *mut mpi_sys::MPI_Comm) {}
    #[inline]
    fn post_comm_join(output: c_int, fd: c_int, intercomm: *mut mpi_sys::MPI_Comm) {}
    #[inline]
    fn pre_comm_rank(comm: mpi_sys::MPI_Comm, rank: *mut c_int) {}
    #[inline]
    fn post_comm_rank(output: c_int, comm: mpi_sys::MPI_Comm, rank: *mut c_int) {}
    #[inline]
    fn pre_comm_remote_group(comm: mpi_sys::MPI_Comm, group: *mut mpi_sys::MPI_Group) {}
    #[inline]
    fn post_comm_remote_group(
        output: c_int,
        comm: mpi_sys::MPI_Comm,
        group: *mut mpi_sys::MPI_Group,
    ) {
    }
    #[inline]
    fn pre_comm_remote_size(comm: mpi_sys::MPI_Comm, size: *mut c_int) {}
    #[inline]
    fn post_comm_remote_size(output: c_int, comm: mpi_sys::MPI_Comm, size: *mut c_int) {}
    #[inline]
    fn pre_comm_set_attr(comm: mpi_sys::MPI_Comm, comm_keyval: c_int, attribute_val: *mut c_void) {}
    #[inline]
    fn post_comm_set_attr(
        output: c_int,
        comm: mpi_sys::MPI_Comm,
        comm_keyval: c_int,
        attribute_val: *mut c_void,
    ) {
    }
    #[inline]
    fn pre_comm_set_errhandler(comm: mpi_sys::MPI_Comm, errhandler: mpi_sys::MPI_Errhandler) {}
    #[inline]
    fn post_comm_set_errhandler(
        output: c_int,
        comm: mpi_sys::MPI_Comm,
        errhandler: mpi_sys::MPI_Errhandler,
    ) {
    }
    #[inline]
    fn pre_comm_set_info(comm: mpi_sys::MPI_Comm, info: mpi_sys::MPI_Info) {}
    #[inline]
    fn post_comm_set_info(output: c_int, comm: mpi_sys::MPI_Comm, info: mpi_sys::MPI_Info) {}
    #[inline]
    fn pre_comm_set_name(comm: mpi_sys::MPI_Comm, comm_name: *const c_char) {}
    #[inline]
    fn post_comm_set_name(output: c_int, comm: mpi_sys::MPI_Comm, comm_name: *const c_char) {}
    #[inline]
    fn pre_comm_size(comm: mpi_sys::MPI_Comm, size: *mut c_int) {}
    #[inline]
    fn post_comm_size(output: c_int, comm: mpi_sys::MPI_Comm, size: *mut c_int) {}
    #[inline]
    fn pre_comm_split(
        comm: mpi_sys::MPI_Comm,
        color: c_int,
        key: c_int,
        newcomm: *mut mpi_sys::MPI_Comm,
    ) {
    }
    #[inline]
    fn post_comm_split(
        output: c_int,
        comm: mpi_sys::MPI_Comm,
        color: c_int,
        key: c_int,
        newcomm: *mut mpi_sys::MPI_Comm,
    ) {
    }
    #[inline]
    fn pre_comm_split_type(
        comm: mpi_sys::MPI_Comm,
        split_type: c_int,
        key: c_int,
        info: mpi_sys::MPI_Info,
        newcomm: *mut mpi_sys::MPI_Comm,
    ) {
    }
    #[inline]
    fn post_comm_split_type(
        output: c_int,
        comm: mpi_sys::MPI_Comm,
        split_type: c_int,
        key: c_int,
        info: mpi_sys::MPI_Info,
        newcomm: *mut mpi_sys::MPI_Comm,
    ) {
    }
    #[inline]
    fn pre_comm_test_inter(comm: mpi_sys::MPI_Comm, flag: *mut c_int) {}
    #[inline]
    fn post_comm_test_inter(output: c_int, comm: mpi_sys::MPI_Comm, flag: *mut c_int) {}
    #[inline]
    fn pre_compare_and_swap(
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
    fn pre_dims_create(nnodes: c_int, ndims: c_int, dims: *mut c_int) {}
    #[inline]
    fn post_dims_create(output: c_int, nnodes: c_int, ndims: c_int, dims: *mut c_int) {}
    #[inline]
    fn pre_dist_graph_create(
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
        comm: mpi_sys::MPI_Comm,
        inneighbors: *mut c_int,
        outneighbors: *mut c_int,
        weighted: *mut c_int,
    ) {
    }
    #[inline]
    fn post_dist_graph_neighbors_count(
        output: c_int,
        comm: mpi_sys::MPI_Comm,
        inneighbors: *mut c_int,
        outneighbors: *mut c_int,
        weighted: *mut c_int,
    ) {
    }
    #[inline]
    fn pre_errhandler_create(
        function: *mut mpi_sys::MPI_Handler_function,
        errhandler: *mut mpi_sys::MPI_Errhandler,
    ) {
    }
    #[inline]
    fn post_errhandler_create(
        output: c_int,
        function: *mut mpi_sys::MPI_Handler_function,
        errhandler: *mut mpi_sys::MPI_Errhandler,
    ) {
    }
    #[inline]
    fn pre_errhandler_free(errhandler: *mut mpi_sys::MPI_Errhandler) {}
    #[inline]
    fn post_errhandler_free(output: c_int, errhandler: *mut mpi_sys::MPI_Errhandler) {}
    #[inline]
    fn pre_errhandler_get(comm: mpi_sys::MPI_Comm, errhandler: *mut mpi_sys::MPI_Errhandler) {}
    #[inline]
    fn post_errhandler_get(
        output: c_int,
        comm: mpi_sys::MPI_Comm,
        errhandler: *mut mpi_sys::MPI_Errhandler,
    ) {
    }
    #[inline]
    fn pre_errhandler_set(comm: mpi_sys::MPI_Comm, errhandler: mpi_sys::MPI_Errhandler) {}
    #[inline]
    fn post_errhandler_set(
        output: c_int,
        comm: mpi_sys::MPI_Comm,
        errhandler: mpi_sys::MPI_Errhandler,
    ) {
    }
    #[inline]
    fn pre_error_class(errorcode: c_int, errorclass: *mut c_int) {}
    #[inline]
    fn post_error_class(output: c_int, errorcode: c_int, errorclass: *mut c_int) {}
    #[inline]
    fn pre_error_string(errorcode: c_int, string: *mut c_char, resultlen: *mut c_int) {}
    #[inline]
    fn post_error_string(
        output: c_int,
        errorcode: c_int,
        string: *mut c_char,
        resultlen: *mut c_int,
    ) {
    }
    #[inline]
    fn pre_exscan(
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
    fn pre_file_call_errhandler(fh: mpi_sys::MPI_File, errorcode: c_int) {}
    #[inline]
    fn post_file_call_errhandler(output: c_int, fh: mpi_sys::MPI_File, errorcode: c_int) {}
    #[inline]
    fn pre_file_close(fh: *mut mpi_sys::MPI_File) {}
    #[inline]
    fn post_file_close(output: c_int, fh: *mut mpi_sys::MPI_File) {}
    #[inline]
    fn pre_file_create_errhandler(
        function: *mut mpi_sys::MPI_File_errhandler_function,
        errhandler: *mut mpi_sys::MPI_Errhandler,
    ) {
    }
    #[inline]
    fn post_file_create_errhandler(
        output: c_int,
        function: *mut mpi_sys::MPI_File_errhandler_function,
        errhandler: *mut mpi_sys::MPI_Errhandler,
    ) {
    }
    #[inline]
    fn pre_file_delete(filename: *const c_char, info: mpi_sys::MPI_Info) {}
    #[inline]
    fn post_file_delete(output: c_int, filename: *const c_char, info: mpi_sys::MPI_Info) {}
    #[inline]
    fn pre_file_get_amode(fh: mpi_sys::MPI_File, amode: *mut c_int) {}
    #[inline]
    fn post_file_get_amode(output: c_int, fh: mpi_sys::MPI_File, amode: *mut c_int) {}
    #[inline]
    fn pre_file_get_atomicity(fh: mpi_sys::MPI_File, flag: *mut c_int) {}
    #[inline]
    fn post_file_get_atomicity(output: c_int, fh: mpi_sys::MPI_File, flag: *mut c_int) {}
    #[inline]
    fn pre_file_get_byte_offset(
        fh: mpi_sys::MPI_File,
        offset: mpi_sys::MPI_Offset,
        disp: *mut mpi_sys::MPI_Offset,
    ) {
    }
    #[inline]
    fn post_file_get_byte_offset(
        output: c_int,
        fh: mpi_sys::MPI_File,
        offset: mpi_sys::MPI_Offset,
        disp: *mut mpi_sys::MPI_Offset,
    ) {
    }
    #[inline]
    fn pre_file_get_errhandler(file: mpi_sys::MPI_File, errhandler: *mut mpi_sys::MPI_Errhandler) {}
    #[inline]
    fn post_file_get_errhandler(
        output: c_int,
        file: mpi_sys::MPI_File,
        errhandler: *mut mpi_sys::MPI_Errhandler,
    ) {
    }
    #[inline]
    fn pre_file_get_group(fh: mpi_sys::MPI_File, group: *mut mpi_sys::MPI_Group) {}
    #[inline]
    fn post_file_get_group(output: c_int, fh: mpi_sys::MPI_File, group: *mut mpi_sys::MPI_Group) {}
    #[inline]
    fn pre_file_get_info(fh: mpi_sys::MPI_File, info_used: *mut mpi_sys::MPI_Info) {}
    #[inline]
    fn post_file_get_info(output: c_int, fh: mpi_sys::MPI_File, info_used: *mut mpi_sys::MPI_Info) {
    }
    #[inline]
    fn pre_file_get_position(fh: mpi_sys::MPI_File, offset: *mut mpi_sys::MPI_Offset) {}
    #[inline]
    fn post_file_get_position(
        output: c_int,
        fh: mpi_sys::MPI_File,
        offset: *mut mpi_sys::MPI_Offset,
    ) {
    }
    #[inline]
    fn pre_file_get_position_shared(fh: mpi_sys::MPI_File, offset: *mut mpi_sys::MPI_Offset) {}
    #[inline]
    fn post_file_get_position_shared(
        output: c_int,
        fh: mpi_sys::MPI_File,
        offset: *mut mpi_sys::MPI_Offset,
    ) {
    }
    #[inline]
    fn pre_file_get_size(fh: mpi_sys::MPI_File, size: *mut mpi_sys::MPI_Offset) {}
    #[inline]
    fn post_file_get_size(output: c_int, fh: mpi_sys::MPI_File, size: *mut mpi_sys::MPI_Offset) {}
    #[inline]
    fn pre_file_get_type_extent(
        fh: mpi_sys::MPI_File,
        datatype: mpi_sys::MPI_Datatype,
        extent: *mut mpi_sys::MPI_Aint,
    ) {
    }
    #[inline]
    fn post_file_get_type_extent(
        output: c_int,
        fh: mpi_sys::MPI_File,
        datatype: mpi_sys::MPI_Datatype,
        extent: *mut mpi_sys::MPI_Aint,
    ) {
    }
    #[inline]
    fn pre_file_get_view(
        fh: mpi_sys::MPI_File,
        disp: *mut mpi_sys::MPI_Offset,
        etype: *mut mpi_sys::MPI_Datatype,
        filetype: *mut mpi_sys::MPI_Datatype,
        datarep: *mut c_char,
    ) {
    }
    #[inline]
    fn post_file_get_view(
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
        fh: mpi_sys::MPI_File,
        buf: *mut c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        request: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn post_file_iread(
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
        fh: mpi_sys::MPI_File,
        buf: *mut c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        request: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn post_file_iread_all(
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
        fh: mpi_sys::MPI_File,
        buf: *mut c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        request: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn post_file_iread_shared(
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
        fh: mpi_sys::MPI_File,
        buf: *const c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        request: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn post_file_iwrite(
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
        fh: mpi_sys::MPI_File,
        buf: *const c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        request: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn post_file_iwrite_all(
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
        fh: mpi_sys::MPI_File,
        buf: *const c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        request: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn post_file_iwrite_shared(
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
        comm: mpi_sys::MPI_Comm,
        filename: *const c_char,
        amode: c_int,
        info: mpi_sys::MPI_Info,
        fh: *mut mpi_sys::MPI_File,
    ) {
    }
    #[inline]
    fn post_file_open(
        output: c_int,
        comm: mpi_sys::MPI_Comm,
        filename: *const c_char,
        amode: c_int,
        info: mpi_sys::MPI_Info,
        fh: *mut mpi_sys::MPI_File,
    ) {
    }
    #[inline]
    fn pre_file_preallocate(fh: mpi_sys::MPI_File, size: mpi_sys::MPI_Offset) {}
    #[inline]
    fn post_file_preallocate(output: c_int, fh: mpi_sys::MPI_File, size: mpi_sys::MPI_Offset) {}
    #[inline]
    fn pre_file_read(
        fh: mpi_sys::MPI_File,
        buf: *mut c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        status: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn post_file_read(
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
        fh: mpi_sys::MPI_File,
        buf: *mut c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        status: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn post_file_read_all(
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
        fh: mpi_sys::MPI_File,
        buf: *mut c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
    ) {
    }
    #[inline]
    fn post_file_read_all_begin(
        output: c_int,
        fh: mpi_sys::MPI_File,
        buf: *mut c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
    ) {
    }
    #[inline]
    fn pre_file_read_all_end(
        fh: mpi_sys::MPI_File,
        buf: *mut c_void,
        status: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn post_file_read_all_end(
        output: c_int,
        fh: mpi_sys::MPI_File,
        buf: *mut c_void,
        status: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn pre_file_read_at(
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
        fh: mpi_sys::MPI_File,
        offset: mpi_sys::MPI_Offset,
        buf: *mut c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
    ) {
    }
    #[inline]
    fn post_file_read_at_all_begin(
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
        fh: mpi_sys::MPI_File,
        buf: *mut c_void,
        status: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn post_file_read_at_all_end(
        output: c_int,
        fh: mpi_sys::MPI_File,
        buf: *mut c_void,
        status: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn pre_file_read_ordered(
        fh: mpi_sys::MPI_File,
        buf: *mut c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        status: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn post_file_read_ordered(
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
        fh: mpi_sys::MPI_File,
        buf: *mut c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
    ) {
    }
    #[inline]
    fn post_file_read_ordered_begin(
        output: c_int,
        fh: mpi_sys::MPI_File,
        buf: *mut c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
    ) {
    }
    #[inline]
    fn pre_file_read_ordered_end(
        fh: mpi_sys::MPI_File,
        buf: *mut c_void,
        status: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn post_file_read_ordered_end(
        output: c_int,
        fh: mpi_sys::MPI_File,
        buf: *mut c_void,
        status: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn pre_file_read_shared(
        fh: mpi_sys::MPI_File,
        buf: *mut c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        status: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn post_file_read_shared(
        output: c_int,
        fh: mpi_sys::MPI_File,
        buf: *mut c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        status: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn pre_file_seek(fh: mpi_sys::MPI_File, offset: mpi_sys::MPI_Offset, whence: c_int) {}
    #[inline]
    fn post_file_seek(
        output: c_int,
        fh: mpi_sys::MPI_File,
        offset: mpi_sys::MPI_Offset,
        whence: c_int,
    ) {
    }
    #[inline]
    fn pre_file_seek_shared(fh: mpi_sys::MPI_File, offset: mpi_sys::MPI_Offset, whence: c_int) {}
    #[inline]
    fn post_file_seek_shared(
        output: c_int,
        fh: mpi_sys::MPI_File,
        offset: mpi_sys::MPI_Offset,
        whence: c_int,
    ) {
    }
    #[inline]
    fn pre_file_set_atomicity(fh: mpi_sys::MPI_File, flag: c_int) {}
    #[inline]
    fn post_file_set_atomicity(output: c_int, fh: mpi_sys::MPI_File, flag: c_int) {}
    #[inline]
    fn pre_file_set_errhandler(file: mpi_sys::MPI_File, errhandler: mpi_sys::MPI_Errhandler) {}
    #[inline]
    fn post_file_set_errhandler(
        output: c_int,
        file: mpi_sys::MPI_File,
        errhandler: mpi_sys::MPI_Errhandler,
    ) {
    }
    #[inline]
    fn pre_file_set_info(fh: mpi_sys::MPI_File, info: mpi_sys::MPI_Info) {}
    #[inline]
    fn post_file_set_info(output: c_int, fh: mpi_sys::MPI_File, info: mpi_sys::MPI_Info) {}
    #[inline]
    fn pre_file_set_size(fh: mpi_sys::MPI_File, size: mpi_sys::MPI_Offset) {}
    #[inline]
    fn post_file_set_size(output: c_int, fh: mpi_sys::MPI_File, size: mpi_sys::MPI_Offset) {}
    #[inline]
    fn pre_file_set_view(
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
    fn pre_file_sync(fh: mpi_sys::MPI_File) {}
    #[inline]
    fn post_file_sync(output: c_int, fh: mpi_sys::MPI_File) {}
    #[inline]
    fn pre_file_write(
        fh: mpi_sys::MPI_File,
        buf: *const c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        status: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn post_file_write(
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
        fh: mpi_sys::MPI_File,
        buf: *const c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        status: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn post_file_write_all(
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
        fh: mpi_sys::MPI_File,
        buf: *const c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
    ) {
    }
    #[inline]
    fn post_file_write_all_begin(
        output: c_int,
        fh: mpi_sys::MPI_File,
        buf: *const c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
    ) {
    }
    #[inline]
    fn pre_file_write_all_end(
        fh: mpi_sys::MPI_File,
        buf: *const c_void,
        status: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn post_file_write_all_end(
        output: c_int,
        fh: mpi_sys::MPI_File,
        buf: *const c_void,
        status: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn pre_file_write_at(
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
        fh: mpi_sys::MPI_File,
        offset: mpi_sys::MPI_Offset,
        buf: *const c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
    ) {
    }
    #[inline]
    fn post_file_write_at_all_begin(
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
        fh: mpi_sys::MPI_File,
        buf: *const c_void,
        status: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn post_file_write_at_all_end(
        output: c_int,
        fh: mpi_sys::MPI_File,
        buf: *const c_void,
        status: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn pre_file_write_ordered(
        fh: mpi_sys::MPI_File,
        buf: *const c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        status: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn post_file_write_ordered(
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
        fh: mpi_sys::MPI_File,
        buf: *const c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
    ) {
    }
    #[inline]
    fn post_file_write_ordered_begin(
        output: c_int,
        fh: mpi_sys::MPI_File,
        buf: *const c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
    ) {
    }
    #[inline]
    fn pre_file_write_ordered_end(
        fh: mpi_sys::MPI_File,
        buf: *const c_void,
        status: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn post_file_write_ordered_end(
        output: c_int,
        fh: mpi_sys::MPI_File,
        buf: *const c_void,
        status: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn pre_file_write_shared(
        fh: mpi_sys::MPI_File,
        buf: *const c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        status: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn post_file_write_shared(
        output: c_int,
        fh: mpi_sys::MPI_File,
        buf: *const c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        status: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn pre_finalize() {}
    #[inline]
    fn post_finalize(output: c_int) {}
    #[inline]
    fn pre_finalized(flag: *mut c_int) {}
    #[inline]
    fn post_finalized(output: c_int, flag: *mut c_int) {}
    #[inline]
    fn pre_free_mem(base: *mut c_void) {}
    #[inline]
    fn post_free_mem(output: c_int, base: *mut c_void) {}
    #[inline]
    fn pre_gather(
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
    fn pre_get_address(location: *const c_void, address: *mut mpi_sys::MPI_Aint) {}
    #[inline]
    fn post_get_address(output: c_int, location: *const c_void, address: *mut mpi_sys::MPI_Aint) {}
    #[inline]
    fn pre_get_count(
        status: *const mpi_sys::MPI_Status,
        datatype: mpi_sys::MPI_Datatype,
        count: *mut c_int,
    ) {
    }
    #[inline]
    fn post_get_count(
        output: c_int,
        status: *const mpi_sys::MPI_Status,
        datatype: mpi_sys::MPI_Datatype,
        count: *mut c_int,
    ) {
    }
    #[inline]
    fn pre_get_elements(
        status: *const mpi_sys::MPI_Status,
        datatype: mpi_sys::MPI_Datatype,
        count: *mut c_int,
    ) {
    }
    #[inline]
    fn post_get_elements(
        output: c_int,
        status: *const mpi_sys::MPI_Status,
        datatype: mpi_sys::MPI_Datatype,
        count: *mut c_int,
    ) {
    }
    #[inline]
    fn pre_get_elements_x(
        status: *const mpi_sys::MPI_Status,
        datatype: mpi_sys::MPI_Datatype,
        count: *mut mpi_sys::MPI_Count,
    ) {
    }
    #[inline]
    fn post_get_elements_x(
        output: c_int,
        status: *const mpi_sys::MPI_Status,
        datatype: mpi_sys::MPI_Datatype,
        count: *mut mpi_sys::MPI_Count,
    ) {
    }
    #[inline]
    fn pre_get_library_version(version: *mut c_char, resultlen: *mut c_int) {}
    #[inline]
    fn post_get_library_version(output: c_int, version: *mut c_char, resultlen: *mut c_int) {}
    #[inline]
    fn pre_get_processor_name(name: *mut c_char, resultlen: *mut c_int) {}
    #[inline]
    fn post_get_processor_name(output: c_int, name: *mut c_char, resultlen: *mut c_int) {}
    #[inline]
    fn pre_get_version(version: *mut c_int, subversion: *mut c_int) {}
    #[inline]
    fn post_get_version(output: c_int, version: *mut c_int, subversion: *mut c_int) {}
    #[inline]
    fn pre_graph_create(
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
        comm: mpi_sys::MPI_Comm,
        maxindex: c_int,
        maxedges: c_int,
        index: *mut c_int,
        edges: *mut c_int,
    ) {
    }
    #[inline]
    fn post_graph_get(
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
        comm: mpi_sys::MPI_Comm,
        nnodes: c_int,
        index: *const c_int,
        edges: *const c_int,
        newrank: *mut c_int,
    ) {
    }
    #[inline]
    fn post_graph_map(
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
        comm: mpi_sys::MPI_Comm,
        rank: c_int,
        maxneighbors: c_int,
        neighbors: *mut c_int,
    ) {
    }
    #[inline]
    fn post_graph_neighbors(
        output: c_int,
        comm: mpi_sys::MPI_Comm,
        rank: c_int,
        maxneighbors: c_int,
        neighbors: *mut c_int,
    ) {
    }
    #[inline]
    fn pre_graph_neighbors_count(comm: mpi_sys::MPI_Comm, rank: c_int, nneighbors: *mut c_int) {}
    #[inline]
    fn post_graph_neighbors_count(
        output: c_int,
        comm: mpi_sys::MPI_Comm,
        rank: c_int,
        nneighbors: *mut c_int,
    ) {
    }
    #[inline]
    fn pre_graphdims_get(comm: mpi_sys::MPI_Comm, nnodes: *mut c_int, nedges: *mut c_int) {}
    #[inline]
    fn post_graphdims_get(
        output: c_int,
        comm: mpi_sys::MPI_Comm,
        nnodes: *mut c_int,
        nedges: *mut c_int,
    ) {
    }
    #[inline]
    fn pre_grequest_complete(request: mpi_sys::MPI_Request) {}
    #[inline]
    fn post_grequest_complete(output: c_int, request: mpi_sys::MPI_Request) {}
    #[inline]
    fn pre_grequest_start(
        query_fn: *mut mpi_sys::MPI_Grequest_query_function,
        free_fn: *mut mpi_sys::MPI_Grequest_free_function,
        cancel_fn: *mut mpi_sys::MPI_Grequest_cancel_function,
        extra_state: *mut c_void,
        request: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn post_grequest_start(
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
        group1: mpi_sys::MPI_Group,
        group2: mpi_sys::MPI_Group,
        result: *mut c_int,
    ) {
    }
    #[inline]
    fn post_group_compare(
        output: c_int,
        group1: mpi_sys::MPI_Group,
        group2: mpi_sys::MPI_Group,
        result: *mut c_int,
    ) {
    }
    #[inline]
    fn pre_group_difference(
        group1: mpi_sys::MPI_Group,
        group2: mpi_sys::MPI_Group,
        newgroup: *mut mpi_sys::MPI_Group,
    ) {
    }
    #[inline]
    fn post_group_difference(
        output: c_int,
        group1: mpi_sys::MPI_Group,
        group2: mpi_sys::MPI_Group,
        newgroup: *mut mpi_sys::MPI_Group,
    ) {
    }
    #[inline]
    fn pre_group_excl(
        group: mpi_sys::MPI_Group,
        n: c_int,
        ranks: *const c_int,
        newgroup: *mut mpi_sys::MPI_Group,
    ) {
    }
    #[inline]
    fn post_group_excl(
        output: c_int,
        group: mpi_sys::MPI_Group,
        n: c_int,
        ranks: *const c_int,
        newgroup: *mut mpi_sys::MPI_Group,
    ) {
    }
    #[inline]
    fn pre_group_free(group: *mut mpi_sys::MPI_Group) {}
    #[inline]
    fn post_group_free(output: c_int, group: *mut mpi_sys::MPI_Group) {}
    #[inline]
    fn pre_group_incl(
        group: mpi_sys::MPI_Group,
        n: c_int,
        ranks: *const c_int,
        newgroup: *mut mpi_sys::MPI_Group,
    ) {
    }
    #[inline]
    fn post_group_incl(
        output: c_int,
        group: mpi_sys::MPI_Group,
        n: c_int,
        ranks: *const c_int,
        newgroup: *mut mpi_sys::MPI_Group,
    ) {
    }
    #[inline]
    fn pre_group_intersection(
        group1: mpi_sys::MPI_Group,
        group2: mpi_sys::MPI_Group,
        newgroup: *mut mpi_sys::MPI_Group,
    ) {
    }
    #[inline]
    fn post_group_intersection(
        output: c_int,
        group1: mpi_sys::MPI_Group,
        group2: mpi_sys::MPI_Group,
        newgroup: *mut mpi_sys::MPI_Group,
    ) {
    }
    #[inline]
    fn pre_group_range_excl(
        group: mpi_sys::MPI_Group,
        n: c_int,
        ranges: *mut [c_int; 3],
        newgroup: *mut mpi_sys::MPI_Group,
    ) {
    }
    #[inline]
    fn post_group_range_excl(
        output: c_int,
        group: mpi_sys::MPI_Group,
        n: c_int,
        ranges: *mut [c_int; 3],
        newgroup: *mut mpi_sys::MPI_Group,
    ) {
    }
    #[inline]
    fn pre_group_range_incl(
        group: mpi_sys::MPI_Group,
        n: c_int,
        ranges: *mut [c_int; 3],
        newgroup: *mut mpi_sys::MPI_Group,
    ) {
    }
    #[inline]
    fn post_group_range_incl(
        output: c_int,
        group: mpi_sys::MPI_Group,
        n: c_int,
        ranges: *mut [c_int; 3],
        newgroup: *mut mpi_sys::MPI_Group,
    ) {
    }
    #[inline]
    fn pre_group_rank(group: mpi_sys::MPI_Group, rank: *mut c_int) {}
    #[inline]
    fn post_group_rank(output: c_int, group: mpi_sys::MPI_Group, rank: *mut c_int) {}
    #[inline]
    fn pre_group_size(group: mpi_sys::MPI_Group, size: *mut c_int) {}
    #[inline]
    fn post_group_size(output: c_int, group: mpi_sys::MPI_Group, size: *mut c_int) {}
    #[inline]
    fn pre_group_translate_ranks(
        group1: mpi_sys::MPI_Group,
        n: c_int,
        ranks1: *const c_int,
        group2: mpi_sys::MPI_Group,
        ranks2: *mut c_int,
    ) {
    }
    #[inline]
    fn post_group_translate_ranks(
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
        group1: mpi_sys::MPI_Group,
        group2: mpi_sys::MPI_Group,
        newgroup: *mut mpi_sys::MPI_Group,
    ) {
    }
    #[inline]
    fn post_group_union(
        output: c_int,
        group1: mpi_sys::MPI_Group,
        group2: mpi_sys::MPI_Group,
        newgroup: *mut mpi_sys::MPI_Group,
    ) {
    }
    #[inline]
    fn pre_iallgather(
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
    fn pre_ibarrier(comm: mpi_sys::MPI_Comm, request: *mut mpi_sys::MPI_Request) {}
    #[inline]
    fn post_ibarrier(output: c_int, comm: mpi_sys::MPI_Comm, request: *mut mpi_sys::MPI_Request) {}
    #[inline]
    fn pre_ibcast(
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
        buf: *mut c_void,
        count: c_int,
        r#type: mpi_sys::MPI_Datatype,
        message: *mut mpi_sys::MPI_Message,
        request: *mut mpi_sys::MPI_Request,
    ) {
    }
    #[inline]
    fn post_imrecv(
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
    fn pre_info_create(info: *mut mpi_sys::MPI_Info) {}
    #[inline]
    fn post_info_create(output: c_int, info: *mut mpi_sys::MPI_Info) {}
    #[inline]
    fn pre_info_delete(info: mpi_sys::MPI_Info, key: *const c_char) {}
    #[inline]
    fn post_info_delete(output: c_int, info: mpi_sys::MPI_Info, key: *const c_char) {}
    #[inline]
    fn pre_info_dup(info: mpi_sys::MPI_Info, newinfo: *mut mpi_sys::MPI_Info) {}
    #[inline]
    fn post_info_dup(output: c_int, info: mpi_sys::MPI_Info, newinfo: *mut mpi_sys::MPI_Info) {}
    #[inline]
    fn pre_info_free(info: *mut mpi_sys::MPI_Info) {}
    #[inline]
    fn post_info_free(output: c_int, info: *mut mpi_sys::MPI_Info) {}
    #[inline]
    fn pre_info_get(
        info: mpi_sys::MPI_Info,
        key: *const c_char,
        valuelen: c_int,
        value: *mut c_char,
        flag: *mut c_int,
    ) {
    }
    #[inline]
    fn post_info_get(
        output: c_int,
        info: mpi_sys::MPI_Info,
        key: *const c_char,
        valuelen: c_int,
        value: *mut c_char,
        flag: *mut c_int,
    ) {
    }
    #[inline]
    fn pre_info_get_nkeys(info: mpi_sys::MPI_Info, nkeys: *mut c_int) {}
    #[inline]
    fn post_info_get_nkeys(output: c_int, info: mpi_sys::MPI_Info, nkeys: *mut c_int) {}
    #[inline]
    fn pre_info_get_nthkey(info: mpi_sys::MPI_Info, n: c_int, key: *mut c_char) {}
    #[inline]
    fn post_info_get_nthkey(output: c_int, info: mpi_sys::MPI_Info, n: c_int, key: *mut c_char) {}
    #[inline]
    fn pre_info_get_valuelen(
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
    fn pre_info_set(info: mpi_sys::MPI_Info, key: *const c_char, value: *const c_char) {}
    #[inline]
    fn post_info_set(
        output: c_int,
        info: mpi_sys::MPI_Info,
        key: *const c_char,
        value: *const c_char,
    ) {
    }
    #[inline]
    fn pre_init(argc: *mut c_int, argv: *mut *mut *mut c_char) {}
    #[inline]
    fn post_init(output: c_int, argc: *mut c_int, argv: *mut *mut *mut c_char) {}
    #[inline]
    fn pre_init_thread(
        argc: *mut c_int,
        argv: *mut *mut *mut c_char,
        required: c_int,
        provided: *mut c_int,
    ) {
    }
    #[inline]
    fn post_init_thread(
        output: c_int,
        argc: *mut c_int,
        argv: *mut *mut *mut c_char,
        required: c_int,
        provided: *mut c_int,
    ) {
    }
    #[inline]
    fn pre_initialized(flag: *mut c_int) {}
    #[inline]
    fn post_initialized(output: c_int, flag: *mut c_int) {}
    #[inline]
    fn pre_intercomm_create(
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
        intercomm: mpi_sys::MPI_Comm,
        high: c_int,
        newintercomm: *mut mpi_sys::MPI_Comm,
    ) {
    }
    #[inline]
    fn post_intercomm_merge(
        output: c_int,
        intercomm: mpi_sys::MPI_Comm,
        high: c_int,
        newintercomm: *mut mpi_sys::MPI_Comm,
    ) {
    }
    #[inline]
    fn pre_iprobe(
        source: c_int,
        tag: c_int,
        comm: mpi_sys::MPI_Comm,
        flag: *mut c_int,
        status: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn post_iprobe(
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
    fn pre_is_thread_main(flag: *mut c_int) {}
    #[inline]
    fn post_is_thread_main(output: c_int, flag: *mut c_int) {}
    #[inline]
    fn pre_iscan(
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
        copy_fn: *mut mpi_sys::MPI_Copy_function,
        delete_fn: *mut mpi_sys::MPI_Delete_function,
        keyval: *mut c_int,
        extra_state: *mut c_void,
    ) {
    }
    #[inline]
    fn post_keyval_create(
        output: c_int,
        copy_fn: *mut mpi_sys::MPI_Copy_function,
        delete_fn: *mut mpi_sys::MPI_Delete_function,
        keyval: *mut c_int,
        extra_state: *mut c_void,
    ) {
    }
    #[inline]
    fn pre_keyval_free(keyval: *mut c_int) {}
    #[inline]
    fn post_keyval_free(output: c_int, keyval: *mut c_int) {}
    #[inline]
    fn pre_lookup_name(
        service_name: *const c_char,
        info: mpi_sys::MPI_Info,
        port_name: *mut c_char,
    ) {
    }
    #[inline]
    fn post_lookup_name(
        output: c_int,
        service_name: *const c_char,
        info: mpi_sys::MPI_Info,
        port_name: *mut c_char,
    ) {
    }
    #[inline]
    fn pre_mprobe(
        source: c_int,
        tag: c_int,
        comm: mpi_sys::MPI_Comm,
        message: *mut mpi_sys::MPI_Message,
        status: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn post_mprobe(
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
        buf: *mut c_void,
        count: c_int,
        r#type: mpi_sys::MPI_Datatype,
        message: *mut mpi_sys::MPI_Message,
        status: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn post_mrecv(
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
    fn pre_op_commutative(op: mpi_sys::MPI_Op, commute: *mut c_int) {}
    #[inline]
    fn post_op_commutative(output: c_int, op: mpi_sys::MPI_Op, commute: *mut c_int) {}
    #[inline]
    fn pre_op_create(
        function: *mut mpi_sys::MPI_User_function,
        commute: c_int,
        op: *mut mpi_sys::MPI_Op,
    ) {
    }
    #[inline]
    fn post_op_create(
        output: c_int,
        function: *mut mpi_sys::MPI_User_function,
        commute: c_int,
        op: *mut mpi_sys::MPI_Op,
    ) {
    }
    #[inline]
    fn pre_op_free(op: *mut mpi_sys::MPI_Op) {}
    #[inline]
    fn post_op_free(output: c_int, op: *mut mpi_sys::MPI_Op) {}
    #[inline]
    fn pre_open_port(info: mpi_sys::MPI_Info, port_name: *mut c_char) {}
    #[inline]
    fn post_open_port(output: c_int, info: mpi_sys::MPI_Info, port_name: *mut c_char) {}
    #[inline]
    fn pre_pack(
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
        datarep: *const c_char,
        incount: c_int,
        datatype: mpi_sys::MPI_Datatype,
        size: *mut mpi_sys::MPI_Aint,
    ) {
    }
    #[inline]
    fn post_pack_external_size(
        output: c_int,
        datarep: *const c_char,
        incount: c_int,
        datatype: mpi_sys::MPI_Datatype,
        size: *mut mpi_sys::MPI_Aint,
    ) {
    }
    #[inline]
    fn pre_pack_size(
        incount: c_int,
        datatype: mpi_sys::MPI_Datatype,
        comm: mpi_sys::MPI_Comm,
        size: *mut c_int,
    ) {
    }
    #[inline]
    fn post_pack_size(
        output: c_int,
        incount: c_int,
        datatype: mpi_sys::MPI_Datatype,
        comm: mpi_sys::MPI_Comm,
        size: *mut c_int,
    ) {
    }
    #[inline]
    fn pre_pcontrol(level: c_int) {}
    #[inline]
    fn post_pcontrol(output: c_int, level: c_int) {}
    #[inline]
    fn pre_probe(
        source: c_int,
        tag: c_int,
        comm: mpi_sys::MPI_Comm,
        status: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn post_probe(
        output: c_int,
        source: c_int,
        tag: c_int,
        comm: mpi_sys::MPI_Comm,
        status: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn pre_publish_name(
        service_name: *const c_char,
        info: mpi_sys::MPI_Info,
        port_name: *const c_char,
    ) {
    }
    #[inline]
    fn post_publish_name(
        output: c_int,
        service_name: *const c_char,
        info: mpi_sys::MPI_Info,
        port_name: *const c_char,
    ) {
    }
    #[inline]
    fn pre_put(
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
    fn pre_query_thread(provided: *mut c_int) {}
    #[inline]
    fn post_query_thread(output: c_int, provided: *mut c_int) {}
    #[inline]
    fn pre_raccumulate(
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
        inbuf: *const c_void,
        inoutbuf: *mut c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        op: mpi_sys::MPI_Op,
    ) {
    }
    #[inline]
    fn post_reduce_local(
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
        datarep: *const c_char,
        read_conversion_fn: *mut mpi_sys::MPI_Datarep_conversion_function,
        write_conversion_fn: *mut mpi_sys::MPI_Datarep_conversion_function,
        dtype_file_extent_fn: *mut mpi_sys::MPI_Datarep_extent_function,
        extra_state: *mut c_void,
    ) {
    }
    #[inline]
    fn post_register_datarep(
        output: c_int,
        datarep: *const c_char,
        read_conversion_fn: *mut mpi_sys::MPI_Datarep_conversion_function,
        write_conversion_fn: *mut mpi_sys::MPI_Datarep_conversion_function,
        dtype_file_extent_fn: *mut mpi_sys::MPI_Datarep_extent_function,
        extra_state: *mut c_void,
    ) {
    }
    #[inline]
    fn pre_request_free(request: *mut mpi_sys::MPI_Request) {}
    #[inline]
    fn post_request_free(output: c_int, request: *mut mpi_sys::MPI_Request) {}
    #[inline]
    fn pre_request_get_status(
        request: mpi_sys::MPI_Request,
        flag: *mut c_int,
        status: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn post_request_get_status(
        output: c_int,
        request: mpi_sys::MPI_Request,
        flag: *mut c_int,
        status: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn pre_rget(
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
    fn pre_start(request: *mut mpi_sys::MPI_Request) {}
    #[inline]
    fn post_start(output: c_int, request: *mut mpi_sys::MPI_Request) {}
    #[inline]
    fn pre_startall(count: c_int, array_of_requests: *mut mpi_sys::MPI_Request) {}
    #[inline]
    fn post_startall(output: c_int, count: c_int, array_of_requests: *mut mpi_sys::MPI_Request) {}
    #[inline]
    fn pre_status_set_cancelled(status: *mut mpi_sys::MPI_Status, flag: c_int) {}
    #[inline]
    fn post_status_set_cancelled(output: c_int, status: *mut mpi_sys::MPI_Status, flag: c_int) {}
    #[inline]
    fn pre_status_set_elements(
        status: *mut mpi_sys::MPI_Status,
        datatype: mpi_sys::MPI_Datatype,
        count: c_int,
    ) {
    }
    #[inline]
    fn post_status_set_elements(
        output: c_int,
        status: *mut mpi_sys::MPI_Status,
        datatype: mpi_sys::MPI_Datatype,
        count: c_int,
    ) {
    }
    #[inline]
    fn pre_status_set_elements_x(
        status: *mut mpi_sys::MPI_Status,
        datatype: mpi_sys::MPI_Datatype,
        count: mpi_sys::MPI_Count,
    ) {
    }
    #[inline]
    fn post_status_set_elements_x(
        output: c_int,
        status: *mut mpi_sys::MPI_Status,
        datatype: mpi_sys::MPI_Datatype,
        count: mpi_sys::MPI_Count,
    ) {
    }
    #[inline]
    fn pre_test(
        request: *mut mpi_sys::MPI_Request,
        flag: *mut c_int,
        status: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn post_test(
        output: c_int,
        request: *mut mpi_sys::MPI_Request,
        flag: *mut c_int,
        status: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn pre_test_cancelled(status: *const mpi_sys::MPI_Status, flag: *mut c_int) {}
    #[inline]
    fn post_test_cancelled(output: c_int, status: *const mpi_sys::MPI_Status, flag: *mut c_int) {}
    #[inline]
    fn pre_testall(
        count: c_int,
        array_of_requests: *mut mpi_sys::MPI_Request,
        flag: *mut c_int,
        array_of_statuses: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn post_testall(
        output: c_int,
        count: c_int,
        array_of_requests: *mut mpi_sys::MPI_Request,
        flag: *mut c_int,
        array_of_statuses: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn pre_testany(
        count: c_int,
        array_of_requests: *mut mpi_sys::MPI_Request,
        index: *mut c_int,
        flag: *mut c_int,
        status: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn post_testany(
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
        incount: c_int,
        array_of_requests: *mut mpi_sys::MPI_Request,
        outcount: *mut c_int,
        array_of_indices: *mut c_int,
        array_of_statuses: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn post_testsome(
        output: c_int,
        incount: c_int,
        array_of_requests: *mut mpi_sys::MPI_Request,
        outcount: *mut c_int,
        array_of_indices: *mut c_int,
        array_of_statuses: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn pre_topo_test(comm: mpi_sys::MPI_Comm, status: *mut c_int) {}
    #[inline]
    fn post_topo_test(output: c_int, comm: mpi_sys::MPI_Comm, status: *mut c_int) {}
    #[inline]
    fn pre_type_commit(r#type: *mut mpi_sys::MPI_Datatype) {}
    #[inline]
    fn post_type_commit(output: c_int, r#type: *mut mpi_sys::MPI_Datatype) {}
    #[inline]
    fn pre_type_contiguous(
        count: c_int,
        oldtype: mpi_sys::MPI_Datatype,
        newtype: *mut mpi_sys::MPI_Datatype,
    ) {
    }
    #[inline]
    fn post_type_contiguous(
        output: c_int,
        count: c_int,
        oldtype: mpi_sys::MPI_Datatype,
        newtype: *mut mpi_sys::MPI_Datatype,
    ) {
    }
    #[inline]
    fn pre_type_create_darray(
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
    fn pre_type_create_f90_complex(p: c_int, r: c_int, newtype: *mut mpi_sys::MPI_Datatype) {}
    #[inline]
    fn post_type_create_f90_complex(
        output: c_int,
        p: c_int,
        r: c_int,
        newtype: *mut mpi_sys::MPI_Datatype,
    ) {
    }
    #[inline]
    fn pre_type_create_f90_integer(r: c_int, newtype: *mut mpi_sys::MPI_Datatype) {}
    #[inline]
    fn post_type_create_f90_integer(output: c_int, r: c_int, newtype: *mut mpi_sys::MPI_Datatype) {}
    #[inline]
    fn pre_type_create_f90_real(p: c_int, r: c_int, newtype: *mut mpi_sys::MPI_Datatype) {}
    #[inline]
    fn post_type_create_f90_real(
        output: c_int,
        p: c_int,
        r: c_int,
        newtype: *mut mpi_sys::MPI_Datatype,
    ) {
    }
    #[inline]
    fn pre_type_create_hindexed(
        count: c_int,
        array_of_blocklengths: *const c_int,
        array_of_displacements: *const mpi_sys::MPI_Aint,
        oldtype: mpi_sys::MPI_Datatype,
        newtype: *mut mpi_sys::MPI_Datatype,
    ) {
    }
    #[inline]
    fn post_type_create_hindexed(
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
        count: c_int,
        blocklength: c_int,
        array_of_displacements: *const mpi_sys::MPI_Aint,
        oldtype: mpi_sys::MPI_Datatype,
        newtype: *mut mpi_sys::MPI_Datatype,
    ) {
    }
    #[inline]
    fn post_type_create_hindexed_block(
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
        count: c_int,
        blocklength: c_int,
        stride: mpi_sys::MPI_Aint,
        oldtype: mpi_sys::MPI_Datatype,
        newtype: *mut mpi_sys::MPI_Datatype,
    ) {
    }
    #[inline]
    fn post_type_create_hvector(
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
        count: c_int,
        blocklength: c_int,
        array_of_displacements: *const c_int,
        oldtype: mpi_sys::MPI_Datatype,
        newtype: *mut mpi_sys::MPI_Datatype,
    ) {
    }
    #[inline]
    fn post_type_create_indexed_block(
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
        type_copy_attr_fn: *mut mpi_sys::MPI_Type_copy_attr_function,
        type_delete_attr_fn: *mut mpi_sys::MPI_Type_delete_attr_function,
        type_keyval: *mut c_int,
        extra_state: *mut c_void,
    ) {
    }
    #[inline]
    fn post_type_create_keyval(
        output: c_int,
        type_copy_attr_fn: *mut mpi_sys::MPI_Type_copy_attr_function,
        type_delete_attr_fn: *mut mpi_sys::MPI_Type_delete_attr_function,
        type_keyval: *mut c_int,
        extra_state: *mut c_void,
    ) {
    }
    #[inline]
    fn pre_type_create_resized(
        oldtype: mpi_sys::MPI_Datatype,
        lb: mpi_sys::MPI_Aint,
        extent: mpi_sys::MPI_Aint,
        newtype: *mut mpi_sys::MPI_Datatype,
    ) {
    }
    #[inline]
    fn post_type_create_resized(
        output: c_int,
        oldtype: mpi_sys::MPI_Datatype,
        lb: mpi_sys::MPI_Aint,
        extent: mpi_sys::MPI_Aint,
        newtype: *mut mpi_sys::MPI_Datatype,
    ) {
    }
    #[inline]
    fn pre_type_create_struct(
        count: c_int,
        array_of_block_lengths: *const c_int,
        array_of_displacements: *const mpi_sys::MPI_Aint,
        array_of_types: *const mpi_sys::MPI_Datatype,
        newtype: *mut mpi_sys::MPI_Datatype,
    ) {
    }
    #[inline]
    fn post_type_create_struct(
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
    fn pre_type_delete_attr(r#type: mpi_sys::MPI_Datatype, type_keyval: c_int) {}
    #[inline]
    fn post_type_delete_attr(output: c_int, r#type: mpi_sys::MPI_Datatype, type_keyval: c_int) {}
    #[inline]
    fn pre_type_dup(r#type: mpi_sys::MPI_Datatype, newtype: *mut mpi_sys::MPI_Datatype) {}
    #[inline]
    fn post_type_dup(
        output: c_int,
        r#type: mpi_sys::MPI_Datatype,
        newtype: *mut mpi_sys::MPI_Datatype,
    ) {
    }
    #[inline]
    fn pre_type_extent(r#type: mpi_sys::MPI_Datatype, extent: *mut mpi_sys::MPI_Aint) {}
    #[inline]
    fn post_type_extent(
        output: c_int,
        r#type: mpi_sys::MPI_Datatype,
        extent: *mut mpi_sys::MPI_Aint,
    ) {
    }
    #[inline]
    fn pre_type_free(r#type: *mut mpi_sys::MPI_Datatype) {}
    #[inline]
    fn post_type_free(output: c_int, r#type: *mut mpi_sys::MPI_Datatype) {}
    #[inline]
    fn pre_type_free_keyval(type_keyval: *mut c_int) {}
    #[inline]
    fn post_type_free_keyval(output: c_int, type_keyval: *mut c_int) {}
    #[inline]
    fn pre_type_get_attr(
        r#type: mpi_sys::MPI_Datatype,
        type_keyval: c_int,
        attribute_val: *mut c_void,
        flag: *mut c_int,
    ) {
    }
    #[inline]
    fn post_type_get_attr(
        output: c_int,
        r#type: mpi_sys::MPI_Datatype,
        type_keyval: c_int,
        attribute_val: *mut c_void,
        flag: *mut c_int,
    ) {
    }
    #[inline]
    fn pre_type_get_contents(
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
        r#type: mpi_sys::MPI_Datatype,
        num_integers: *mut c_int,
        num_addresses: *mut c_int,
        num_datatypes: *mut c_int,
        combiner: *mut c_int,
    ) {
    }
    #[inline]
    fn post_type_get_envelope(
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
        r#type: mpi_sys::MPI_Datatype,
        lb: *mut mpi_sys::MPI_Aint,
        extent: *mut mpi_sys::MPI_Aint,
    ) {
    }
    #[inline]
    fn post_type_get_extent(
        output: c_int,
        r#type: mpi_sys::MPI_Datatype,
        lb: *mut mpi_sys::MPI_Aint,
        extent: *mut mpi_sys::MPI_Aint,
    ) {
    }
    #[inline]
    fn pre_type_get_extent_x(
        r#type: mpi_sys::MPI_Datatype,
        lb: *mut mpi_sys::MPI_Count,
        extent: *mut mpi_sys::MPI_Count,
    ) {
    }
    #[inline]
    fn post_type_get_extent_x(
        output: c_int,
        r#type: mpi_sys::MPI_Datatype,
        lb: *mut mpi_sys::MPI_Count,
        extent: *mut mpi_sys::MPI_Count,
    ) {
    }
    #[inline]
    fn pre_type_get_name(
        r#type: mpi_sys::MPI_Datatype,
        type_name: *mut c_char,
        resultlen: *mut c_int,
    ) {
    }
    #[inline]
    fn post_type_get_name(
        output: c_int,
        r#type: mpi_sys::MPI_Datatype,
        type_name: *mut c_char,
        resultlen: *mut c_int,
    ) {
    }
    #[inline]
    fn pre_type_get_true_extent(
        datatype: mpi_sys::MPI_Datatype,
        true_lb: *mut mpi_sys::MPI_Aint,
        true_extent: *mut mpi_sys::MPI_Aint,
    ) {
    }
    #[inline]
    fn post_type_get_true_extent(
        output: c_int,
        datatype: mpi_sys::MPI_Datatype,
        true_lb: *mut mpi_sys::MPI_Aint,
        true_extent: *mut mpi_sys::MPI_Aint,
    ) {
    }
    #[inline]
    fn pre_type_get_true_extent_x(
        datatype: mpi_sys::MPI_Datatype,
        true_lb: *mut mpi_sys::MPI_Count,
        true_extent: *mut mpi_sys::MPI_Count,
    ) {
    }
    #[inline]
    fn post_type_get_true_extent_x(
        output: c_int,
        datatype: mpi_sys::MPI_Datatype,
        true_lb: *mut mpi_sys::MPI_Count,
        true_extent: *mut mpi_sys::MPI_Count,
    ) {
    }
    #[inline]
    fn pre_type_hindexed(
        count: c_int,
        array_of_blocklengths: *mut c_int,
        array_of_displacements: *mut mpi_sys::MPI_Aint,
        oldtype: mpi_sys::MPI_Datatype,
        newtype: *mut mpi_sys::MPI_Datatype,
    ) {
    }
    #[inline]
    fn post_type_hindexed(
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
        count: c_int,
        blocklength: c_int,
        stride: mpi_sys::MPI_Aint,
        oldtype: mpi_sys::MPI_Datatype,
        newtype: *mut mpi_sys::MPI_Datatype,
    ) {
    }
    #[inline]
    fn post_type_hvector(
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
        count: c_int,
        array_of_blocklengths: *const c_int,
        array_of_displacements: *const c_int,
        oldtype: mpi_sys::MPI_Datatype,
        newtype: *mut mpi_sys::MPI_Datatype,
    ) {
    }
    #[inline]
    fn post_type_indexed(
        output: c_int,
        count: c_int,
        array_of_blocklengths: *const c_int,
        array_of_displacements: *const c_int,
        oldtype: mpi_sys::MPI_Datatype,
        newtype: *mut mpi_sys::MPI_Datatype,
    ) {
    }
    #[inline]
    fn pre_type_lb(r#type: mpi_sys::MPI_Datatype, lb: *mut mpi_sys::MPI_Aint) {}
    #[inline]
    fn post_type_lb(output: c_int, r#type: mpi_sys::MPI_Datatype, lb: *mut mpi_sys::MPI_Aint) {}
    #[inline]
    fn pre_type_match_size(typeclass: c_int, size: c_int, r#type: *mut mpi_sys::MPI_Datatype) {}
    #[inline]
    fn post_type_match_size(
        output: c_int,
        typeclass: c_int,
        size: c_int,
        r#type: *mut mpi_sys::MPI_Datatype,
    ) {
    }
    #[inline]
    fn pre_type_set_attr(r#type: mpi_sys::MPI_Datatype, type_keyval: c_int, attr_val: *mut c_void) {
    }
    #[inline]
    fn post_type_set_attr(
        output: c_int,
        r#type: mpi_sys::MPI_Datatype,
        type_keyval: c_int,
        attr_val: *mut c_void,
    ) {
    }
    #[inline]
    fn pre_type_set_name(r#type: mpi_sys::MPI_Datatype, type_name: *const c_char) {}
    #[inline]
    fn post_type_set_name(output: c_int, r#type: mpi_sys::MPI_Datatype, type_name: *const c_char) {}
    #[inline]
    fn pre_type_size(r#type: mpi_sys::MPI_Datatype, size: *mut c_int) {}
    #[inline]
    fn post_type_size(output: c_int, r#type: mpi_sys::MPI_Datatype, size: *mut c_int) {}
    #[inline]
    fn pre_type_size_x(r#type: mpi_sys::MPI_Datatype, size: *mut mpi_sys::MPI_Count) {}
    #[inline]
    fn post_type_size_x(
        output: c_int,
        r#type: mpi_sys::MPI_Datatype,
        size: *mut mpi_sys::MPI_Count,
    ) {
    }
    #[inline]
    fn pre_type_struct(
        count: c_int,
        array_of_blocklengths: *mut c_int,
        array_of_displacements: *mut mpi_sys::MPI_Aint,
        array_of_types: *mut mpi_sys::MPI_Datatype,
        newtype: *mut mpi_sys::MPI_Datatype,
    ) {
    }
    #[inline]
    fn post_type_struct(
        output: c_int,
        count: c_int,
        array_of_blocklengths: *mut c_int,
        array_of_displacements: *mut mpi_sys::MPI_Aint,
        array_of_types: *mut mpi_sys::MPI_Datatype,
        newtype: *mut mpi_sys::MPI_Datatype,
    ) {
    }
    #[inline]
    fn pre_type_ub(mtype: mpi_sys::MPI_Datatype, ub: *mut mpi_sys::MPI_Aint) {}
    #[inline]
    fn post_type_ub(output: c_int, mtype: mpi_sys::MPI_Datatype, ub: *mut mpi_sys::MPI_Aint) {}
    #[inline]
    fn pre_type_vector(
        count: c_int,
        blocklength: c_int,
        stride: c_int,
        oldtype: mpi_sys::MPI_Datatype,
        newtype: *mut mpi_sys::MPI_Datatype,
    ) {
    }
    #[inline]
    fn post_type_vector(
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
        service_name: *const c_char,
        info: mpi_sys::MPI_Info,
        port_name: *const c_char,
    ) {
    }
    #[inline]
    fn post_unpublish_name(
        output: c_int,
        service_name: *const c_char,
        info: mpi_sys::MPI_Info,
        port_name: *const c_char,
    ) {
    }
    #[inline]
    fn pre_wait(request: *mut mpi_sys::MPI_Request, status: *mut mpi_sys::MPI_Status) {}
    #[inline]
    fn post_wait(
        output: c_int,
        request: *mut mpi_sys::MPI_Request,
        status: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn pre_waitall(
        count: c_int,
        array_of_requests: *mut mpi_sys::MPI_Request,
        array_of_statuses: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn post_waitall(
        output: c_int,
        count: c_int,
        array_of_requests: *mut mpi_sys::MPI_Request,
        array_of_statuses: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn pre_waitany(
        count: c_int,
        array_of_requests: *mut mpi_sys::MPI_Request,
        index: *mut c_int,
        status: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn post_waitany(
        output: c_int,
        count: c_int,
        array_of_requests: *mut mpi_sys::MPI_Request,
        index: *mut c_int,
        status: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn pre_waitsome(
        incount: c_int,
        array_of_requests: *mut mpi_sys::MPI_Request,
        outcount: *mut c_int,
        array_of_indices: *mut c_int,
        array_of_statuses: *mut mpi_sys::MPI_Status,
    ) {
    }
    #[inline]
    fn post_waitsome(
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
    fn pre_win_attach(win: mpi_sys::MPI_Win, base: *mut c_void, size: mpi_sys::MPI_Aint) {}
    #[inline]
    fn post_win_attach(
        output: c_int,
        win: mpi_sys::MPI_Win,
        base: *mut c_void,
        size: mpi_sys::MPI_Aint,
    ) {
    }
    #[inline]
    fn pre_win_call_errhandler(win: mpi_sys::MPI_Win, errorcode: c_int) {}
    #[inline]
    fn post_win_call_errhandler(output: c_int, win: mpi_sys::MPI_Win, errorcode: c_int) {}
    #[inline]
    fn pre_win_complete(win: mpi_sys::MPI_Win) {}
    #[inline]
    fn post_win_complete(output: c_int, win: mpi_sys::MPI_Win) {}
    #[inline]
    fn pre_win_create(
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
        info: mpi_sys::MPI_Info,
        comm: mpi_sys::MPI_Comm,
        win: *mut mpi_sys::MPI_Win,
    ) {
    }
    #[inline]
    fn post_win_create_dynamic(
        output: c_int,
        info: mpi_sys::MPI_Info,
        comm: mpi_sys::MPI_Comm,
        win: *mut mpi_sys::MPI_Win,
    ) {
    }
    #[inline]
    fn pre_win_create_errhandler(
        function: *mut mpi_sys::MPI_Win_errhandler_function,
        errhandler: *mut mpi_sys::MPI_Errhandler,
    ) {
    }
    #[inline]
    fn post_win_create_errhandler(
        output: c_int,
        function: *mut mpi_sys::MPI_Win_errhandler_function,
        errhandler: *mut mpi_sys::MPI_Errhandler,
    ) {
    }
    #[inline]
    fn pre_win_create_keyval(
        win_copy_attr_fn: *mut mpi_sys::MPI_Win_copy_attr_function,
        win_delete_attr_fn: *mut mpi_sys::MPI_Win_delete_attr_function,
        win_keyval: *mut c_int,
        extra_state: *mut c_void,
    ) {
    }
    #[inline]
    fn post_win_create_keyval(
        output: c_int,
        win_copy_attr_fn: *mut mpi_sys::MPI_Win_copy_attr_function,
        win_delete_attr_fn: *mut mpi_sys::MPI_Win_delete_attr_function,
        win_keyval: *mut c_int,
        extra_state: *mut c_void,
    ) {
    }
    #[inline]
    fn pre_win_delete_attr(win: mpi_sys::MPI_Win, win_keyval: c_int) {}
    #[inline]
    fn post_win_delete_attr(output: c_int, win: mpi_sys::MPI_Win, win_keyval: c_int) {}
    #[inline]
    fn pre_win_detach(win: mpi_sys::MPI_Win, base: *const c_void) {}
    #[inline]
    fn post_win_detach(output: c_int, win: mpi_sys::MPI_Win, base: *const c_void) {}
    #[inline]
    fn pre_win_fence(assert: c_int, win: mpi_sys::MPI_Win) {}
    #[inline]
    fn post_win_fence(output: c_int, assert: c_int, win: mpi_sys::MPI_Win) {}
    #[inline]
    fn pre_win_flush(rank: c_int, win: mpi_sys::MPI_Win) {}
    #[inline]
    fn post_win_flush(output: c_int, rank: c_int, win: mpi_sys::MPI_Win) {}
    #[inline]
    fn pre_win_flush_all(win: mpi_sys::MPI_Win) {}
    #[inline]
    fn post_win_flush_all(output: c_int, win: mpi_sys::MPI_Win) {}
    #[inline]
    fn pre_win_flush_local(rank: c_int, win: mpi_sys::MPI_Win) {}
    #[inline]
    fn post_win_flush_local(output: c_int, rank: c_int, win: mpi_sys::MPI_Win) {}
    #[inline]
    fn pre_win_flush_local_all(win: mpi_sys::MPI_Win) {}
    #[inline]
    fn post_win_flush_local_all(output: c_int, win: mpi_sys::MPI_Win) {}
    #[inline]
    fn pre_win_free(win: *mut mpi_sys::MPI_Win) {}
    #[inline]
    fn post_win_free(output: c_int, win: *mut mpi_sys::MPI_Win) {}
    #[inline]
    fn pre_win_free_keyval(win_keyval: *mut c_int) {}
    #[inline]
    fn post_win_free_keyval(output: c_int, win_keyval: *mut c_int) {}
    #[inline]
    fn pre_win_get_attr(
        win: mpi_sys::MPI_Win,
        win_keyval: c_int,
        attribute_val: *mut c_void,
        flag: *mut c_int,
    ) {
    }
    #[inline]
    fn post_win_get_attr(
        output: c_int,
        win: mpi_sys::MPI_Win,
        win_keyval: c_int,
        attribute_val: *mut c_void,
        flag: *mut c_int,
    ) {
    }
    #[inline]
    fn pre_win_get_errhandler(win: mpi_sys::MPI_Win, errhandler: *mut mpi_sys::MPI_Errhandler) {}
    #[inline]
    fn post_win_get_errhandler(
        output: c_int,
        win: mpi_sys::MPI_Win,
        errhandler: *mut mpi_sys::MPI_Errhandler,
    ) {
    }
    #[inline]
    fn pre_win_get_group(win: mpi_sys::MPI_Win, group: *mut mpi_sys::MPI_Group) {}
    #[inline]
    fn post_win_get_group(output: c_int, win: mpi_sys::MPI_Win, group: *mut mpi_sys::MPI_Group) {}
    #[inline]
    fn pre_win_get_info(win: mpi_sys::MPI_Win, info_used: *mut mpi_sys::MPI_Info) {}
    #[inline]
    fn post_win_get_info(output: c_int, win: mpi_sys::MPI_Win, info_used: *mut mpi_sys::MPI_Info) {}
    #[inline]
    fn pre_win_get_name(win: mpi_sys::MPI_Win, win_name: *mut c_char, resultlen: *mut c_int) {}
    #[inline]
    fn post_win_get_name(
        output: c_int,
        win: mpi_sys::MPI_Win,
        win_name: *mut c_char,
        resultlen: *mut c_int,
    ) {
    }
    #[inline]
    fn pre_win_lock(lock_type: c_int, rank: c_int, assert: c_int, win: mpi_sys::MPI_Win) {}
    #[inline]
    fn post_win_lock(
        output: c_int,
        lock_type: c_int,
        rank: c_int,
        assert: c_int,
        win: mpi_sys::MPI_Win,
    ) {
    }
    #[inline]
    fn pre_win_lock_all(assert: c_int, win: mpi_sys::MPI_Win) {}
    #[inline]
    fn post_win_lock_all(output: c_int, assert: c_int, win: mpi_sys::MPI_Win) {}
    #[inline]
    fn pre_win_post(group: mpi_sys::MPI_Group, assert: c_int, win: mpi_sys::MPI_Win) {}
    #[inline]
    fn post_win_post(
        output: c_int,
        group: mpi_sys::MPI_Group,
        assert: c_int,
        win: mpi_sys::MPI_Win,
    ) {
    }
    #[inline]
    fn pre_win_set_attr(win: mpi_sys::MPI_Win, win_keyval: c_int, attribute_val: *mut c_void) {}
    #[inline]
    fn post_win_set_attr(
        output: c_int,
        win: mpi_sys::MPI_Win,
        win_keyval: c_int,
        attribute_val: *mut c_void,
    ) {
    }
    #[inline]
    fn pre_win_set_errhandler(win: mpi_sys::MPI_Win, errhandler: mpi_sys::MPI_Errhandler) {}
    #[inline]
    fn post_win_set_errhandler(
        output: c_int,
        win: mpi_sys::MPI_Win,
        errhandler: mpi_sys::MPI_Errhandler,
    ) {
    }
    #[inline]
    fn pre_win_set_info(win: mpi_sys::MPI_Win, info: mpi_sys::MPI_Info) {}
    #[inline]
    fn post_win_set_info(output: c_int, win: mpi_sys::MPI_Win, info: mpi_sys::MPI_Info) {}
    #[inline]
    fn pre_win_set_name(win: mpi_sys::MPI_Win, win_name: *const c_char) {}
    #[inline]
    fn post_win_set_name(output: c_int, win: mpi_sys::MPI_Win, win_name: *const c_char) {}
    #[inline]
    fn pre_win_shared_query(
        win: mpi_sys::MPI_Win,
        rank: c_int,
        size: *mut mpi_sys::MPI_Aint,
        disp_unit: *mut c_int,
        baseptr: *mut c_void,
    ) {
    }
    #[inline]
    fn post_win_shared_query(
        output: c_int,
        win: mpi_sys::MPI_Win,
        rank: c_int,
        size: *mut mpi_sys::MPI_Aint,
        disp_unit: *mut c_int,
        baseptr: *mut c_void,
    ) {
    }
    #[inline]
    fn pre_win_start(group: mpi_sys::MPI_Group, assert: c_int, win: mpi_sys::MPI_Win) {}
    #[inline]
    fn post_win_start(
        output: c_int,
        group: mpi_sys::MPI_Group,
        assert: c_int,
        win: mpi_sys::MPI_Win,
    ) {
    }
    #[inline]
    fn pre_win_sync(win: mpi_sys::MPI_Win) {}
    #[inline]
    fn post_win_sync(output: c_int, win: mpi_sys::MPI_Win) {}
    #[inline]
    fn pre_win_test(win: mpi_sys::MPI_Win, flag: *mut c_int) {}
    #[inline]
    fn post_win_test(output: c_int, win: mpi_sys::MPI_Win, flag: *mut c_int) {}
    #[inline]
    fn pre_win_unlock(rank: c_int, win: mpi_sys::MPI_Win) {}
    #[inline]
    fn post_win_unlock(output: c_int, rank: c_int, win: mpi_sys::MPI_Win) {}
    #[inline]
    fn pre_win_unlock_all(win: mpi_sys::MPI_Win) {}
    #[inline]
    fn post_win_unlock_all(output: c_int, win: mpi_sys::MPI_Win) {}
    #[inline]
    fn pre_win_wait(win: mpi_sys::MPI_Win) {}
    #[inline]
    fn post_win_wait(output: c_int, win: mpi_sys::MPI_Win) {}
    #[inline]
    fn pre_wtick() {}
    #[inline]
    fn post_wtick(output: c_double) {}
    #[inline]
    fn pre_wtime() {}
    #[inline]
    fn post_wtime(output: c_double) {}
}
