use std::os::raw::{c_char, c_double, c_int, c_void};

use qmpi_sys::mpi_sys;

#[allow(unused_variables)]
pub trait QmpiLayer {
    #[inline]
    fn abort<F>(&self, next_f: F, comm: mpi_sys::MPI_Comm, errorcode: c_int) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Comm, c_int) -> c_int,
    {
        next_f(comm, errorcode)
    }
    #[inline]
    fn accumulate<F>(
        &self,
        next_f: F,
        origin_addr: *const c_void,
        origin_count: c_int,
        origin_datatype: mpi_sys::MPI_Datatype,
        target_rank: c_int,
        target_disp: mpi_sys::MPI_Aint,
        target_count: c_int,
        target_datatype: mpi_sys::MPI_Datatype,
        op: mpi_sys::MPI_Op,
        win: mpi_sys::MPI_Win,
    ) -> c_int
    where
        F: FnOnce(
            *const c_void,
            c_int,
            mpi_sys::MPI_Datatype,
            c_int,
            mpi_sys::MPI_Aint,
            c_int,
            mpi_sys::MPI_Datatype,
            mpi_sys::MPI_Op,
            mpi_sys::MPI_Win,
        ) -> c_int,
    {
        next_f(
            origin_addr,
            origin_count,
            origin_datatype,
            target_rank,
            target_disp,
            target_count,
            target_datatype,
            op,
            win,
        )
    }
    #[inline]
    fn add_error_class<F>(&self, next_f: F, errorclass: *mut c_int) -> c_int
    where
        F: FnOnce(*mut c_int) -> c_int,
    {
        next_f(errorclass)
    }
    #[inline]
    fn add_error_code<F>(&self, next_f: F, errorclass: c_int, errorcode: *mut c_int) -> c_int
    where
        F: FnOnce(c_int, *mut c_int) -> c_int,
    {
        next_f(errorclass, errorcode)
    }
    #[inline]
    fn add_error_string<F>(&self, next_f: F, errorcode: c_int, string: *const c_char) -> c_int
    where
        F: FnOnce(c_int, *const c_char) -> c_int,
    {
        next_f(errorcode, string)
    }
    #[inline]
    fn address<F>(&self, next_f: F, location: *mut c_void, address: *mut mpi_sys::MPI_Aint) -> c_int
    where
        F: FnOnce(*mut c_void, *mut mpi_sys::MPI_Aint) -> c_int,
    {
        next_f(location, address)
    }
    #[inline]
    fn allgather<F>(
        &self,
        next_f: F,
        sendbuf: *const c_void,
        sendcount: c_int,
        sendtype: mpi_sys::MPI_Datatype,
        recvbuf: *mut c_void,
        recvcount: c_int,
        recvtype: mpi_sys::MPI_Datatype,
        comm: mpi_sys::MPI_Comm,
    ) -> c_int
    where
        F: FnOnce(
            *const c_void,
            c_int,
            mpi_sys::MPI_Datatype,
            *mut c_void,
            c_int,
            mpi_sys::MPI_Datatype,
            mpi_sys::MPI_Comm,
        ) -> c_int,
    {
        next_f(
            sendbuf, sendcount, sendtype, recvbuf, recvcount, recvtype, comm,
        )
    }
    #[inline]
    fn allgatherv<F>(
        &self,
        next_f: F,
        sendbuf: *const c_void,
        sendcount: c_int,
        sendtype: mpi_sys::MPI_Datatype,
        recvbuf: *mut c_void,
        recvcounts: *const c_int,
        displs: *const c_int,
        recvtype: mpi_sys::MPI_Datatype,
        comm: mpi_sys::MPI_Comm,
    ) -> c_int
    where
        F: FnOnce(
            *const c_void,
            c_int,
            mpi_sys::MPI_Datatype,
            *mut c_void,
            *const c_int,
            *const c_int,
            mpi_sys::MPI_Datatype,
            mpi_sys::MPI_Comm,
        ) -> c_int,
    {
        next_f(
            sendbuf, sendcount, sendtype, recvbuf, recvcounts, displs, recvtype, comm,
        )
    }
    #[inline]
    fn alloc_mem<F>(
        &self,
        next_f: F,
        size: mpi_sys::MPI_Aint,
        info: mpi_sys::MPI_Info,
        baseptr: *mut c_void,
    ) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Aint, mpi_sys::MPI_Info, *mut c_void) -> c_int,
    {
        next_f(size, info, baseptr)
    }
    #[inline]
    fn allreduce<F>(
        &self,
        next_f: F,
        sendbuf: *const c_void,
        recvbuf: *mut c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        op: mpi_sys::MPI_Op,
        comm: mpi_sys::MPI_Comm,
    ) -> c_int
    where
        F: FnOnce(
            *const c_void,
            *mut c_void,
            c_int,
            mpi_sys::MPI_Datatype,
            mpi_sys::MPI_Op,
            mpi_sys::MPI_Comm,
        ) -> c_int,
    {
        next_f(sendbuf, recvbuf, count, datatype, op, comm)
    }
    #[inline]
    fn alltoall<F>(
        &self,
        next_f: F,
        sendbuf: *const c_void,
        sendcount: c_int,
        sendtype: mpi_sys::MPI_Datatype,
        recvbuf: *mut c_void,
        recvcount: c_int,
        recvtype: mpi_sys::MPI_Datatype,
        comm: mpi_sys::MPI_Comm,
    ) -> c_int
    where
        F: FnOnce(
            *const c_void,
            c_int,
            mpi_sys::MPI_Datatype,
            *mut c_void,
            c_int,
            mpi_sys::MPI_Datatype,
            mpi_sys::MPI_Comm,
        ) -> c_int,
    {
        next_f(
            sendbuf, sendcount, sendtype, recvbuf, recvcount, recvtype, comm,
        )
    }
    #[inline]
    fn alltoallv<F>(
        &self,
        next_f: F,
        sendbuf: *const c_void,
        sendcounts: *const c_int,
        sdispls: *const c_int,
        sendtype: mpi_sys::MPI_Datatype,
        recvbuf: *mut c_void,
        recvcounts: *const c_int,
        rdispls: *const c_int,
        recvtype: mpi_sys::MPI_Datatype,
        comm: mpi_sys::MPI_Comm,
    ) -> c_int
    where
        F: FnOnce(
            *const c_void,
            *const c_int,
            *const c_int,
            mpi_sys::MPI_Datatype,
            *mut c_void,
            *const c_int,
            *const c_int,
            mpi_sys::MPI_Datatype,
            mpi_sys::MPI_Comm,
        ) -> c_int,
    {
        next_f(
            sendbuf, sendcounts, sdispls, sendtype, recvbuf, recvcounts, rdispls, recvtype, comm,
        )
    }
    #[inline]
    fn alltoallw<F>(
        &self,
        next_f: F,
        sendbuf: *const c_void,
        sendcounts: *const c_int,
        sdispls: *const c_int,
        sendtypes: *const mpi_sys::MPI_Datatype,
        recvbuf: *mut c_void,
        recvcounts: *const c_int,
        rdispls: *const c_int,
        recvtypes: *const mpi_sys::MPI_Datatype,
        comm: mpi_sys::MPI_Comm,
    ) -> c_int
    where
        F: FnOnce(
            *const c_void,
            *const c_int,
            *const c_int,
            *const mpi_sys::MPI_Datatype,
            *mut c_void,
            *const c_int,
            *const c_int,
            *const mpi_sys::MPI_Datatype,
            mpi_sys::MPI_Comm,
        ) -> c_int,
    {
        next_f(
            sendbuf, sendcounts, sdispls, sendtypes, recvbuf, recvcounts, rdispls, recvtypes, comm,
        )
    }
    #[inline]
    fn attr_delete<F>(&self, next_f: F, comm: mpi_sys::MPI_Comm, keyval: c_int) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Comm, c_int) -> c_int,
    {
        next_f(comm, keyval)
    }
    #[inline]
    fn attr_get<F>(
        &self,
        next_f: F,
        comm: mpi_sys::MPI_Comm,
        keyval: c_int,
        attribute_val: *mut c_void,
        flag: *mut c_int,
    ) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Comm, c_int, *mut c_void, *mut c_int) -> c_int,
    {
        next_f(comm, keyval, attribute_val, flag)
    }
    #[inline]
    fn attr_put<F>(
        &self,
        next_f: F,
        comm: mpi_sys::MPI_Comm,
        keyval: c_int,
        attribute_val: *mut c_void,
    ) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Comm, c_int, *mut c_void) -> c_int,
    {
        next_f(comm, keyval, attribute_val)
    }
    #[inline]
    fn barrier<F>(&self, next_f: F, comm: mpi_sys::MPI_Comm) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Comm) -> c_int,
    {
        next_f(comm)
    }
    #[inline]
    fn bcast<F>(
        &self,
        next_f: F,
        buffer: *mut c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        root: c_int,
        comm: mpi_sys::MPI_Comm,
    ) -> c_int
    where
        F: FnOnce(*mut c_void, c_int, mpi_sys::MPI_Datatype, c_int, mpi_sys::MPI_Comm) -> c_int,
    {
        next_f(buffer, count, datatype, root, comm)
    }
    #[inline]
    fn bsend<F>(
        &self,
        next_f: F,
        buf: *const c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        dest: c_int,
        tag: c_int,
        comm: mpi_sys::MPI_Comm,
    ) -> c_int
    where
        F: FnOnce(
            *const c_void,
            c_int,
            mpi_sys::MPI_Datatype,
            c_int,
            c_int,
            mpi_sys::MPI_Comm,
        ) -> c_int,
    {
        next_f(buf, count, datatype, dest, tag, comm)
    }
    #[inline]
    fn bsend_init<F>(
        &self,
        next_f: F,
        buf: *const c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        dest: c_int,
        tag: c_int,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) -> c_int
    where
        F: FnOnce(
            *const c_void,
            c_int,
            mpi_sys::MPI_Datatype,
            c_int,
            c_int,
            mpi_sys::MPI_Comm,
            *mut mpi_sys::MPI_Request,
        ) -> c_int,
    {
        next_f(buf, count, datatype, dest, tag, comm, request)
    }
    #[inline]
    fn buffer_attach<F>(&self, next_f: F, buffer: *mut c_void, size: c_int) -> c_int
    where
        F: FnOnce(*mut c_void, c_int) -> c_int,
    {
        next_f(buffer, size)
    }
    #[inline]
    fn buffer_detach<F>(&self, next_f: F, buffer: *mut c_void, size: *mut c_int) -> c_int
    where
        F: FnOnce(*mut c_void, *mut c_int) -> c_int,
    {
        next_f(buffer, size)
    }
    #[inline]
    fn cancel<F>(&self, next_f: F, request: *mut mpi_sys::MPI_Request) -> c_int
    where
        F: FnOnce(*mut mpi_sys::MPI_Request) -> c_int,
    {
        next_f(request)
    }
    #[inline]
    fn cart_coords<F>(
        &self,
        next_f: F,
        comm: mpi_sys::MPI_Comm,
        rank: c_int,
        maxdims: c_int,
        coords: *mut c_int,
    ) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Comm, c_int, c_int, *mut c_int) -> c_int,
    {
        next_f(comm, rank, maxdims, coords)
    }
    #[inline]
    fn cart_create<F>(
        &self,
        next_f: F,
        old_comm: mpi_sys::MPI_Comm,
        ndims: c_int,
        dims: *const c_int,
        periods: *const c_int,
        reorder: c_int,
        comm_cart: *mut mpi_sys::MPI_Comm,
    ) -> c_int
    where
        F: FnOnce(
            mpi_sys::MPI_Comm,
            c_int,
            *const c_int,
            *const c_int,
            c_int,
            *mut mpi_sys::MPI_Comm,
        ) -> c_int,
    {
        next_f(old_comm, ndims, dims, periods, reorder, comm_cart)
    }
    #[inline]
    fn cart_get<F>(
        &self,
        next_f: F,
        comm: mpi_sys::MPI_Comm,
        maxdims: c_int,
        dims: *mut c_int,
        periods: *mut c_int,
        coords: *mut c_int,
    ) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Comm, c_int, *mut c_int, *mut c_int, *mut c_int) -> c_int,
    {
        next_f(comm, maxdims, dims, periods, coords)
    }
    #[inline]
    fn cart_map<F>(
        &self,
        next_f: F,
        comm: mpi_sys::MPI_Comm,
        ndims: c_int,
        dims: *const c_int,
        periods: *const c_int,
        newrank: *mut c_int,
    ) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Comm, c_int, *const c_int, *const c_int, *mut c_int) -> c_int,
    {
        next_f(comm, ndims, dims, periods, newrank)
    }
    #[inline]
    fn cart_rank<F>(
        &self,
        next_f: F,
        comm: mpi_sys::MPI_Comm,
        coords: *const c_int,
        rank: *mut c_int,
    ) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Comm, *const c_int, *mut c_int) -> c_int,
    {
        next_f(comm, coords, rank)
    }
    #[inline]
    fn cart_shift<F>(
        &self,
        next_f: F,
        comm: mpi_sys::MPI_Comm,
        direction: c_int,
        disp: c_int,
        rank_source: *mut c_int,
        rank_dest: *mut c_int,
    ) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Comm, c_int, c_int, *mut c_int, *mut c_int) -> c_int,
    {
        next_f(comm, direction, disp, rank_source, rank_dest)
    }
    #[inline]
    fn cart_sub<F>(
        &self,
        next_f: F,
        comm: mpi_sys::MPI_Comm,
        remain_dims: *const c_int,
        new_comm: *mut mpi_sys::MPI_Comm,
    ) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Comm, *const c_int, *mut mpi_sys::MPI_Comm) -> c_int,
    {
        next_f(comm, remain_dims, new_comm)
    }
    #[inline]
    fn cartdim_get<F>(&self, next_f: F, comm: mpi_sys::MPI_Comm, ndims: *mut c_int) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Comm, *mut c_int) -> c_int,
    {
        next_f(comm, ndims)
    }
    #[inline]
    fn close_port<F>(&self, next_f: F, port_name: *const c_char) -> c_int
    where
        F: FnOnce(*const c_char) -> c_int,
    {
        next_f(port_name)
    }
    #[inline]
    fn comm_accept<F>(
        &self,
        next_f: F,
        port_name: *const c_char,
        info: mpi_sys::MPI_Info,
        root: c_int,
        comm: mpi_sys::MPI_Comm,
        newcomm: *mut mpi_sys::MPI_Comm,
    ) -> c_int
    where
        F: FnOnce(
            *const c_char,
            mpi_sys::MPI_Info,
            c_int,
            mpi_sys::MPI_Comm,
            *mut mpi_sys::MPI_Comm,
        ) -> c_int,
    {
        next_f(port_name, info, root, comm, newcomm)
    }
    #[inline]
    fn comm_call_errhandler<F>(&self, next_f: F, comm: mpi_sys::MPI_Comm, errorcode: c_int) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Comm, c_int) -> c_int,
    {
        next_f(comm, errorcode)
    }
    #[inline]
    fn comm_compare<F>(
        &self,
        next_f: F,
        comm1: mpi_sys::MPI_Comm,
        comm2: mpi_sys::MPI_Comm,
        result: *mut c_int,
    ) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Comm, mpi_sys::MPI_Comm, *mut c_int) -> c_int,
    {
        next_f(comm1, comm2, result)
    }
    #[inline]
    fn comm_connect<F>(
        &self,
        next_f: F,
        port_name: *const c_char,
        info: mpi_sys::MPI_Info,
        root: c_int,
        comm: mpi_sys::MPI_Comm,
        newcomm: *mut mpi_sys::MPI_Comm,
    ) -> c_int
    where
        F: FnOnce(
            *const c_char,
            mpi_sys::MPI_Info,
            c_int,
            mpi_sys::MPI_Comm,
            *mut mpi_sys::MPI_Comm,
        ) -> c_int,
    {
        next_f(port_name, info, root, comm, newcomm)
    }
    #[inline]
    fn comm_create<F>(
        &self,
        next_f: F,
        comm: mpi_sys::MPI_Comm,
        group: mpi_sys::MPI_Group,
        newcomm: *mut mpi_sys::MPI_Comm,
    ) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Comm, mpi_sys::MPI_Group, *mut mpi_sys::MPI_Comm) -> c_int,
    {
        next_f(comm, group, newcomm)
    }
    #[inline]
    fn comm_create_errhandler<F>(
        &self,
        next_f: F,
        function: *mut mpi_sys::MPI_Comm_errhandler_function,
        errhandler: *mut mpi_sys::MPI_Errhandler,
    ) -> c_int
    where
        F: FnOnce(
            *mut mpi_sys::MPI_Comm_errhandler_function,
            *mut mpi_sys::MPI_Errhandler,
        ) -> c_int,
    {
        next_f(function, errhandler)
    }
    #[inline]
    fn comm_create_group<F>(
        &self,
        next_f: F,
        comm: mpi_sys::MPI_Comm,
        group: mpi_sys::MPI_Group,
        tag: c_int,
        newcomm: *mut mpi_sys::MPI_Comm,
    ) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Comm, mpi_sys::MPI_Group, c_int, *mut mpi_sys::MPI_Comm) -> c_int,
    {
        next_f(comm, group, tag, newcomm)
    }
    #[inline]
    fn comm_create_keyval<F>(
        &self,
        next_f: F,
        comm_copy_attr_fn: *mut mpi_sys::MPI_Comm_copy_attr_function,
        comm_delete_attr_fn: *mut mpi_sys::MPI_Comm_delete_attr_function,
        comm_keyval: *mut c_int,
        extra_state: *mut c_void,
    ) -> c_int
    where
        F: FnOnce(
            *mut mpi_sys::MPI_Comm_copy_attr_function,
            *mut mpi_sys::MPI_Comm_delete_attr_function,
            *mut c_int,
            *mut c_void,
        ) -> c_int,
    {
        next_f(
            comm_copy_attr_fn,
            comm_delete_attr_fn,
            comm_keyval,
            extra_state,
        )
    }
    #[inline]
    fn comm_delete_attr<F>(&self, next_f: F, comm: mpi_sys::MPI_Comm, comm_keyval: c_int) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Comm, c_int) -> c_int,
    {
        next_f(comm, comm_keyval)
    }
    #[inline]
    fn comm_disconnect<F>(&self, next_f: F, comm: *mut mpi_sys::MPI_Comm) -> c_int
    where
        F: FnOnce(*mut mpi_sys::MPI_Comm) -> c_int,
    {
        next_f(comm)
    }
    #[inline]
    fn comm_dup<F>(
        &self,
        next_f: F,
        comm: mpi_sys::MPI_Comm,
        newcomm: *mut mpi_sys::MPI_Comm,
    ) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Comm, *mut mpi_sys::MPI_Comm) -> c_int,
    {
        next_f(comm, newcomm)
    }
    #[inline]
    fn comm_dup_with_info<F>(
        &self,
        next_f: F,
        comm: mpi_sys::MPI_Comm,
        info: mpi_sys::MPI_Info,
        newcomm: *mut mpi_sys::MPI_Comm,
    ) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Comm, mpi_sys::MPI_Info, *mut mpi_sys::MPI_Comm) -> c_int,
    {
        next_f(comm, info, newcomm)
    }
    #[inline]
    fn comm_free<F>(&self, next_f: F, comm: *mut mpi_sys::MPI_Comm) -> c_int
    where
        F: FnOnce(*mut mpi_sys::MPI_Comm) -> c_int,
    {
        next_f(comm)
    }
    #[inline]
    fn comm_free_keyval<F>(&self, next_f: F, comm_keyval: *mut c_int) -> c_int
    where
        F: FnOnce(*mut c_int) -> c_int,
    {
        next_f(comm_keyval)
    }
    #[inline]
    fn comm_get_attr<F>(
        &self,
        next_f: F,
        comm: mpi_sys::MPI_Comm,
        comm_keyval: c_int,
        attribute_val: *mut c_void,
        flag: *mut c_int,
    ) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Comm, c_int, *mut c_void, *mut c_int) -> c_int,
    {
        next_f(comm, comm_keyval, attribute_val, flag)
    }
    #[inline]
    fn comm_get_errhandler<F>(
        &self,
        next_f: F,
        comm: mpi_sys::MPI_Comm,
        erhandler: *mut mpi_sys::MPI_Errhandler,
    ) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Comm, *mut mpi_sys::MPI_Errhandler) -> c_int,
    {
        next_f(comm, erhandler)
    }
    #[inline]
    fn comm_get_info<F>(
        &self,
        next_f: F,
        comm: mpi_sys::MPI_Comm,
        info_used: *mut mpi_sys::MPI_Info,
    ) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Comm, *mut mpi_sys::MPI_Info) -> c_int,
    {
        next_f(comm, info_used)
    }
    #[inline]
    fn comm_get_name<F>(
        &self,
        next_f: F,
        comm: mpi_sys::MPI_Comm,
        comm_name: *mut c_char,
        resultlen: *mut c_int,
    ) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Comm, *mut c_char, *mut c_int) -> c_int,
    {
        next_f(comm, comm_name, resultlen)
    }
    #[inline]
    fn comm_get_parent<F>(&self, next_f: F, parent: *mut mpi_sys::MPI_Comm) -> c_int
    where
        F: FnOnce(*mut mpi_sys::MPI_Comm) -> c_int,
    {
        next_f(parent)
    }
    #[inline]
    fn comm_group<F>(
        &self,
        next_f: F,
        comm: mpi_sys::MPI_Comm,
        group: *mut mpi_sys::MPI_Group,
    ) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Comm, *mut mpi_sys::MPI_Group) -> c_int,
    {
        next_f(comm, group)
    }
    #[inline]
    fn comm_idup<F>(
        &self,
        next_f: F,
        comm: mpi_sys::MPI_Comm,
        newcomm: *mut mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Comm, *mut mpi_sys::MPI_Comm, *mut mpi_sys::MPI_Request) -> c_int,
    {
        next_f(comm, newcomm, request)
    }
    #[inline]
    fn comm_join<F>(&self, next_f: F, fd: c_int, intercomm: *mut mpi_sys::MPI_Comm) -> c_int
    where
        F: FnOnce(c_int, *mut mpi_sys::MPI_Comm) -> c_int,
    {
        next_f(fd, intercomm)
    }
    #[inline]
    fn comm_rank<F>(&self, next_f: F, comm: mpi_sys::MPI_Comm, rank: *mut c_int) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Comm, *mut c_int) -> c_int,
    {
        next_f(comm, rank)
    }
    #[inline]
    fn comm_remote_group<F>(
        &self,
        next_f: F,
        comm: mpi_sys::MPI_Comm,
        group: *mut mpi_sys::MPI_Group,
    ) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Comm, *mut mpi_sys::MPI_Group) -> c_int,
    {
        next_f(comm, group)
    }
    #[inline]
    fn comm_remote_size<F>(&self, next_f: F, comm: mpi_sys::MPI_Comm, size: *mut c_int) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Comm, *mut c_int) -> c_int,
    {
        next_f(comm, size)
    }
    #[inline]
    fn comm_set_attr<F>(
        &self,
        next_f: F,
        comm: mpi_sys::MPI_Comm,
        comm_keyval: c_int,
        attribute_val: *mut c_void,
    ) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Comm, c_int, *mut c_void) -> c_int,
    {
        next_f(comm, comm_keyval, attribute_val)
    }
    #[inline]
    fn comm_set_errhandler<F>(
        &self,
        next_f: F,
        comm: mpi_sys::MPI_Comm,
        errhandler: mpi_sys::MPI_Errhandler,
    ) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Comm, mpi_sys::MPI_Errhandler) -> c_int,
    {
        next_f(comm, errhandler)
    }
    #[inline]
    fn comm_set_info<F>(&self, next_f: F, comm: mpi_sys::MPI_Comm, info: mpi_sys::MPI_Info) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Comm, mpi_sys::MPI_Info) -> c_int,
    {
        next_f(comm, info)
    }
    #[inline]
    fn comm_set_name<F>(
        &self,
        next_f: F,
        comm: mpi_sys::MPI_Comm,
        comm_name: *const c_char,
    ) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Comm, *const c_char) -> c_int,
    {
        next_f(comm, comm_name)
    }
    #[inline]
    fn comm_size<F>(&self, next_f: F, comm: mpi_sys::MPI_Comm, size: *mut c_int) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Comm, *mut c_int) -> c_int,
    {
        next_f(comm, size)
    }
    #[inline]
    fn comm_split<F>(
        &self,
        next_f: F,
        comm: mpi_sys::MPI_Comm,
        color: c_int,
        key: c_int,
        newcomm: *mut mpi_sys::MPI_Comm,
    ) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Comm, c_int, c_int, *mut mpi_sys::MPI_Comm) -> c_int,
    {
        next_f(comm, color, key, newcomm)
    }
    #[inline]
    fn comm_split_type<F>(
        &self,
        next_f: F,
        comm: mpi_sys::MPI_Comm,
        split_type: c_int,
        key: c_int,
        info: mpi_sys::MPI_Info,
        newcomm: *mut mpi_sys::MPI_Comm,
    ) -> c_int
    where
        F: FnOnce(
            mpi_sys::MPI_Comm,
            c_int,
            c_int,
            mpi_sys::MPI_Info,
            *mut mpi_sys::MPI_Comm,
        ) -> c_int,
    {
        next_f(comm, split_type, key, info, newcomm)
    }
    #[inline]
    fn comm_test_inter<F>(&self, next_f: F, comm: mpi_sys::MPI_Comm, flag: *mut c_int) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Comm, *mut c_int) -> c_int,
    {
        next_f(comm, flag)
    }
    #[inline]
    fn compare_and_swap<F>(
        &self,
        next_f: F,
        origin_addr: *const c_void,
        compare_addr: *const c_void,
        result_addr: *mut c_void,
        datatype: mpi_sys::MPI_Datatype,
        target_rank: c_int,
        target_disp: mpi_sys::MPI_Aint,
        win: mpi_sys::MPI_Win,
    ) -> c_int
    where
        F: FnOnce(
            *const c_void,
            *const c_void,
            *mut c_void,
            mpi_sys::MPI_Datatype,
            c_int,
            mpi_sys::MPI_Aint,
            mpi_sys::MPI_Win,
        ) -> c_int,
    {
        next_f(
            origin_addr,
            compare_addr,
            result_addr,
            datatype,
            target_rank,
            target_disp,
            win,
        )
    }
    #[inline]
    fn dims_create<F>(&self, next_f: F, nnodes: c_int, ndims: c_int, dims: *mut c_int) -> c_int
    where
        F: FnOnce(c_int, c_int, *mut c_int) -> c_int,
    {
        next_f(nnodes, ndims, dims)
    }
    #[inline]
    fn dist_graph_create<F>(
        &self,
        next_f: F,
        comm_old: mpi_sys::MPI_Comm,
        n: c_int,
        nodes: *const c_int,
        degrees: *const c_int,
        targets: *const c_int,
        weights: *const c_int,
        info: mpi_sys::MPI_Info,
        reorder: c_int,
        newcomm: *mut mpi_sys::MPI_Comm,
    ) -> c_int
    where
        F: FnOnce(
            mpi_sys::MPI_Comm,
            c_int,
            *const c_int,
            *const c_int,
            *const c_int,
            *const c_int,
            mpi_sys::MPI_Info,
            c_int,
            *mut mpi_sys::MPI_Comm,
        ) -> c_int,
    {
        next_f(
            comm_old, n, nodes, degrees, targets, weights, info, reorder, newcomm,
        )
    }
    #[inline]
    fn dist_graph_create_adjacent<F>(
        &self,
        next_f: F,
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
    ) -> c_int
    where
        F: FnOnce(
            mpi_sys::MPI_Comm,
            c_int,
            *const c_int,
            *const c_int,
            c_int,
            *const c_int,
            *const c_int,
            mpi_sys::MPI_Info,
            c_int,
            *mut mpi_sys::MPI_Comm,
        ) -> c_int,
    {
        next_f(
            comm_old,
            indegree,
            sources,
            sourceweights,
            outdegree,
            destinations,
            destweights,
            info,
            reorder,
            comm_dist_graph,
        )
    }
    #[inline]
    fn dist_graph_neighbors<F>(
        &self,
        next_f: F,
        comm: mpi_sys::MPI_Comm,
        maxindegree: c_int,
        sources: *mut c_int,
        sourceweights: *mut c_int,
        maxoutdegree: c_int,
        destinations: *mut c_int,
        destweights: *mut c_int,
    ) -> c_int
    where
        F: FnOnce(
            mpi_sys::MPI_Comm,
            c_int,
            *mut c_int,
            *mut c_int,
            c_int,
            *mut c_int,
            *mut c_int,
        ) -> c_int,
    {
        next_f(
            comm,
            maxindegree,
            sources,
            sourceweights,
            maxoutdegree,
            destinations,
            destweights,
        )
    }
    #[inline]
    fn dist_graph_neighbors_count<F>(
        &self,
        next_f: F,
        comm: mpi_sys::MPI_Comm,
        inneighbors: *mut c_int,
        outneighbors: *mut c_int,
        weighted: *mut c_int,
    ) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Comm, *mut c_int, *mut c_int, *mut c_int) -> c_int,
    {
        next_f(comm, inneighbors, outneighbors, weighted)
    }
    #[inline]
    fn errhandler_create<F>(
        &self,
        next_f: F,
        function: *mut mpi_sys::MPI_Handler_function,
        errhandler: *mut mpi_sys::MPI_Errhandler,
    ) -> c_int
    where
        F: FnOnce(*mut mpi_sys::MPI_Handler_function, *mut mpi_sys::MPI_Errhandler) -> c_int,
    {
        next_f(function, errhandler)
    }
    #[inline]
    fn errhandler_free<F>(&self, next_f: F, errhandler: *mut mpi_sys::MPI_Errhandler) -> c_int
    where
        F: FnOnce(*mut mpi_sys::MPI_Errhandler) -> c_int,
    {
        next_f(errhandler)
    }
    #[inline]
    fn errhandler_get<F>(
        &self,
        next_f: F,
        comm: mpi_sys::MPI_Comm,
        errhandler: *mut mpi_sys::MPI_Errhandler,
    ) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Comm, *mut mpi_sys::MPI_Errhandler) -> c_int,
    {
        next_f(comm, errhandler)
    }
    #[inline]
    fn errhandler_set<F>(
        &self,
        next_f: F,
        comm: mpi_sys::MPI_Comm,
        errhandler: mpi_sys::MPI_Errhandler,
    ) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Comm, mpi_sys::MPI_Errhandler) -> c_int,
    {
        next_f(comm, errhandler)
    }
    #[inline]
    fn error_class<F>(&self, next_f: F, errorcode: c_int, errorclass: *mut c_int) -> c_int
    where
        F: FnOnce(c_int, *mut c_int) -> c_int,
    {
        next_f(errorcode, errorclass)
    }
    #[inline]
    fn error_string<F>(
        &self,
        next_f: F,
        errorcode: c_int,
        string: *mut c_char,
        resultlen: *mut c_int,
    ) -> c_int
    where
        F: FnOnce(c_int, *mut c_char, *mut c_int) -> c_int,
    {
        next_f(errorcode, string, resultlen)
    }
    #[inline]
    fn exscan<F>(
        &self,
        next_f: F,
        sendbuf: *const c_void,
        recvbuf: *mut c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        op: mpi_sys::MPI_Op,
        comm: mpi_sys::MPI_Comm,
    ) -> c_int
    where
        F: FnOnce(
            *const c_void,
            *mut c_void,
            c_int,
            mpi_sys::MPI_Datatype,
            mpi_sys::MPI_Op,
            mpi_sys::MPI_Comm,
        ) -> c_int,
    {
        next_f(sendbuf, recvbuf, count, datatype, op, comm)
    }
    #[inline]
    fn fetch_and_op<F>(
        &self,
        next_f: F,
        origin_addr: *const c_void,
        result_addr: *mut c_void,
        datatype: mpi_sys::MPI_Datatype,
        target_rank: c_int,
        target_disp: mpi_sys::MPI_Aint,
        op: mpi_sys::MPI_Op,
        win: mpi_sys::MPI_Win,
    ) -> c_int
    where
        F: FnOnce(
            *const c_void,
            *mut c_void,
            mpi_sys::MPI_Datatype,
            c_int,
            mpi_sys::MPI_Aint,
            mpi_sys::MPI_Op,
            mpi_sys::MPI_Win,
        ) -> c_int,
    {
        next_f(
            origin_addr,
            result_addr,
            datatype,
            target_rank,
            target_disp,
            op,
            win,
        )
    }
    #[inline]
    fn file_call_errhandler<F>(&self, next_f: F, fh: mpi_sys::MPI_File, errorcode: c_int) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_File, c_int) -> c_int,
    {
        next_f(fh, errorcode)
    }
    #[inline]
    fn file_close<F>(&self, next_f: F, fh: *mut mpi_sys::MPI_File) -> c_int
    where
        F: FnOnce(*mut mpi_sys::MPI_File) -> c_int,
    {
        next_f(fh)
    }
    #[inline]
    fn file_create_errhandler<F>(
        &self,
        next_f: F,
        function: *mut mpi_sys::MPI_File_errhandler_function,
        errhandler: *mut mpi_sys::MPI_Errhandler,
    ) -> c_int
    where
        F: FnOnce(
            *mut mpi_sys::MPI_File_errhandler_function,
            *mut mpi_sys::MPI_Errhandler,
        ) -> c_int,
    {
        next_f(function, errhandler)
    }
    #[inline]
    fn file_delete<F>(&self, next_f: F, filename: *const c_char, info: mpi_sys::MPI_Info) -> c_int
    where
        F: FnOnce(*const c_char, mpi_sys::MPI_Info) -> c_int,
    {
        next_f(filename, info)
    }
    #[inline]
    fn file_get_amode<F>(&self, next_f: F, fh: mpi_sys::MPI_File, amode: *mut c_int) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_File, *mut c_int) -> c_int,
    {
        next_f(fh, amode)
    }
    #[inline]
    fn file_get_atomicity<F>(&self, next_f: F, fh: mpi_sys::MPI_File, flag: *mut c_int) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_File, *mut c_int) -> c_int,
    {
        next_f(fh, flag)
    }
    #[inline]
    fn file_get_byte_offset<F>(
        &self,
        next_f: F,
        fh: mpi_sys::MPI_File,
        offset: mpi_sys::MPI_Offset,
        disp: *mut mpi_sys::MPI_Offset,
    ) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_File, mpi_sys::MPI_Offset, *mut mpi_sys::MPI_Offset) -> c_int,
    {
        next_f(fh, offset, disp)
    }
    #[inline]
    fn file_get_errhandler<F>(
        &self,
        next_f: F,
        file: mpi_sys::MPI_File,
        errhandler: *mut mpi_sys::MPI_Errhandler,
    ) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_File, *mut mpi_sys::MPI_Errhandler) -> c_int,
    {
        next_f(file, errhandler)
    }
    #[inline]
    fn file_get_group<F>(
        &self,
        next_f: F,
        fh: mpi_sys::MPI_File,
        group: *mut mpi_sys::MPI_Group,
    ) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_File, *mut mpi_sys::MPI_Group) -> c_int,
    {
        next_f(fh, group)
    }
    #[inline]
    fn file_get_info<F>(
        &self,
        next_f: F,
        fh: mpi_sys::MPI_File,
        info_used: *mut mpi_sys::MPI_Info,
    ) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_File, *mut mpi_sys::MPI_Info) -> c_int,
    {
        next_f(fh, info_used)
    }
    #[inline]
    fn file_get_position<F>(
        &self,
        next_f: F,
        fh: mpi_sys::MPI_File,
        offset: *mut mpi_sys::MPI_Offset,
    ) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_File, *mut mpi_sys::MPI_Offset) -> c_int,
    {
        next_f(fh, offset)
    }
    #[inline]
    fn file_get_position_shared<F>(
        &self,
        next_f: F,
        fh: mpi_sys::MPI_File,
        offset: *mut mpi_sys::MPI_Offset,
    ) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_File, *mut mpi_sys::MPI_Offset) -> c_int,
    {
        next_f(fh, offset)
    }
    #[inline]
    fn file_get_size<F>(
        &self,
        next_f: F,
        fh: mpi_sys::MPI_File,
        size: *mut mpi_sys::MPI_Offset,
    ) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_File, *mut mpi_sys::MPI_Offset) -> c_int,
    {
        next_f(fh, size)
    }
    #[inline]
    fn file_get_type_extent<F>(
        &self,
        next_f: F,
        fh: mpi_sys::MPI_File,
        datatype: mpi_sys::MPI_Datatype,
        extent: *mut mpi_sys::MPI_Aint,
    ) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_File, mpi_sys::MPI_Datatype, *mut mpi_sys::MPI_Aint) -> c_int,
    {
        next_f(fh, datatype, extent)
    }
    #[inline]
    fn file_get_view<F>(
        &self,
        next_f: F,
        fh: mpi_sys::MPI_File,
        disp: *mut mpi_sys::MPI_Offset,
        etype: *mut mpi_sys::MPI_Datatype,
        filetype: *mut mpi_sys::MPI_Datatype,
        datarep: *mut c_char,
    ) -> c_int
    where
        F: FnOnce(
            mpi_sys::MPI_File,
            *mut mpi_sys::MPI_Offset,
            *mut mpi_sys::MPI_Datatype,
            *mut mpi_sys::MPI_Datatype,
            *mut c_char,
        ) -> c_int,
    {
        next_f(fh, disp, etype, filetype, datarep)
    }
    #[inline]
    fn file_iread<F>(
        &self,
        next_f: F,
        fh: mpi_sys::MPI_File,
        buf: *mut c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        request: *mut mpi_sys::MPI_Request,
    ) -> c_int
    where
        F: FnOnce(
            mpi_sys::MPI_File,
            *mut c_void,
            c_int,
            mpi_sys::MPI_Datatype,
            *mut mpi_sys::MPI_Request,
        ) -> c_int,
    {
        next_f(fh, buf, count, datatype, request)
    }
    #[inline]
    fn file_iread_all<F>(
        &self,
        next_f: F,
        fh: mpi_sys::MPI_File,
        buf: *mut c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        request: *mut mpi_sys::MPI_Request,
    ) -> c_int
    where
        F: FnOnce(
            mpi_sys::MPI_File,
            *mut c_void,
            c_int,
            mpi_sys::MPI_Datatype,
            *mut mpi_sys::MPI_Request,
        ) -> c_int,
    {
        next_f(fh, buf, count, datatype, request)
    }
    #[inline]
    fn file_iread_at<F>(
        &self,
        next_f: F,
        fh: mpi_sys::MPI_File,
        offset: mpi_sys::MPI_Offset,
        buf: *mut c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        request: *mut mpi_sys::MPI_Request,
    ) -> c_int
    where
        F: FnOnce(
            mpi_sys::MPI_File,
            mpi_sys::MPI_Offset,
            *mut c_void,
            c_int,
            mpi_sys::MPI_Datatype,
            *mut mpi_sys::MPI_Request,
        ) -> c_int,
    {
        next_f(fh, offset, buf, count, datatype, request)
    }
    #[inline]
    fn file_iread_at_all<F>(
        &self,
        next_f: F,
        fh: mpi_sys::MPI_File,
        offset: mpi_sys::MPI_Offset,
        buf: *mut c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        request: *mut mpi_sys::MPI_Request,
    ) -> c_int
    where
        F: FnOnce(
            mpi_sys::MPI_File,
            mpi_sys::MPI_Offset,
            *mut c_void,
            c_int,
            mpi_sys::MPI_Datatype,
            *mut mpi_sys::MPI_Request,
        ) -> c_int,
    {
        next_f(fh, offset, buf, count, datatype, request)
    }
    #[inline]
    fn file_iread_shared<F>(
        &self,
        next_f: F,
        fh: mpi_sys::MPI_File,
        buf: *mut c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        request: *mut mpi_sys::MPI_Request,
    ) -> c_int
    where
        F: FnOnce(
            mpi_sys::MPI_File,
            *mut c_void,
            c_int,
            mpi_sys::MPI_Datatype,
            *mut mpi_sys::MPI_Request,
        ) -> c_int,
    {
        next_f(fh, buf, count, datatype, request)
    }
    #[inline]
    fn file_iwrite<F>(
        &self,
        next_f: F,
        fh: mpi_sys::MPI_File,
        buf: *const c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        request: *mut mpi_sys::MPI_Request,
    ) -> c_int
    where
        F: FnOnce(
            mpi_sys::MPI_File,
            *const c_void,
            c_int,
            mpi_sys::MPI_Datatype,
            *mut mpi_sys::MPI_Request,
        ) -> c_int,
    {
        next_f(fh, buf, count, datatype, request)
    }
    #[inline]
    fn file_iwrite_all<F>(
        &self,
        next_f: F,
        fh: mpi_sys::MPI_File,
        buf: *const c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        request: *mut mpi_sys::MPI_Request,
    ) -> c_int
    where
        F: FnOnce(
            mpi_sys::MPI_File,
            *const c_void,
            c_int,
            mpi_sys::MPI_Datatype,
            *mut mpi_sys::MPI_Request,
        ) -> c_int,
    {
        next_f(fh, buf, count, datatype, request)
    }
    #[inline]
    fn file_iwrite_at<F>(
        &self,
        next_f: F,
        fh: mpi_sys::MPI_File,
        offset: mpi_sys::MPI_Offset,
        buf: *const c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        request: *mut mpi_sys::MPI_Request,
    ) -> c_int
    where
        F: FnOnce(
            mpi_sys::MPI_File,
            mpi_sys::MPI_Offset,
            *const c_void,
            c_int,
            mpi_sys::MPI_Datatype,
            *mut mpi_sys::MPI_Request,
        ) -> c_int,
    {
        next_f(fh, offset, buf, count, datatype, request)
    }
    #[inline]
    fn file_iwrite_at_all<F>(
        &self,
        next_f: F,
        fh: mpi_sys::MPI_File,
        offset: mpi_sys::MPI_Offset,
        buf: *const c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        request: *mut mpi_sys::MPI_Request,
    ) -> c_int
    where
        F: FnOnce(
            mpi_sys::MPI_File,
            mpi_sys::MPI_Offset,
            *const c_void,
            c_int,
            mpi_sys::MPI_Datatype,
            *mut mpi_sys::MPI_Request,
        ) -> c_int,
    {
        next_f(fh, offset, buf, count, datatype, request)
    }
    #[inline]
    fn file_iwrite_shared<F>(
        &self,
        next_f: F,
        fh: mpi_sys::MPI_File,
        buf: *const c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        request: *mut mpi_sys::MPI_Request,
    ) -> c_int
    where
        F: FnOnce(
            mpi_sys::MPI_File,
            *const c_void,
            c_int,
            mpi_sys::MPI_Datatype,
            *mut mpi_sys::MPI_Request,
        ) -> c_int,
    {
        next_f(fh, buf, count, datatype, request)
    }
    #[inline]
    fn file_open<F>(
        &self,
        next_f: F,
        comm: mpi_sys::MPI_Comm,
        filename: *const c_char,
        amode: c_int,
        info: mpi_sys::MPI_Info,
        fh: *mut mpi_sys::MPI_File,
    ) -> c_int
    where
        F: FnOnce(
            mpi_sys::MPI_Comm,
            *const c_char,
            c_int,
            mpi_sys::MPI_Info,
            *mut mpi_sys::MPI_File,
        ) -> c_int,
    {
        next_f(comm, filename, amode, info, fh)
    }
    #[inline]
    fn file_preallocate<F>(
        &self,
        next_f: F,
        fh: mpi_sys::MPI_File,
        size: mpi_sys::MPI_Offset,
    ) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_File, mpi_sys::MPI_Offset) -> c_int,
    {
        next_f(fh, size)
    }
    #[inline]
    fn file_read<F>(
        &self,
        next_f: F,
        fh: mpi_sys::MPI_File,
        buf: *mut c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        status: *mut mpi_sys::MPI_Status,
    ) -> c_int
    where
        F: FnOnce(
            mpi_sys::MPI_File,
            *mut c_void,
            c_int,
            mpi_sys::MPI_Datatype,
            *mut mpi_sys::MPI_Status,
        ) -> c_int,
    {
        next_f(fh, buf, count, datatype, status)
    }
    #[inline]
    fn file_read_all<F>(
        &self,
        next_f: F,
        fh: mpi_sys::MPI_File,
        buf: *mut c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        status: *mut mpi_sys::MPI_Status,
    ) -> c_int
    where
        F: FnOnce(
            mpi_sys::MPI_File,
            *mut c_void,
            c_int,
            mpi_sys::MPI_Datatype,
            *mut mpi_sys::MPI_Status,
        ) -> c_int,
    {
        next_f(fh, buf, count, datatype, status)
    }
    #[inline]
    fn file_read_all_begin<F>(
        &self,
        next_f: F,
        fh: mpi_sys::MPI_File,
        buf: *mut c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
    ) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_File, *mut c_void, c_int, mpi_sys::MPI_Datatype) -> c_int,
    {
        next_f(fh, buf, count, datatype)
    }
    #[inline]
    fn file_read_all_end<F>(
        &self,
        next_f: F,
        fh: mpi_sys::MPI_File,
        buf: *mut c_void,
        status: *mut mpi_sys::MPI_Status,
    ) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_File, *mut c_void, *mut mpi_sys::MPI_Status) -> c_int,
    {
        next_f(fh, buf, status)
    }
    #[inline]
    fn file_read_at<F>(
        &self,
        next_f: F,
        fh: mpi_sys::MPI_File,
        offset: mpi_sys::MPI_Offset,
        buf: *mut c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        status: *mut mpi_sys::MPI_Status,
    ) -> c_int
    where
        F: FnOnce(
            mpi_sys::MPI_File,
            mpi_sys::MPI_Offset,
            *mut c_void,
            c_int,
            mpi_sys::MPI_Datatype,
            *mut mpi_sys::MPI_Status,
        ) -> c_int,
    {
        next_f(fh, offset, buf, count, datatype, status)
    }
    #[inline]
    fn file_read_at_all<F>(
        &self,
        next_f: F,
        fh: mpi_sys::MPI_File,
        offset: mpi_sys::MPI_Offset,
        buf: *mut c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        status: *mut mpi_sys::MPI_Status,
    ) -> c_int
    where
        F: FnOnce(
            mpi_sys::MPI_File,
            mpi_sys::MPI_Offset,
            *mut c_void,
            c_int,
            mpi_sys::MPI_Datatype,
            *mut mpi_sys::MPI_Status,
        ) -> c_int,
    {
        next_f(fh, offset, buf, count, datatype, status)
    }
    #[inline]
    fn file_read_at_all_begin<F>(
        &self,
        next_f: F,
        fh: mpi_sys::MPI_File,
        offset: mpi_sys::MPI_Offset,
        buf: *mut c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
    ) -> c_int
    where
        F: FnOnce(
            mpi_sys::MPI_File,
            mpi_sys::MPI_Offset,
            *mut c_void,
            c_int,
            mpi_sys::MPI_Datatype,
        ) -> c_int,
    {
        next_f(fh, offset, buf, count, datatype)
    }
    #[inline]
    fn file_read_at_all_end<F>(
        &self,
        next_f: F,
        fh: mpi_sys::MPI_File,
        buf: *mut c_void,
        status: *mut mpi_sys::MPI_Status,
    ) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_File, *mut c_void, *mut mpi_sys::MPI_Status) -> c_int,
    {
        next_f(fh, buf, status)
    }
    #[inline]
    fn file_read_ordered<F>(
        &self,
        next_f: F,
        fh: mpi_sys::MPI_File,
        buf: *mut c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        status: *mut mpi_sys::MPI_Status,
    ) -> c_int
    where
        F: FnOnce(
            mpi_sys::MPI_File,
            *mut c_void,
            c_int,
            mpi_sys::MPI_Datatype,
            *mut mpi_sys::MPI_Status,
        ) -> c_int,
    {
        next_f(fh, buf, count, datatype, status)
    }
    #[inline]
    fn file_read_ordered_begin<F>(
        &self,
        next_f: F,
        fh: mpi_sys::MPI_File,
        buf: *mut c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
    ) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_File, *mut c_void, c_int, mpi_sys::MPI_Datatype) -> c_int,
    {
        next_f(fh, buf, count, datatype)
    }
    #[inline]
    fn file_read_ordered_end<F>(
        &self,
        next_f: F,
        fh: mpi_sys::MPI_File,
        buf: *mut c_void,
        status: *mut mpi_sys::MPI_Status,
    ) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_File, *mut c_void, *mut mpi_sys::MPI_Status) -> c_int,
    {
        next_f(fh, buf, status)
    }
    #[inline]
    fn file_read_shared<F>(
        &self,
        next_f: F,
        fh: mpi_sys::MPI_File,
        buf: *mut c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        status: *mut mpi_sys::MPI_Status,
    ) -> c_int
    where
        F: FnOnce(
            mpi_sys::MPI_File,
            *mut c_void,
            c_int,
            mpi_sys::MPI_Datatype,
            *mut mpi_sys::MPI_Status,
        ) -> c_int,
    {
        next_f(fh, buf, count, datatype, status)
    }
    #[inline]
    fn file_seek<F>(
        &self,
        next_f: F,
        fh: mpi_sys::MPI_File,
        offset: mpi_sys::MPI_Offset,
        whence: c_int,
    ) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_File, mpi_sys::MPI_Offset, c_int) -> c_int,
    {
        next_f(fh, offset, whence)
    }
    #[inline]
    fn file_seek_shared<F>(
        &self,
        next_f: F,
        fh: mpi_sys::MPI_File,
        offset: mpi_sys::MPI_Offset,
        whence: c_int,
    ) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_File, mpi_sys::MPI_Offset, c_int) -> c_int,
    {
        next_f(fh, offset, whence)
    }
    #[inline]
    fn file_set_atomicity<F>(&self, next_f: F, fh: mpi_sys::MPI_File, flag: c_int) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_File, c_int) -> c_int,
    {
        next_f(fh, flag)
    }
    #[inline]
    fn file_set_errhandler<F>(
        &self,
        next_f: F,
        file: mpi_sys::MPI_File,
        errhandler: mpi_sys::MPI_Errhandler,
    ) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_File, mpi_sys::MPI_Errhandler) -> c_int,
    {
        next_f(file, errhandler)
    }
    #[inline]
    fn file_set_info<F>(&self, next_f: F, fh: mpi_sys::MPI_File, info: mpi_sys::MPI_Info) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_File, mpi_sys::MPI_Info) -> c_int,
    {
        next_f(fh, info)
    }
    #[inline]
    fn file_set_size<F>(&self, next_f: F, fh: mpi_sys::MPI_File, size: mpi_sys::MPI_Offset) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_File, mpi_sys::MPI_Offset) -> c_int,
    {
        next_f(fh, size)
    }
    #[inline]
    fn file_set_view<F>(
        &self,
        next_f: F,
        fh: mpi_sys::MPI_File,
        disp: mpi_sys::MPI_Offset,
        etype: mpi_sys::MPI_Datatype,
        filetype: mpi_sys::MPI_Datatype,
        datarep: *const c_char,
        info: mpi_sys::MPI_Info,
    ) -> c_int
    where
        F: FnOnce(
            mpi_sys::MPI_File,
            mpi_sys::MPI_Offset,
            mpi_sys::MPI_Datatype,
            mpi_sys::MPI_Datatype,
            *const c_char,
            mpi_sys::MPI_Info,
        ) -> c_int,
    {
        next_f(fh, disp, etype, filetype, datarep, info)
    }
    #[inline]
    fn file_sync<F>(&self, next_f: F, fh: mpi_sys::MPI_File) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_File) -> c_int,
    {
        next_f(fh)
    }
    #[inline]
    fn file_write<F>(
        &self,
        next_f: F,
        fh: mpi_sys::MPI_File,
        buf: *const c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        status: *mut mpi_sys::MPI_Status,
    ) -> c_int
    where
        F: FnOnce(
            mpi_sys::MPI_File,
            *const c_void,
            c_int,
            mpi_sys::MPI_Datatype,
            *mut mpi_sys::MPI_Status,
        ) -> c_int,
    {
        next_f(fh, buf, count, datatype, status)
    }
    #[inline]
    fn file_write_all<F>(
        &self,
        next_f: F,
        fh: mpi_sys::MPI_File,
        buf: *const c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        status: *mut mpi_sys::MPI_Status,
    ) -> c_int
    where
        F: FnOnce(
            mpi_sys::MPI_File,
            *const c_void,
            c_int,
            mpi_sys::MPI_Datatype,
            *mut mpi_sys::MPI_Status,
        ) -> c_int,
    {
        next_f(fh, buf, count, datatype, status)
    }
    #[inline]
    fn file_write_all_begin<F>(
        &self,
        next_f: F,
        fh: mpi_sys::MPI_File,
        buf: *const c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
    ) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_File, *const c_void, c_int, mpi_sys::MPI_Datatype) -> c_int,
    {
        next_f(fh, buf, count, datatype)
    }
    #[inline]
    fn file_write_all_end<F>(
        &self,
        next_f: F,
        fh: mpi_sys::MPI_File,
        buf: *const c_void,
        status: *mut mpi_sys::MPI_Status,
    ) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_File, *const c_void, *mut mpi_sys::MPI_Status) -> c_int,
    {
        next_f(fh, buf, status)
    }
    #[inline]
    fn file_write_at<F>(
        &self,
        next_f: F,
        fh: mpi_sys::MPI_File,
        offset: mpi_sys::MPI_Offset,
        buf: *const c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        status: *mut mpi_sys::MPI_Status,
    ) -> c_int
    where
        F: FnOnce(
            mpi_sys::MPI_File,
            mpi_sys::MPI_Offset,
            *const c_void,
            c_int,
            mpi_sys::MPI_Datatype,
            *mut mpi_sys::MPI_Status,
        ) -> c_int,
    {
        next_f(fh, offset, buf, count, datatype, status)
    }
    #[inline]
    fn file_write_at_all<F>(
        &self,
        next_f: F,
        fh: mpi_sys::MPI_File,
        offset: mpi_sys::MPI_Offset,
        buf: *const c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        status: *mut mpi_sys::MPI_Status,
    ) -> c_int
    where
        F: FnOnce(
            mpi_sys::MPI_File,
            mpi_sys::MPI_Offset,
            *const c_void,
            c_int,
            mpi_sys::MPI_Datatype,
            *mut mpi_sys::MPI_Status,
        ) -> c_int,
    {
        next_f(fh, offset, buf, count, datatype, status)
    }
    #[inline]
    fn file_write_at_all_begin<F>(
        &self,
        next_f: F,
        fh: mpi_sys::MPI_File,
        offset: mpi_sys::MPI_Offset,
        buf: *const c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
    ) -> c_int
    where
        F: FnOnce(
            mpi_sys::MPI_File,
            mpi_sys::MPI_Offset,
            *const c_void,
            c_int,
            mpi_sys::MPI_Datatype,
        ) -> c_int,
    {
        next_f(fh, offset, buf, count, datatype)
    }
    #[inline]
    fn file_write_at_all_end<F>(
        &self,
        next_f: F,
        fh: mpi_sys::MPI_File,
        buf: *const c_void,
        status: *mut mpi_sys::MPI_Status,
    ) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_File, *const c_void, *mut mpi_sys::MPI_Status) -> c_int,
    {
        next_f(fh, buf, status)
    }
    #[inline]
    fn file_write_ordered<F>(
        &self,
        next_f: F,
        fh: mpi_sys::MPI_File,
        buf: *const c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        status: *mut mpi_sys::MPI_Status,
    ) -> c_int
    where
        F: FnOnce(
            mpi_sys::MPI_File,
            *const c_void,
            c_int,
            mpi_sys::MPI_Datatype,
            *mut mpi_sys::MPI_Status,
        ) -> c_int,
    {
        next_f(fh, buf, count, datatype, status)
    }
    #[inline]
    fn file_write_ordered_begin<F>(
        &self,
        next_f: F,
        fh: mpi_sys::MPI_File,
        buf: *const c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
    ) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_File, *const c_void, c_int, mpi_sys::MPI_Datatype) -> c_int,
    {
        next_f(fh, buf, count, datatype)
    }
    #[inline]
    fn file_write_ordered_end<F>(
        &self,
        next_f: F,
        fh: mpi_sys::MPI_File,
        buf: *const c_void,
        status: *mut mpi_sys::MPI_Status,
    ) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_File, *const c_void, *mut mpi_sys::MPI_Status) -> c_int,
    {
        next_f(fh, buf, status)
    }
    #[inline]
    fn file_write_shared<F>(
        &self,
        next_f: F,
        fh: mpi_sys::MPI_File,
        buf: *const c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        status: *mut mpi_sys::MPI_Status,
    ) -> c_int
    where
        F: FnOnce(
            mpi_sys::MPI_File,
            *const c_void,
            c_int,
            mpi_sys::MPI_Datatype,
            *mut mpi_sys::MPI_Status,
        ) -> c_int,
    {
        next_f(fh, buf, count, datatype, status)
    }
    #[inline]
    fn finalize<F>(&self, next_f: F) -> c_int
    where
        F: FnOnce() -> c_int,
    {
        next_f()
    }
    #[inline]
    fn finalized<F>(&self, next_f: F, flag: *mut c_int) -> c_int
    where
        F: FnOnce(*mut c_int) -> c_int,
    {
        next_f(flag)
    }
    #[inline]
    fn free_mem<F>(&self, next_f: F, base: *mut c_void) -> c_int
    where
        F: FnOnce(*mut c_void) -> c_int,
    {
        next_f(base)
    }
    #[inline]
    fn gather<F>(
        &self,
        next_f: F,
        sendbuf: *const c_void,
        sendcount: c_int,
        sendtype: mpi_sys::MPI_Datatype,
        recvbuf: *mut c_void,
        recvcount: c_int,
        recvtype: mpi_sys::MPI_Datatype,
        root: c_int,
        comm: mpi_sys::MPI_Comm,
    ) -> c_int
    where
        F: FnOnce(
            *const c_void,
            c_int,
            mpi_sys::MPI_Datatype,
            *mut c_void,
            c_int,
            mpi_sys::MPI_Datatype,
            c_int,
            mpi_sys::MPI_Comm,
        ) -> c_int,
    {
        next_f(
            sendbuf, sendcount, sendtype, recvbuf, recvcount, recvtype, root, comm,
        )
    }
    #[inline]
    fn gatherv<F>(
        &self,
        next_f: F,
        sendbuf: *const c_void,
        sendcount: c_int,
        sendtype: mpi_sys::MPI_Datatype,
        recvbuf: *mut c_void,
        recvcounts: *const c_int,
        displs: *const c_int,
        recvtype: mpi_sys::MPI_Datatype,
        root: c_int,
        comm: mpi_sys::MPI_Comm,
    ) -> c_int
    where
        F: FnOnce(
            *const c_void,
            c_int,
            mpi_sys::MPI_Datatype,
            *mut c_void,
            *const c_int,
            *const c_int,
            mpi_sys::MPI_Datatype,
            c_int,
            mpi_sys::MPI_Comm,
        ) -> c_int,
    {
        next_f(
            sendbuf, sendcount, sendtype, recvbuf, recvcounts, displs, recvtype, root, comm,
        )
    }
    #[inline]
    fn get<F>(
        &self,
        next_f: F,
        origin_addr: *mut c_void,
        origin_count: c_int,
        origin_datatype: mpi_sys::MPI_Datatype,
        target_rank: c_int,
        target_disp: mpi_sys::MPI_Aint,
        target_count: c_int,
        target_datatype: mpi_sys::MPI_Datatype,
        win: mpi_sys::MPI_Win,
    ) -> c_int
    where
        F: FnOnce(
            *mut c_void,
            c_int,
            mpi_sys::MPI_Datatype,
            c_int,
            mpi_sys::MPI_Aint,
            c_int,
            mpi_sys::MPI_Datatype,
            mpi_sys::MPI_Win,
        ) -> c_int,
    {
        next_f(
            origin_addr,
            origin_count,
            origin_datatype,
            target_rank,
            target_disp,
            target_count,
            target_datatype,
            win,
        )
    }
    #[inline]
    fn get_accumulate<F>(
        &self,
        next_f: F,
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
    ) -> c_int
    where
        F: FnOnce(
            *const c_void,
            c_int,
            mpi_sys::MPI_Datatype,
            *mut c_void,
            c_int,
            mpi_sys::MPI_Datatype,
            c_int,
            mpi_sys::MPI_Aint,
            c_int,
            mpi_sys::MPI_Datatype,
            mpi_sys::MPI_Op,
            mpi_sys::MPI_Win,
        ) -> c_int,
    {
        next_f(
            origin_addr,
            origin_count,
            origin_datatype,
            result_addr,
            result_count,
            result_datatype,
            target_rank,
            target_disp,
            target_count,
            target_datatype,
            op,
            win,
        )
    }
    #[inline]
    fn get_address<F>(
        &self,
        next_f: F,
        location: *const c_void,
        address: *mut mpi_sys::MPI_Aint,
    ) -> c_int
    where
        F: FnOnce(*const c_void, *mut mpi_sys::MPI_Aint) -> c_int,
    {
        next_f(location, address)
    }
    #[inline]
    fn get_count<F>(
        &self,
        next_f: F,
        status: *const mpi_sys::MPI_Status,
        datatype: mpi_sys::MPI_Datatype,
        count: *mut c_int,
    ) -> c_int
    where
        F: FnOnce(*const mpi_sys::MPI_Status, mpi_sys::MPI_Datatype, *mut c_int) -> c_int,
    {
        next_f(status, datatype, count)
    }
    #[inline]
    fn get_elements<F>(
        &self,
        next_f: F,
        status: *const mpi_sys::MPI_Status,
        datatype: mpi_sys::MPI_Datatype,
        count: *mut c_int,
    ) -> c_int
    where
        F: FnOnce(*const mpi_sys::MPI_Status, mpi_sys::MPI_Datatype, *mut c_int) -> c_int,
    {
        next_f(status, datatype, count)
    }
    #[inline]
    fn get_elements_x<F>(
        &self,
        next_f: F,
        status: *const mpi_sys::MPI_Status,
        datatype: mpi_sys::MPI_Datatype,
        count: *mut mpi_sys::MPI_Count,
    ) -> c_int
    where
        F: FnOnce(
            *const mpi_sys::MPI_Status,
            mpi_sys::MPI_Datatype,
            *mut mpi_sys::MPI_Count,
        ) -> c_int,
    {
        next_f(status, datatype, count)
    }
    #[inline]
    fn get_library_version<F>(
        &self,
        next_f: F,
        version: *mut c_char,
        resultlen: *mut c_int,
    ) -> c_int
    where
        F: FnOnce(*mut c_char, *mut c_int) -> c_int,
    {
        next_f(version, resultlen)
    }
    #[inline]
    fn get_processor_name<F>(&self, next_f: F, name: *mut c_char, resultlen: *mut c_int) -> c_int
    where
        F: FnOnce(*mut c_char, *mut c_int) -> c_int,
    {
        next_f(name, resultlen)
    }
    #[inline]
    fn get_version<F>(&self, next_f: F, version: *mut c_int, subversion: *mut c_int) -> c_int
    where
        F: FnOnce(*mut c_int, *mut c_int) -> c_int,
    {
        next_f(version, subversion)
    }
    #[inline]
    fn graph_create<F>(
        &self,
        next_f: F,
        comm_old: mpi_sys::MPI_Comm,
        nnodes: c_int,
        index: *const c_int,
        edges: *const c_int,
        reorder: c_int,
        comm_graph: *mut mpi_sys::MPI_Comm,
    ) -> c_int
    where
        F: FnOnce(
            mpi_sys::MPI_Comm,
            c_int,
            *const c_int,
            *const c_int,
            c_int,
            *mut mpi_sys::MPI_Comm,
        ) -> c_int,
    {
        next_f(comm_old, nnodes, index, edges, reorder, comm_graph)
    }
    #[inline]
    fn graph_get<F>(
        &self,
        next_f: F,
        comm: mpi_sys::MPI_Comm,
        maxindex: c_int,
        maxedges: c_int,
        index: *mut c_int,
        edges: *mut c_int,
    ) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Comm, c_int, c_int, *mut c_int, *mut c_int) -> c_int,
    {
        next_f(comm, maxindex, maxedges, index, edges)
    }
    #[inline]
    fn graph_map<F>(
        &self,
        next_f: F,
        comm: mpi_sys::MPI_Comm,
        nnodes: c_int,
        index: *const c_int,
        edges: *const c_int,
        newrank: *mut c_int,
    ) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Comm, c_int, *const c_int, *const c_int, *mut c_int) -> c_int,
    {
        next_f(comm, nnodes, index, edges, newrank)
    }
    #[inline]
    fn graph_neighbors<F>(
        &self,
        next_f: F,
        comm: mpi_sys::MPI_Comm,
        rank: c_int,
        maxneighbors: c_int,
        neighbors: *mut c_int,
    ) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Comm, c_int, c_int, *mut c_int) -> c_int,
    {
        next_f(comm, rank, maxneighbors, neighbors)
    }
    #[inline]
    fn graph_neighbors_count<F>(
        &self,
        next_f: F,
        comm: mpi_sys::MPI_Comm,
        rank: c_int,
        nneighbors: *mut c_int,
    ) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Comm, c_int, *mut c_int) -> c_int,
    {
        next_f(comm, rank, nneighbors)
    }
    #[inline]
    fn graphdims_get<F>(
        &self,
        next_f: F,
        comm: mpi_sys::MPI_Comm,
        nnodes: *mut c_int,
        nedges: *mut c_int,
    ) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Comm, *mut c_int, *mut c_int) -> c_int,
    {
        next_f(comm, nnodes, nedges)
    }
    #[inline]
    fn grequest_complete<F>(&self, next_f: F, request: mpi_sys::MPI_Request) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Request) -> c_int,
    {
        next_f(request)
    }
    #[inline]
    fn grequest_start<F>(
        &self,
        next_f: F,
        query_fn: *mut mpi_sys::MPI_Grequest_query_function,
        free_fn: *mut mpi_sys::MPI_Grequest_free_function,
        cancel_fn: *mut mpi_sys::MPI_Grequest_cancel_function,
        extra_state: *mut c_void,
        request: *mut mpi_sys::MPI_Request,
    ) -> c_int
    where
        F: FnOnce(
            *mut mpi_sys::MPI_Grequest_query_function,
            *mut mpi_sys::MPI_Grequest_free_function,
            *mut mpi_sys::MPI_Grequest_cancel_function,
            *mut c_void,
            *mut mpi_sys::MPI_Request,
        ) -> c_int,
    {
        next_f(query_fn, free_fn, cancel_fn, extra_state, request)
    }
    #[inline]
    fn group_compare<F>(
        &self,
        next_f: F,
        group1: mpi_sys::MPI_Group,
        group2: mpi_sys::MPI_Group,
        result: *mut c_int,
    ) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Group, mpi_sys::MPI_Group, *mut c_int) -> c_int,
    {
        next_f(group1, group2, result)
    }
    #[inline]
    fn group_difference<F>(
        &self,
        next_f: F,
        group1: mpi_sys::MPI_Group,
        group2: mpi_sys::MPI_Group,
        newgroup: *mut mpi_sys::MPI_Group,
    ) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Group, mpi_sys::MPI_Group, *mut mpi_sys::MPI_Group) -> c_int,
    {
        next_f(group1, group2, newgroup)
    }
    #[inline]
    fn group_excl<F>(
        &self,
        next_f: F,
        group: mpi_sys::MPI_Group,
        n: c_int,
        ranks: *const c_int,
        newgroup: *mut mpi_sys::MPI_Group,
    ) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Group, c_int, *const c_int, *mut mpi_sys::MPI_Group) -> c_int,
    {
        next_f(group, n, ranks, newgroup)
    }
    #[inline]
    fn group_free<F>(&self, next_f: F, group: *mut mpi_sys::MPI_Group) -> c_int
    where
        F: FnOnce(*mut mpi_sys::MPI_Group) -> c_int,
    {
        next_f(group)
    }
    #[inline]
    fn group_incl<F>(
        &self,
        next_f: F,
        group: mpi_sys::MPI_Group,
        n: c_int,
        ranks: *const c_int,
        newgroup: *mut mpi_sys::MPI_Group,
    ) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Group, c_int, *const c_int, *mut mpi_sys::MPI_Group) -> c_int,
    {
        next_f(group, n, ranks, newgroup)
    }
    #[inline]
    fn group_intersection<F>(
        &self,
        next_f: F,
        group1: mpi_sys::MPI_Group,
        group2: mpi_sys::MPI_Group,
        newgroup: *mut mpi_sys::MPI_Group,
    ) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Group, mpi_sys::MPI_Group, *mut mpi_sys::MPI_Group) -> c_int,
    {
        next_f(group1, group2, newgroup)
    }
    #[inline]
    fn group_range_excl<F>(
        &self,
        next_f: F,
        group: mpi_sys::MPI_Group,
        n: c_int,
        ranges: *mut [c_int; 3],
        newgroup: *mut mpi_sys::MPI_Group,
    ) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Group, c_int, *mut [c_int; 3], *mut mpi_sys::MPI_Group) -> c_int,
    {
        next_f(group, n, ranges, newgroup)
    }
    #[inline]
    fn group_range_incl<F>(
        &self,
        next_f: F,
        group: mpi_sys::MPI_Group,
        n: c_int,
        ranges: *mut [c_int; 3],
        newgroup: *mut mpi_sys::MPI_Group,
    ) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Group, c_int, *mut [c_int; 3], *mut mpi_sys::MPI_Group) -> c_int,
    {
        next_f(group, n, ranges, newgroup)
    }
    #[inline]
    fn group_rank<F>(&self, next_f: F, group: mpi_sys::MPI_Group, rank: *mut c_int) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Group, *mut c_int) -> c_int,
    {
        next_f(group, rank)
    }
    #[inline]
    fn group_size<F>(&self, next_f: F, group: mpi_sys::MPI_Group, size: *mut c_int) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Group, *mut c_int) -> c_int,
    {
        next_f(group, size)
    }
    #[inline]
    fn group_translate_ranks<F>(
        &self,
        next_f: F,
        group1: mpi_sys::MPI_Group,
        n: c_int,
        ranks1: *const c_int,
        group2: mpi_sys::MPI_Group,
        ranks2: *mut c_int,
    ) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Group, c_int, *const c_int, mpi_sys::MPI_Group, *mut c_int) -> c_int,
    {
        next_f(group1, n, ranks1, group2, ranks2)
    }
    #[inline]
    fn group_union<F>(
        &self,
        next_f: F,
        group1: mpi_sys::MPI_Group,
        group2: mpi_sys::MPI_Group,
        newgroup: *mut mpi_sys::MPI_Group,
    ) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Group, mpi_sys::MPI_Group, *mut mpi_sys::MPI_Group) -> c_int,
    {
        next_f(group1, group2, newgroup)
    }
    #[inline]
    fn iallgather<F>(
        &self,
        next_f: F,
        sendbuf: *const c_void,
        sendcount: c_int,
        sendtype: mpi_sys::MPI_Datatype,
        recvbuf: *mut c_void,
        recvcount: c_int,
        recvtype: mpi_sys::MPI_Datatype,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) -> c_int
    where
        F: FnOnce(
            *const c_void,
            c_int,
            mpi_sys::MPI_Datatype,
            *mut c_void,
            c_int,
            mpi_sys::MPI_Datatype,
            mpi_sys::MPI_Comm,
            *mut mpi_sys::MPI_Request,
        ) -> c_int,
    {
        next_f(
            sendbuf, sendcount, sendtype, recvbuf, recvcount, recvtype, comm, request,
        )
    }
    #[inline]
    fn iallgatherv<F>(
        &self,
        next_f: F,
        sendbuf: *const c_void,
        sendcount: c_int,
        sendtype: mpi_sys::MPI_Datatype,
        recvbuf: *mut c_void,
        recvcounts: *const c_int,
        displs: *const c_int,
        recvtype: mpi_sys::MPI_Datatype,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) -> c_int
    where
        F: FnOnce(
            *const c_void,
            c_int,
            mpi_sys::MPI_Datatype,
            *mut c_void,
            *const c_int,
            *const c_int,
            mpi_sys::MPI_Datatype,
            mpi_sys::MPI_Comm,
            *mut mpi_sys::MPI_Request,
        ) -> c_int,
    {
        next_f(
            sendbuf, sendcount, sendtype, recvbuf, recvcounts, displs, recvtype, comm, request,
        )
    }
    #[inline]
    fn iallreduce<F>(
        &self,
        next_f: F,
        sendbuf: *const c_void,
        recvbuf: *mut c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        op: mpi_sys::MPI_Op,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) -> c_int
    where
        F: FnOnce(
            *const c_void,
            *mut c_void,
            c_int,
            mpi_sys::MPI_Datatype,
            mpi_sys::MPI_Op,
            mpi_sys::MPI_Comm,
            *mut mpi_sys::MPI_Request,
        ) -> c_int,
    {
        next_f(sendbuf, recvbuf, count, datatype, op, comm, request)
    }
    #[inline]
    fn ialltoall<F>(
        &self,
        next_f: F,
        sendbuf: *const c_void,
        sendcount: c_int,
        sendtype: mpi_sys::MPI_Datatype,
        recvbuf: *mut c_void,
        recvcount: c_int,
        recvtype: mpi_sys::MPI_Datatype,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) -> c_int
    where
        F: FnOnce(
            *const c_void,
            c_int,
            mpi_sys::MPI_Datatype,
            *mut c_void,
            c_int,
            mpi_sys::MPI_Datatype,
            mpi_sys::MPI_Comm,
            *mut mpi_sys::MPI_Request,
        ) -> c_int,
    {
        next_f(
            sendbuf, sendcount, sendtype, recvbuf, recvcount, recvtype, comm, request,
        )
    }
    #[inline]
    fn ialltoallv<F>(
        &self,
        next_f: F,
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
    ) -> c_int
    where
        F: FnOnce(
            *const c_void,
            *const c_int,
            *const c_int,
            mpi_sys::MPI_Datatype,
            *mut c_void,
            *const c_int,
            *const c_int,
            mpi_sys::MPI_Datatype,
            mpi_sys::MPI_Comm,
            *mut mpi_sys::MPI_Request,
        ) -> c_int,
    {
        next_f(
            sendbuf, sendcounts, sdispls, sendtype, recvbuf, recvcounts, rdispls, recvtype, comm,
            request,
        )
    }
    #[inline]
    fn ialltoallw<F>(
        &self,
        next_f: F,
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
    ) -> c_int
    where
        F: FnOnce(
            *const c_void,
            *const c_int,
            *const c_int,
            *const mpi_sys::MPI_Datatype,
            *mut c_void,
            *const c_int,
            *const c_int,
            *const mpi_sys::MPI_Datatype,
            mpi_sys::MPI_Comm,
            *mut mpi_sys::MPI_Request,
        ) -> c_int,
    {
        next_f(
            sendbuf, sendcounts, sdispls, sendtypes, recvbuf, recvcounts, rdispls, recvtypes, comm,
            request,
        )
    }
    #[inline]
    fn ibarrier<F>(
        &self,
        next_f: F,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Comm, *mut mpi_sys::MPI_Request) -> c_int,
    {
        next_f(comm, request)
    }
    #[inline]
    fn ibcast<F>(
        &self,
        next_f: F,
        buffer: *mut c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        root: c_int,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) -> c_int
    where
        F: FnOnce(
            *mut c_void,
            c_int,
            mpi_sys::MPI_Datatype,
            c_int,
            mpi_sys::MPI_Comm,
            *mut mpi_sys::MPI_Request,
        ) -> c_int,
    {
        next_f(buffer, count, datatype, root, comm, request)
    }
    #[inline]
    fn ibsend<F>(
        &self,
        next_f: F,
        buf: *const c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        dest: c_int,
        tag: c_int,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) -> c_int
    where
        F: FnOnce(
            *const c_void,
            c_int,
            mpi_sys::MPI_Datatype,
            c_int,
            c_int,
            mpi_sys::MPI_Comm,
            *mut mpi_sys::MPI_Request,
        ) -> c_int,
    {
        next_f(buf, count, datatype, dest, tag, comm, request)
    }
    #[inline]
    fn iexscan<F>(
        &self,
        next_f: F,
        sendbuf: *const c_void,
        recvbuf: *mut c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        op: mpi_sys::MPI_Op,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) -> c_int
    where
        F: FnOnce(
            *const c_void,
            *mut c_void,
            c_int,
            mpi_sys::MPI_Datatype,
            mpi_sys::MPI_Op,
            mpi_sys::MPI_Comm,
            *mut mpi_sys::MPI_Request,
        ) -> c_int,
    {
        next_f(sendbuf, recvbuf, count, datatype, op, comm, request)
    }
    #[inline]
    fn igather<F>(
        &self,
        next_f: F,
        sendbuf: *const c_void,
        sendcount: c_int,
        sendtype: mpi_sys::MPI_Datatype,
        recvbuf: *mut c_void,
        recvcount: c_int,
        recvtype: mpi_sys::MPI_Datatype,
        root: c_int,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) -> c_int
    where
        F: FnOnce(
            *const c_void,
            c_int,
            mpi_sys::MPI_Datatype,
            *mut c_void,
            c_int,
            mpi_sys::MPI_Datatype,
            c_int,
            mpi_sys::MPI_Comm,
            *mut mpi_sys::MPI_Request,
        ) -> c_int,
    {
        next_f(
            sendbuf, sendcount, sendtype, recvbuf, recvcount, recvtype, root, comm, request,
        )
    }
    #[inline]
    fn igatherv<F>(
        &self,
        next_f: F,
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
    ) -> c_int
    where
        F: FnOnce(
            *const c_void,
            c_int,
            mpi_sys::MPI_Datatype,
            *mut c_void,
            *const c_int,
            *const c_int,
            mpi_sys::MPI_Datatype,
            c_int,
            mpi_sys::MPI_Comm,
            *mut mpi_sys::MPI_Request,
        ) -> c_int,
    {
        next_f(
            sendbuf, sendcount, sendtype, recvbuf, recvcounts, displs, recvtype, root, comm,
            request,
        )
    }
    #[inline]
    fn improbe<F>(
        &self,
        next_f: F,
        source: c_int,
        tag: c_int,
        comm: mpi_sys::MPI_Comm,
        flag: *mut c_int,
        message: *mut mpi_sys::MPI_Message,
        status: *mut mpi_sys::MPI_Status,
    ) -> c_int
    where
        F: FnOnce(
            c_int,
            c_int,
            mpi_sys::MPI_Comm,
            *mut c_int,
            *mut mpi_sys::MPI_Message,
            *mut mpi_sys::MPI_Status,
        ) -> c_int,
    {
        next_f(source, tag, comm, flag, message, status)
    }
    #[inline]
    fn imrecv<F>(
        &self,
        next_f: F,
        buf: *mut c_void,
        count: c_int,
        r#type: mpi_sys::MPI_Datatype,
        message: *mut mpi_sys::MPI_Message,
        request: *mut mpi_sys::MPI_Request,
    ) -> c_int
    where
        F: FnOnce(
            *mut c_void,
            c_int,
            mpi_sys::MPI_Datatype,
            *mut mpi_sys::MPI_Message,
            *mut mpi_sys::MPI_Request,
        ) -> c_int,
    {
        next_f(buf, count, r#type, message, request)
    }
    #[inline]
    fn ineighbor_allgather<F>(
        &self,
        next_f: F,
        sendbuf: *const c_void,
        sendcount: c_int,
        sendtype: mpi_sys::MPI_Datatype,
        recvbuf: *mut c_void,
        recvcount: c_int,
        recvtype: mpi_sys::MPI_Datatype,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) -> c_int
    where
        F: FnOnce(
            *const c_void,
            c_int,
            mpi_sys::MPI_Datatype,
            *mut c_void,
            c_int,
            mpi_sys::MPI_Datatype,
            mpi_sys::MPI_Comm,
            *mut mpi_sys::MPI_Request,
        ) -> c_int,
    {
        next_f(
            sendbuf, sendcount, sendtype, recvbuf, recvcount, recvtype, comm, request,
        )
    }
    #[inline]
    fn ineighbor_allgatherv<F>(
        &self,
        next_f: F,
        sendbuf: *const c_void,
        sendcount: c_int,
        sendtype: mpi_sys::MPI_Datatype,
        recvbuf: *mut c_void,
        recvcounts: *const c_int,
        displs: *const c_int,
        recvtype: mpi_sys::MPI_Datatype,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) -> c_int
    where
        F: FnOnce(
            *const c_void,
            c_int,
            mpi_sys::MPI_Datatype,
            *mut c_void,
            *const c_int,
            *const c_int,
            mpi_sys::MPI_Datatype,
            mpi_sys::MPI_Comm,
            *mut mpi_sys::MPI_Request,
        ) -> c_int,
    {
        next_f(
            sendbuf, sendcount, sendtype, recvbuf, recvcounts, displs, recvtype, comm, request,
        )
    }
    #[inline]
    fn ineighbor_alltoall<F>(
        &self,
        next_f: F,
        sendbuf: *const c_void,
        sendcount: c_int,
        sendtype: mpi_sys::MPI_Datatype,
        recvbuf: *mut c_void,
        recvcount: c_int,
        recvtype: mpi_sys::MPI_Datatype,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) -> c_int
    where
        F: FnOnce(
            *const c_void,
            c_int,
            mpi_sys::MPI_Datatype,
            *mut c_void,
            c_int,
            mpi_sys::MPI_Datatype,
            mpi_sys::MPI_Comm,
            *mut mpi_sys::MPI_Request,
        ) -> c_int,
    {
        next_f(
            sendbuf, sendcount, sendtype, recvbuf, recvcount, recvtype, comm, request,
        )
    }
    #[inline]
    fn ineighbor_alltoallv<F>(
        &self,
        next_f: F,
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
    ) -> c_int
    where
        F: FnOnce(
            *const c_void,
            *const c_int,
            *const c_int,
            mpi_sys::MPI_Datatype,
            *mut c_void,
            *const c_int,
            *const c_int,
            mpi_sys::MPI_Datatype,
            mpi_sys::MPI_Comm,
            *mut mpi_sys::MPI_Request,
        ) -> c_int,
    {
        next_f(
            sendbuf, sendcounts, sdispls, sendtype, recvbuf, recvcounts, rdispls, recvtype, comm,
            request,
        )
    }
    #[inline]
    fn ineighbor_alltoallw<F>(
        &self,
        next_f: F,
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
    ) -> c_int
    where
        F: FnOnce(
            *const c_void,
            *const c_int,
            *const mpi_sys::MPI_Aint,
            *const mpi_sys::MPI_Datatype,
            *mut c_void,
            *const c_int,
            *const mpi_sys::MPI_Aint,
            *const mpi_sys::MPI_Datatype,
            mpi_sys::MPI_Comm,
            *mut mpi_sys::MPI_Request,
        ) -> c_int,
    {
        next_f(
            sendbuf, sendcounts, sdispls, sendtypes, recvbuf, recvcounts, rdispls, recvtypes, comm,
            request,
        )
    }
    #[inline]
    fn info_create<F>(&self, next_f: F, info: *mut mpi_sys::MPI_Info) -> c_int
    where
        F: FnOnce(*mut mpi_sys::MPI_Info) -> c_int,
    {
        next_f(info)
    }
    #[inline]
    fn info_delete<F>(&self, next_f: F, info: mpi_sys::MPI_Info, key: *const c_char) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Info, *const c_char) -> c_int,
    {
        next_f(info, key)
    }
    #[inline]
    fn info_dup<F>(
        &self,
        next_f: F,
        info: mpi_sys::MPI_Info,
        newinfo: *mut mpi_sys::MPI_Info,
    ) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Info, *mut mpi_sys::MPI_Info) -> c_int,
    {
        next_f(info, newinfo)
    }
    #[inline]
    fn info_free<F>(&self, next_f: F, info: *mut mpi_sys::MPI_Info) -> c_int
    where
        F: FnOnce(*mut mpi_sys::MPI_Info) -> c_int,
    {
        next_f(info)
    }
    #[inline]
    fn info_get<F>(
        &self,
        next_f: F,
        info: mpi_sys::MPI_Info,
        key: *const c_char,
        valuelen: c_int,
        value: *mut c_char,
        flag: *mut c_int,
    ) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Info, *const c_char, c_int, *mut c_char, *mut c_int) -> c_int,
    {
        next_f(info, key, valuelen, value, flag)
    }
    #[inline]
    fn info_get_nkeys<F>(&self, next_f: F, info: mpi_sys::MPI_Info, nkeys: *mut c_int) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Info, *mut c_int) -> c_int,
    {
        next_f(info, nkeys)
    }
    #[inline]
    fn info_get_nthkey<F>(
        &self,
        next_f: F,
        info: mpi_sys::MPI_Info,
        n: c_int,
        key: *mut c_char,
    ) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Info, c_int, *mut c_char) -> c_int,
    {
        next_f(info, n, key)
    }
    #[inline]
    fn info_get_valuelen<F>(
        &self,
        next_f: F,
        info: mpi_sys::MPI_Info,
        key: *const c_char,
        valuelen: *mut c_int,
        flag: *mut c_int,
        i: c_int,
        v: *mut qmpi_sys::vector,
    ) -> c_int
    where
        F: FnOnce(
            mpi_sys::MPI_Info,
            *const c_char,
            *mut c_int,
            *mut c_int,
            c_int,
            *mut qmpi_sys::vector,
        ) -> c_int,
    {
        next_f(info, key, valuelen, flag, i, v)
    }
    #[inline]
    fn info_set<F>(
        &self,
        next_f: F,
        info: mpi_sys::MPI_Info,
        key: *const c_char,
        value: *const c_char,
    ) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Info, *const c_char, *const c_char) -> c_int,
    {
        next_f(info, key, value)
    }
    #[inline]
    fn init<F>(&self, next_f: F, argc: *mut c_int, argv: *mut *mut *mut c_char) -> c_int
    where
        F: FnOnce(*mut c_int, *mut *mut *mut c_char) -> c_int,
    {
        next_f(argc, argv)
    }
    #[inline]
    fn init_thread<F>(
        &self,
        next_f: F,
        argc: *mut c_int,
        argv: *mut *mut *mut c_char,
        required: c_int,
        provided: *mut c_int,
    ) -> c_int
    where
        F: FnOnce(*mut c_int, *mut *mut *mut c_char, c_int, *mut c_int) -> c_int,
    {
        next_f(argc, argv, required, provided)
    }
    #[inline]
    fn initialized<F>(&self, next_f: F, flag: *mut c_int) -> c_int
    where
        F: FnOnce(*mut c_int) -> c_int,
    {
        next_f(flag)
    }
    #[inline]
    fn intercomm_create<F>(
        &self,
        next_f: F,
        local_comm: mpi_sys::MPI_Comm,
        local_leader: c_int,
        bridge_comm: mpi_sys::MPI_Comm,
        remote_leader: c_int,
        tag: c_int,
        newintercomm: *mut mpi_sys::MPI_Comm,
    ) -> c_int
    where
        F: FnOnce(
            mpi_sys::MPI_Comm,
            c_int,
            mpi_sys::MPI_Comm,
            c_int,
            c_int,
            *mut mpi_sys::MPI_Comm,
        ) -> c_int,
    {
        next_f(
            local_comm,
            local_leader,
            bridge_comm,
            remote_leader,
            tag,
            newintercomm,
        )
    }
    #[inline]
    fn intercomm_merge<F>(
        &self,
        next_f: F,
        intercomm: mpi_sys::MPI_Comm,
        high: c_int,
        newintercomm: *mut mpi_sys::MPI_Comm,
    ) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Comm, c_int, *mut mpi_sys::MPI_Comm) -> c_int,
    {
        next_f(intercomm, high, newintercomm)
    }
    #[inline]
    fn iprobe<F>(
        &self,
        next_f: F,
        source: c_int,
        tag: c_int,
        comm: mpi_sys::MPI_Comm,
        flag: *mut c_int,
        status: *mut mpi_sys::MPI_Status,
    ) -> c_int
    where
        F: FnOnce(c_int, c_int, mpi_sys::MPI_Comm, *mut c_int, *mut mpi_sys::MPI_Status) -> c_int,
    {
        next_f(source, tag, comm, flag, status)
    }
    #[inline]
    fn irecv<F>(
        &self,
        next_f: F,
        buf: *mut c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        source: c_int,
        tag: c_int,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) -> c_int
    where
        F: FnOnce(
            *mut c_void,
            c_int,
            mpi_sys::MPI_Datatype,
            c_int,
            c_int,
            mpi_sys::MPI_Comm,
            *mut mpi_sys::MPI_Request,
        ) -> c_int,
    {
        next_f(buf, count, datatype, source, tag, comm, request)
    }
    #[inline]
    fn ireduce<F>(
        &self,
        next_f: F,
        sendbuf: *const c_void,
        recvbuf: *mut c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        op: mpi_sys::MPI_Op,
        root: c_int,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) -> c_int
    where
        F: FnOnce(
            *const c_void,
            *mut c_void,
            c_int,
            mpi_sys::MPI_Datatype,
            mpi_sys::MPI_Op,
            c_int,
            mpi_sys::MPI_Comm,
            *mut mpi_sys::MPI_Request,
        ) -> c_int,
    {
        next_f(sendbuf, recvbuf, count, datatype, op, root, comm, request)
    }
    #[inline]
    fn ireduce_scatter<F>(
        &self,
        next_f: F,
        sendbuf: *const c_void,
        recvbuf: *mut c_void,
        recvcounts: *const c_int,
        datatype: mpi_sys::MPI_Datatype,
        op: mpi_sys::MPI_Op,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) -> c_int
    where
        F: FnOnce(
            *const c_void,
            *mut c_void,
            *const c_int,
            mpi_sys::MPI_Datatype,
            mpi_sys::MPI_Op,
            mpi_sys::MPI_Comm,
            *mut mpi_sys::MPI_Request,
        ) -> c_int,
    {
        next_f(sendbuf, recvbuf, recvcounts, datatype, op, comm, request)
    }
    #[inline]
    fn ireduce_scatter_block<F>(
        &self,
        next_f: F,
        sendbuf: *const c_void,
        recvbuf: *mut c_void,
        recvcount: c_int,
        datatype: mpi_sys::MPI_Datatype,
        op: mpi_sys::MPI_Op,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) -> c_int
    where
        F: FnOnce(
            *const c_void,
            *mut c_void,
            c_int,
            mpi_sys::MPI_Datatype,
            mpi_sys::MPI_Op,
            mpi_sys::MPI_Comm,
            *mut mpi_sys::MPI_Request,
        ) -> c_int,
    {
        next_f(sendbuf, recvbuf, recvcount, datatype, op, comm, request)
    }
    #[inline]
    fn irsend<F>(
        &self,
        next_f: F,
        buf: *const c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        dest: c_int,
        tag: c_int,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) -> c_int
    where
        F: FnOnce(
            *const c_void,
            c_int,
            mpi_sys::MPI_Datatype,
            c_int,
            c_int,
            mpi_sys::MPI_Comm,
            *mut mpi_sys::MPI_Request,
        ) -> c_int,
    {
        next_f(buf, count, datatype, dest, tag, comm, request)
    }
    #[inline]
    fn is_thread_main<F>(&self, next_f: F, flag: *mut c_int) -> c_int
    where
        F: FnOnce(*mut c_int) -> c_int,
    {
        next_f(flag)
    }
    #[inline]
    fn iscan<F>(
        &self,
        next_f: F,
        sendbuf: *const c_void,
        recvbuf: *mut c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        op: mpi_sys::MPI_Op,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) -> c_int
    where
        F: FnOnce(
            *const c_void,
            *mut c_void,
            c_int,
            mpi_sys::MPI_Datatype,
            mpi_sys::MPI_Op,
            mpi_sys::MPI_Comm,
            *mut mpi_sys::MPI_Request,
        ) -> c_int,
    {
        next_f(sendbuf, recvbuf, count, datatype, op, comm, request)
    }
    #[inline]
    fn iscatter<F>(
        &self,
        next_f: F,
        sendbuf: *const c_void,
        sendcount: c_int,
        sendtype: mpi_sys::MPI_Datatype,
        recvbuf: *mut c_void,
        recvcount: c_int,
        recvtype: mpi_sys::MPI_Datatype,
        root: c_int,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) -> c_int
    where
        F: FnOnce(
            *const c_void,
            c_int,
            mpi_sys::MPI_Datatype,
            *mut c_void,
            c_int,
            mpi_sys::MPI_Datatype,
            c_int,
            mpi_sys::MPI_Comm,
            *mut mpi_sys::MPI_Request,
        ) -> c_int,
    {
        next_f(
            sendbuf, sendcount, sendtype, recvbuf, recvcount, recvtype, root, comm, request,
        )
    }
    #[inline]
    fn iscatterv<F>(
        &self,
        next_f: F,
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
    ) -> c_int
    where
        F: FnOnce(
            *const c_void,
            *const c_int,
            *const c_int,
            mpi_sys::MPI_Datatype,
            *mut c_void,
            c_int,
            mpi_sys::MPI_Datatype,
            c_int,
            mpi_sys::MPI_Comm,
            *mut mpi_sys::MPI_Request,
        ) -> c_int,
    {
        next_f(
            sendbuf, sendcounts, displs, sendtype, recvbuf, recvcount, recvtype, root, comm,
            request,
        )
    }
    #[inline]
    fn isend<F>(
        &self,
        next_f: F,
        buf: *const c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        dest: c_int,
        tag: c_int,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) -> c_int
    where
        F: FnOnce(
            *const c_void,
            c_int,
            mpi_sys::MPI_Datatype,
            c_int,
            c_int,
            mpi_sys::MPI_Comm,
            *mut mpi_sys::MPI_Request,
        ) -> c_int,
    {
        next_f(buf, count, datatype, dest, tag, comm, request)
    }
    #[inline]
    fn issend<F>(
        &self,
        next_f: F,
        buf: *const c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        dest: c_int,
        tag: c_int,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) -> c_int
    where
        F: FnOnce(
            *const c_void,
            c_int,
            mpi_sys::MPI_Datatype,
            c_int,
            c_int,
            mpi_sys::MPI_Comm,
            *mut mpi_sys::MPI_Request,
        ) -> c_int,
    {
        next_f(buf, count, datatype, dest, tag, comm, request)
    }
    #[inline]
    fn keyval_create<F>(
        &self,
        next_f: F,
        copy_fn: *mut mpi_sys::MPI_Copy_function,
        delete_fn: *mut mpi_sys::MPI_Delete_function,
        keyval: *mut c_int,
        extra_state: *mut c_void,
    ) -> c_int
    where
        F: FnOnce(
            *mut mpi_sys::MPI_Copy_function,
            *mut mpi_sys::MPI_Delete_function,
            *mut c_int,
            *mut c_void,
        ) -> c_int,
    {
        next_f(copy_fn, delete_fn, keyval, extra_state)
    }
    #[inline]
    fn keyval_free<F>(&self, next_f: F, keyval: *mut c_int) -> c_int
    where
        F: FnOnce(*mut c_int) -> c_int,
    {
        next_f(keyval)
    }
    #[inline]
    fn lookup_name<F>(
        &self,
        next_f: F,
        service_name: *const c_char,
        info: mpi_sys::MPI_Info,
        port_name: *mut c_char,
    ) -> c_int
    where
        F: FnOnce(*const c_char, mpi_sys::MPI_Info, *mut c_char) -> c_int,
    {
        next_f(service_name, info, port_name)
    }
    #[inline]
    fn mprobe<F>(
        &self,
        next_f: F,
        source: c_int,
        tag: c_int,
        comm: mpi_sys::MPI_Comm,
        message: *mut mpi_sys::MPI_Message,
        status: *mut mpi_sys::MPI_Status,
    ) -> c_int
    where
        F: FnOnce(
            c_int,
            c_int,
            mpi_sys::MPI_Comm,
            *mut mpi_sys::MPI_Message,
            *mut mpi_sys::MPI_Status,
        ) -> c_int,
    {
        next_f(source, tag, comm, message, status)
    }
    #[inline]
    fn mrecv<F>(
        &self,
        next_f: F,
        buf: *mut c_void,
        count: c_int,
        r#type: mpi_sys::MPI_Datatype,
        message: *mut mpi_sys::MPI_Message,
        status: *mut mpi_sys::MPI_Status,
    ) -> c_int
    where
        F: FnOnce(
            *mut c_void,
            c_int,
            mpi_sys::MPI_Datatype,
            *mut mpi_sys::MPI_Message,
            *mut mpi_sys::MPI_Status,
        ) -> c_int,
    {
        next_f(buf, count, r#type, message, status)
    }
    #[inline]
    fn neighbor_allgather<F>(
        &self,
        next_f: F,
        sendbuf: *const c_void,
        sendcount: c_int,
        sendtype: mpi_sys::MPI_Datatype,
        recvbuf: *mut c_void,
        recvcount: c_int,
        recvtype: mpi_sys::MPI_Datatype,
        comm: mpi_sys::MPI_Comm,
    ) -> c_int
    where
        F: FnOnce(
            *const c_void,
            c_int,
            mpi_sys::MPI_Datatype,
            *mut c_void,
            c_int,
            mpi_sys::MPI_Datatype,
            mpi_sys::MPI_Comm,
        ) -> c_int,
    {
        next_f(
            sendbuf, sendcount, sendtype, recvbuf, recvcount, recvtype, comm,
        )
    }
    #[inline]
    fn neighbor_allgatherv<F>(
        &self,
        next_f: F,
        sendbuf: *const c_void,
        sendcount: c_int,
        sendtype: mpi_sys::MPI_Datatype,
        recvbuf: *mut c_void,
        recvcounts: *const c_int,
        displs: *const c_int,
        recvtype: mpi_sys::MPI_Datatype,
        comm: mpi_sys::MPI_Comm,
    ) -> c_int
    where
        F: FnOnce(
            *const c_void,
            c_int,
            mpi_sys::MPI_Datatype,
            *mut c_void,
            *const c_int,
            *const c_int,
            mpi_sys::MPI_Datatype,
            mpi_sys::MPI_Comm,
        ) -> c_int,
    {
        next_f(
            sendbuf, sendcount, sendtype, recvbuf, recvcounts, displs, recvtype, comm,
        )
    }
    #[inline]
    fn neighbor_alltoall<F>(
        &self,
        next_f: F,
        sendbuf: *const c_void,
        sendcount: c_int,
        sendtype: mpi_sys::MPI_Datatype,
        recvbuf: *mut c_void,
        recvcount: c_int,
        recvtype: mpi_sys::MPI_Datatype,
        comm: mpi_sys::MPI_Comm,
    ) -> c_int
    where
        F: FnOnce(
            *const c_void,
            c_int,
            mpi_sys::MPI_Datatype,
            *mut c_void,
            c_int,
            mpi_sys::MPI_Datatype,
            mpi_sys::MPI_Comm,
        ) -> c_int,
    {
        next_f(
            sendbuf, sendcount, sendtype, recvbuf, recvcount, recvtype, comm,
        )
    }
    #[inline]
    fn neighbor_alltoallv<F>(
        &self,
        next_f: F,
        sendbuf: *const c_void,
        sendcounts: *const c_int,
        sdispls: *const c_int,
        sendtype: mpi_sys::MPI_Datatype,
        recvbuf: *mut c_void,
        recvcounts: *const c_int,
        rdispls: *const c_int,
        recvtype: mpi_sys::MPI_Datatype,
        comm: mpi_sys::MPI_Comm,
    ) -> c_int
    where
        F: FnOnce(
            *const c_void,
            *const c_int,
            *const c_int,
            mpi_sys::MPI_Datatype,
            *mut c_void,
            *const c_int,
            *const c_int,
            mpi_sys::MPI_Datatype,
            mpi_sys::MPI_Comm,
        ) -> c_int,
    {
        next_f(
            sendbuf, sendcounts, sdispls, sendtype, recvbuf, recvcounts, rdispls, recvtype, comm,
        )
    }
    #[inline]
    fn neighbor_alltoallw<F>(
        &self,
        next_f: F,
        sendbuf: *const c_void,
        sendcounts: *const c_int,
        sdispls: *const mpi_sys::MPI_Aint,
        sendtypes: *const mpi_sys::MPI_Datatype,
        recvbuf: *mut c_void,
        recvcounts: *const c_int,
        rdispls: *const mpi_sys::MPI_Aint,
        recvtypes: *const mpi_sys::MPI_Datatype,
        comm: mpi_sys::MPI_Comm,
    ) -> c_int
    where
        F: FnOnce(
            *const c_void,
            *const c_int,
            *const mpi_sys::MPI_Aint,
            *const mpi_sys::MPI_Datatype,
            *mut c_void,
            *const c_int,
            *const mpi_sys::MPI_Aint,
            *const mpi_sys::MPI_Datatype,
            mpi_sys::MPI_Comm,
        ) -> c_int,
    {
        next_f(
            sendbuf, sendcounts, sdispls, sendtypes, recvbuf, recvcounts, rdispls, recvtypes, comm,
        )
    }
    #[inline]
    fn op_commutative<F>(&self, next_f: F, op: mpi_sys::MPI_Op, commute: *mut c_int) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Op, *mut c_int) -> c_int,
    {
        next_f(op, commute)
    }
    #[inline]
    fn op_create<F>(
        &self,
        next_f: F,
        function: *mut mpi_sys::MPI_User_function,
        commute: c_int,
        op: *mut mpi_sys::MPI_Op,
    ) -> c_int
    where
        F: FnOnce(*mut mpi_sys::MPI_User_function, c_int, *mut mpi_sys::MPI_Op) -> c_int,
    {
        next_f(function, commute, op)
    }
    #[inline]
    fn op_free<F>(&self, next_f: F, op: *mut mpi_sys::MPI_Op) -> c_int
    where
        F: FnOnce(*mut mpi_sys::MPI_Op) -> c_int,
    {
        next_f(op)
    }
    #[inline]
    fn open_port<F>(&self, next_f: F, info: mpi_sys::MPI_Info, port_name: *mut c_char) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Info, *mut c_char) -> c_int,
    {
        next_f(info, port_name)
    }
    #[inline]
    fn pack<F>(
        &self,
        next_f: F,
        inbuf: *const c_void,
        incount: c_int,
        datatype: mpi_sys::MPI_Datatype,
        outbuf: *mut c_void,
        outsize: c_int,
        position: *mut c_int,
        comm: mpi_sys::MPI_Comm,
    ) -> c_int
    where
        F: FnOnce(
            *const c_void,
            c_int,
            mpi_sys::MPI_Datatype,
            *mut c_void,
            c_int,
            *mut c_int,
            mpi_sys::MPI_Comm,
        ) -> c_int,
    {
        next_f(inbuf, incount, datatype, outbuf, outsize, position, comm)
    }
    #[inline]
    fn pack_external<F>(
        &self,
        next_f: F,
        datarep: *const c_char,
        inbuf: *const c_void,
        incount: c_int,
        datatype: mpi_sys::MPI_Datatype,
        outbuf: *mut c_void,
        outsize: mpi_sys::MPI_Aint,
        position: *mut mpi_sys::MPI_Aint,
    ) -> c_int
    where
        F: FnOnce(
            *const c_char,
            *const c_void,
            c_int,
            mpi_sys::MPI_Datatype,
            *mut c_void,
            mpi_sys::MPI_Aint,
            *mut mpi_sys::MPI_Aint,
        ) -> c_int,
    {
        next_f(datarep, inbuf, incount, datatype, outbuf, outsize, position)
    }
    #[inline]
    fn pack_external_size<F>(
        &self,
        next_f: F,
        datarep: *const c_char,
        incount: c_int,
        datatype: mpi_sys::MPI_Datatype,
        size: *mut mpi_sys::MPI_Aint,
    ) -> c_int
    where
        F: FnOnce(*const c_char, c_int, mpi_sys::MPI_Datatype, *mut mpi_sys::MPI_Aint) -> c_int,
    {
        next_f(datarep, incount, datatype, size)
    }
    #[inline]
    fn pack_size<F>(
        &self,
        next_f: F,
        incount: c_int,
        datatype: mpi_sys::MPI_Datatype,
        comm: mpi_sys::MPI_Comm,
        size: *mut c_int,
    ) -> c_int
    where
        F: FnOnce(c_int, mpi_sys::MPI_Datatype, mpi_sys::MPI_Comm, *mut c_int) -> c_int,
    {
        next_f(incount, datatype, comm, size)
    }
    #[inline]
    fn pcontrol<F>(&self, next_f: F, level: c_int) -> c_int
    where
        F: FnOnce(c_int) -> c_int,
    {
        next_f(level)
    }
    #[inline]
    fn probe<F>(
        &self,
        next_f: F,
        source: c_int,
        tag: c_int,
        comm: mpi_sys::MPI_Comm,
        status: *mut mpi_sys::MPI_Status,
    ) -> c_int
    where
        F: FnOnce(c_int, c_int, mpi_sys::MPI_Comm, *mut mpi_sys::MPI_Status) -> c_int,
    {
        next_f(source, tag, comm, status)
    }
    #[inline]
    fn publish_name<F>(
        &self,
        next_f: F,
        service_name: *const c_char,
        info: mpi_sys::MPI_Info,
        port_name: *const c_char,
    ) -> c_int
    where
        F: FnOnce(*const c_char, mpi_sys::MPI_Info, *const c_char) -> c_int,
    {
        next_f(service_name, info, port_name)
    }
    #[inline]
    fn put<F>(
        &self,
        next_f: F,
        origin_addr: *const c_void,
        origin_count: c_int,
        origin_datatype: mpi_sys::MPI_Datatype,
        target_rank: c_int,
        target_disp: mpi_sys::MPI_Aint,
        target_count: c_int,
        target_datatype: mpi_sys::MPI_Datatype,
        win: mpi_sys::MPI_Win,
    ) -> c_int
    where
        F: FnOnce(
            *const c_void,
            c_int,
            mpi_sys::MPI_Datatype,
            c_int,
            mpi_sys::MPI_Aint,
            c_int,
            mpi_sys::MPI_Datatype,
            mpi_sys::MPI_Win,
        ) -> c_int,
    {
        next_f(
            origin_addr,
            origin_count,
            origin_datatype,
            target_rank,
            target_disp,
            target_count,
            target_datatype,
            win,
        )
    }
    #[inline]
    fn query_thread<F>(&self, next_f: F, provided: *mut c_int) -> c_int
    where
        F: FnOnce(*mut c_int) -> c_int,
    {
        next_f(provided)
    }
    #[inline]
    fn raccumulate<F>(
        &self,
        next_f: F,
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
    ) -> c_int
    where
        F: FnOnce(
            *const c_void,
            c_int,
            mpi_sys::MPI_Datatype,
            c_int,
            mpi_sys::MPI_Aint,
            c_int,
            mpi_sys::MPI_Datatype,
            mpi_sys::MPI_Op,
            mpi_sys::MPI_Win,
            *mut mpi_sys::MPI_Request,
        ) -> c_int,
    {
        next_f(
            origin_addr,
            origin_count,
            origin_datatype,
            target_rank,
            target_disp,
            target_count,
            target_datatype,
            op,
            win,
            request,
        )
    }
    #[inline]
    fn recv<F>(
        &self,
        next_f: F,
        buf: *mut c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        source: c_int,
        tag: c_int,
        comm: mpi_sys::MPI_Comm,
        status: *mut mpi_sys::MPI_Status,
    ) -> c_int
    where
        F: FnOnce(
            *mut c_void,
            c_int,
            mpi_sys::MPI_Datatype,
            c_int,
            c_int,
            mpi_sys::MPI_Comm,
            *mut mpi_sys::MPI_Status,
        ) -> c_int,
    {
        next_f(buf, count, datatype, source, tag, comm, status)
    }
    #[inline]
    fn recv_init<F>(
        &self,
        next_f: F,
        buf: *mut c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        source: c_int,
        tag: c_int,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) -> c_int
    where
        F: FnOnce(
            *mut c_void,
            c_int,
            mpi_sys::MPI_Datatype,
            c_int,
            c_int,
            mpi_sys::MPI_Comm,
            *mut mpi_sys::MPI_Request,
        ) -> c_int,
    {
        next_f(buf, count, datatype, source, tag, comm, request)
    }
    #[inline]
    fn reduce<F>(
        &self,
        next_f: F,
        sendbuf: *const c_void,
        recvbuf: *mut c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        op: mpi_sys::MPI_Op,
        root: c_int,
        comm: mpi_sys::MPI_Comm,
    ) -> c_int
    where
        F: FnOnce(
            *const c_void,
            *mut c_void,
            c_int,
            mpi_sys::MPI_Datatype,
            mpi_sys::MPI_Op,
            c_int,
            mpi_sys::MPI_Comm,
        ) -> c_int,
    {
        next_f(sendbuf, recvbuf, count, datatype, op, root, comm)
    }
    #[inline]
    fn reduce_local<F>(
        &self,
        next_f: F,
        inbuf: *const c_void,
        inoutbuf: *mut c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        op: mpi_sys::MPI_Op,
    ) -> c_int
    where
        F: FnOnce(
            *const c_void,
            *mut c_void,
            c_int,
            mpi_sys::MPI_Datatype,
            mpi_sys::MPI_Op,
        ) -> c_int,
    {
        next_f(inbuf, inoutbuf, count, datatype, op)
    }
    #[inline]
    fn reduce_scatter<F>(
        &self,
        next_f: F,
        sendbuf: *const c_void,
        recvbuf: *mut c_void,
        recvcounts: *const c_int,
        datatype: mpi_sys::MPI_Datatype,
        op: mpi_sys::MPI_Op,
        comm: mpi_sys::MPI_Comm,
    ) -> c_int
    where
        F: FnOnce(
            *const c_void,
            *mut c_void,
            *const c_int,
            mpi_sys::MPI_Datatype,
            mpi_sys::MPI_Op,
            mpi_sys::MPI_Comm,
        ) -> c_int,
    {
        next_f(sendbuf, recvbuf, recvcounts, datatype, op, comm)
    }
    #[inline]
    fn reduce_scatter_block<F>(
        &self,
        next_f: F,
        sendbuf: *const c_void,
        recvbuf: *mut c_void,
        recvcount: c_int,
        datatype: mpi_sys::MPI_Datatype,
        op: mpi_sys::MPI_Op,
        comm: mpi_sys::MPI_Comm,
    ) -> c_int
    where
        F: FnOnce(
            *const c_void,
            *mut c_void,
            c_int,
            mpi_sys::MPI_Datatype,
            mpi_sys::MPI_Op,
            mpi_sys::MPI_Comm,
        ) -> c_int,
    {
        next_f(sendbuf, recvbuf, recvcount, datatype, op, comm)
    }
    #[inline]
    fn register_datarep<F>(
        &self,
        next_f: F,
        datarep: *const c_char,
        read_conversion_fn: *mut mpi_sys::MPI_Datarep_conversion_function,
        write_conversion_fn: *mut mpi_sys::MPI_Datarep_conversion_function,
        dtype_file_extent_fn: *mut mpi_sys::MPI_Datarep_extent_function,
        extra_state: *mut c_void,
    ) -> c_int
    where
        F: FnOnce(
            *const c_char,
            *mut mpi_sys::MPI_Datarep_conversion_function,
            *mut mpi_sys::MPI_Datarep_conversion_function,
            *mut mpi_sys::MPI_Datarep_extent_function,
            *mut c_void,
        ) -> c_int,
    {
        next_f(
            datarep,
            read_conversion_fn,
            write_conversion_fn,
            dtype_file_extent_fn,
            extra_state,
        )
    }
    #[inline]
    fn request_free<F>(&self, next_f: F, request: *mut mpi_sys::MPI_Request) -> c_int
    where
        F: FnOnce(*mut mpi_sys::MPI_Request) -> c_int,
    {
        next_f(request)
    }
    #[inline]
    fn request_get_status<F>(
        &self,
        next_f: F,
        request: mpi_sys::MPI_Request,
        flag: *mut c_int,
        status: *mut mpi_sys::MPI_Status,
    ) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Request, *mut c_int, *mut mpi_sys::MPI_Status) -> c_int,
    {
        next_f(request, flag, status)
    }
    #[inline]
    fn rget<F>(
        &self,
        next_f: F,
        origin_addr: *mut c_void,
        origin_count: c_int,
        origin_datatype: mpi_sys::MPI_Datatype,
        target_rank: c_int,
        target_disp: mpi_sys::MPI_Aint,
        target_count: c_int,
        target_datatype: mpi_sys::MPI_Datatype,
        win: mpi_sys::MPI_Win,
        request: *mut mpi_sys::MPI_Request,
    ) -> c_int
    where
        F: FnOnce(
            *mut c_void,
            c_int,
            mpi_sys::MPI_Datatype,
            c_int,
            mpi_sys::MPI_Aint,
            c_int,
            mpi_sys::MPI_Datatype,
            mpi_sys::MPI_Win,
            *mut mpi_sys::MPI_Request,
        ) -> c_int,
    {
        next_f(
            origin_addr,
            origin_count,
            origin_datatype,
            target_rank,
            target_disp,
            target_count,
            target_datatype,
            win,
            request,
        )
    }
    #[inline]
    fn rget_accumulate<F>(
        &self,
        next_f: F,
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
    ) -> c_int
    where
        F: FnOnce(
            *const c_void,
            c_int,
            mpi_sys::MPI_Datatype,
            *mut c_void,
            c_int,
            mpi_sys::MPI_Datatype,
            c_int,
            mpi_sys::MPI_Aint,
            c_int,
            mpi_sys::MPI_Datatype,
            mpi_sys::MPI_Op,
            mpi_sys::MPI_Win,
            *mut mpi_sys::MPI_Request,
        ) -> c_int,
    {
        next_f(
            origin_addr,
            origin_count,
            origin_datatype,
            result_addr,
            result_count,
            result_datatype,
            target_rank,
            target_disp,
            target_count,
            target_datatype,
            op,
            win,
            request,
        )
    }
    #[inline]
    fn rput<F>(
        &self,
        next_f: F,
        origin_addr: *const c_void,
        origin_count: c_int,
        origin_datatype: mpi_sys::MPI_Datatype,
        target_rank: c_int,
        target_disp: mpi_sys::MPI_Aint,
        target_cout: c_int,
        target_datatype: mpi_sys::MPI_Datatype,
        win: mpi_sys::MPI_Win,
        request: *mut mpi_sys::MPI_Request,
    ) -> c_int
    where
        F: FnOnce(
            *const c_void,
            c_int,
            mpi_sys::MPI_Datatype,
            c_int,
            mpi_sys::MPI_Aint,
            c_int,
            mpi_sys::MPI_Datatype,
            mpi_sys::MPI_Win,
            *mut mpi_sys::MPI_Request,
        ) -> c_int,
    {
        next_f(
            origin_addr,
            origin_count,
            origin_datatype,
            target_rank,
            target_disp,
            target_cout,
            target_datatype,
            win,
            request,
        )
    }
    #[inline]
    fn rsend<F>(
        &self,
        next_f: F,
        ibuf: *const c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        dest: c_int,
        tag: c_int,
        comm: mpi_sys::MPI_Comm,
    ) -> c_int
    where
        F: FnOnce(
            *const c_void,
            c_int,
            mpi_sys::MPI_Datatype,
            c_int,
            c_int,
            mpi_sys::MPI_Comm,
        ) -> c_int,
    {
        next_f(ibuf, count, datatype, dest, tag, comm)
    }
    #[inline]
    fn rsend_init<F>(
        &self,
        next_f: F,
        buf: *const c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        dest: c_int,
        tag: c_int,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) -> c_int
    where
        F: FnOnce(
            *const c_void,
            c_int,
            mpi_sys::MPI_Datatype,
            c_int,
            c_int,
            mpi_sys::MPI_Comm,
            *mut mpi_sys::MPI_Request,
        ) -> c_int,
    {
        next_f(buf, count, datatype, dest, tag, comm, request)
    }
    #[inline]
    fn scan<F>(
        &self,
        next_f: F,
        sendbuf: *const c_void,
        recvbuf: *mut c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        op: mpi_sys::MPI_Op,
        comm: mpi_sys::MPI_Comm,
    ) -> c_int
    where
        F: FnOnce(
            *const c_void,
            *mut c_void,
            c_int,
            mpi_sys::MPI_Datatype,
            mpi_sys::MPI_Op,
            mpi_sys::MPI_Comm,
        ) -> c_int,
    {
        next_f(sendbuf, recvbuf, count, datatype, op, comm)
    }
    #[inline]
    fn scatter<F>(
        &self,
        next_f: F,
        sendbuf: *const c_void,
        sendcount: c_int,
        sendtype: mpi_sys::MPI_Datatype,
        recvbuf: *mut c_void,
        recvcount: c_int,
        recvtype: mpi_sys::MPI_Datatype,
        root: c_int,
        comm: mpi_sys::MPI_Comm,
    ) -> c_int
    where
        F: FnOnce(
            *const c_void,
            c_int,
            mpi_sys::MPI_Datatype,
            *mut c_void,
            c_int,
            mpi_sys::MPI_Datatype,
            c_int,
            mpi_sys::MPI_Comm,
        ) -> c_int,
    {
        next_f(
            sendbuf, sendcount, sendtype, recvbuf, recvcount, recvtype, root, comm,
        )
    }
    #[inline]
    fn scatterv<F>(
        &self,
        next_f: F,
        sendbuf: *const c_void,
        sendcounts: *const c_int,
        displs: *const c_int,
        sendtype: mpi_sys::MPI_Datatype,
        recvbuf: *mut c_void,
        recvcount: c_int,
        recvtype: mpi_sys::MPI_Datatype,
        root: c_int,
        comm: mpi_sys::MPI_Comm,
    ) -> c_int
    where
        F: FnOnce(
            *const c_void,
            *const c_int,
            *const c_int,
            mpi_sys::MPI_Datatype,
            *mut c_void,
            c_int,
            mpi_sys::MPI_Datatype,
            c_int,
            mpi_sys::MPI_Comm,
        ) -> c_int,
    {
        next_f(
            sendbuf, sendcounts, displs, sendtype, recvbuf, recvcount, recvtype, root, comm,
        )
    }
    #[inline]
    fn send<F>(
        &self,
        next_f: F,
        buf: *const c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        dest: c_int,
        tag: c_int,
        comm: mpi_sys::MPI_Comm,
    ) -> c_int
    where
        F: FnOnce(
            *const c_void,
            c_int,
            mpi_sys::MPI_Datatype,
            c_int,
            c_int,
            mpi_sys::MPI_Comm,
        ) -> c_int,
    {
        next_f(buf, count, datatype, dest, tag, comm)
    }
    #[inline]
    fn send_init<F>(
        &self,
        next_f: F,
        buf: *const c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        dest: c_int,
        tag: c_int,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) -> c_int
    where
        F: FnOnce(
            *const c_void,
            c_int,
            mpi_sys::MPI_Datatype,
            c_int,
            c_int,
            mpi_sys::MPI_Comm,
            *mut mpi_sys::MPI_Request,
        ) -> c_int,
    {
        next_f(buf, count, datatype, dest, tag, comm, request)
    }
    #[inline]
    fn sendrecv<F>(
        &self,
        next_f: F,
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
    ) -> c_int
    where
        F: FnOnce(
            *const c_void,
            c_int,
            mpi_sys::MPI_Datatype,
            c_int,
            c_int,
            *mut c_void,
            c_int,
            mpi_sys::MPI_Datatype,
            c_int,
            c_int,
            mpi_sys::MPI_Comm,
            *mut mpi_sys::MPI_Status,
        ) -> c_int,
    {
        next_f(
            sendbuf, sendcount, sendtype, dest, sendtag, recvbuf, recvcount, recvtype, source,
            recvtag, comm, status,
        )
    }
    #[inline]
    fn sendrecv_replace<F>(
        &self,
        next_f: F,
        buf: *mut c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        dest: c_int,
        sendtag: c_int,
        source: c_int,
        recvtag: c_int,
        comm: mpi_sys::MPI_Comm,
        status: *mut mpi_sys::MPI_Status,
    ) -> c_int
    where
        F: FnOnce(
            *mut c_void,
            c_int,
            mpi_sys::MPI_Datatype,
            c_int,
            c_int,
            c_int,
            c_int,
            mpi_sys::MPI_Comm,
            *mut mpi_sys::MPI_Status,
        ) -> c_int,
    {
        next_f(
            buf, count, datatype, dest, sendtag, source, recvtag, comm, status,
        )
    }
    #[inline]
    fn ssend<F>(
        &self,
        next_f: F,
        buf: *const c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        dest: c_int,
        tag: c_int,
        comm: mpi_sys::MPI_Comm,
    ) -> c_int
    where
        F: FnOnce(
            *const c_void,
            c_int,
            mpi_sys::MPI_Datatype,
            c_int,
            c_int,
            mpi_sys::MPI_Comm,
        ) -> c_int,
    {
        next_f(buf, count, datatype, dest, tag, comm)
    }
    #[inline]
    fn ssend_init<F>(
        &self,
        next_f: F,
        buf: *const c_void,
        count: c_int,
        datatype: mpi_sys::MPI_Datatype,
        dest: c_int,
        tag: c_int,
        comm: mpi_sys::MPI_Comm,
        request: *mut mpi_sys::MPI_Request,
    ) -> c_int
    where
        F: FnOnce(
            *const c_void,
            c_int,
            mpi_sys::MPI_Datatype,
            c_int,
            c_int,
            mpi_sys::MPI_Comm,
            *mut mpi_sys::MPI_Request,
        ) -> c_int,
    {
        next_f(buf, count, datatype, dest, tag, comm, request)
    }
    #[inline]
    fn start<F>(&self, next_f: F, request: *mut mpi_sys::MPI_Request) -> c_int
    where
        F: FnOnce(*mut mpi_sys::MPI_Request) -> c_int,
    {
        next_f(request)
    }
    #[inline]
    fn startall<F>(
        &self,
        next_f: F,
        count: c_int,
        array_of_requests: *mut mpi_sys::MPI_Request,
    ) -> c_int
    where
        F: FnOnce(c_int, *mut mpi_sys::MPI_Request) -> c_int,
    {
        next_f(count, array_of_requests)
    }
    #[inline]
    fn status_set_cancelled<F>(
        &self,
        next_f: F,
        status: *mut mpi_sys::MPI_Status,
        flag: c_int,
    ) -> c_int
    where
        F: FnOnce(*mut mpi_sys::MPI_Status, c_int) -> c_int,
    {
        next_f(status, flag)
    }
    #[inline]
    fn status_set_elements<F>(
        &self,
        next_f: F,
        status: *mut mpi_sys::MPI_Status,
        datatype: mpi_sys::MPI_Datatype,
        count: c_int,
    ) -> c_int
    where
        F: FnOnce(*mut mpi_sys::MPI_Status, mpi_sys::MPI_Datatype, c_int) -> c_int,
    {
        next_f(status, datatype, count)
    }
    #[inline]
    fn status_set_elements_x<F>(
        &self,
        next_f: F,
        status: *mut mpi_sys::MPI_Status,
        datatype: mpi_sys::MPI_Datatype,
        count: mpi_sys::MPI_Count,
    ) -> c_int
    where
        F: FnOnce(*mut mpi_sys::MPI_Status, mpi_sys::MPI_Datatype, mpi_sys::MPI_Count) -> c_int,
    {
        next_f(status, datatype, count)
    }
    #[inline]
    fn test<F>(
        &self,
        next_f: F,
        request: *mut mpi_sys::MPI_Request,
        flag: *mut c_int,
        status: *mut mpi_sys::MPI_Status,
    ) -> c_int
    where
        F: FnOnce(*mut mpi_sys::MPI_Request, *mut c_int, *mut mpi_sys::MPI_Status) -> c_int,
    {
        next_f(request, flag, status)
    }
    #[inline]
    fn test_cancelled<F>(
        &self,
        next_f: F,
        status: *const mpi_sys::MPI_Status,
        flag: *mut c_int,
    ) -> c_int
    where
        F: FnOnce(*const mpi_sys::MPI_Status, *mut c_int) -> c_int,
    {
        next_f(status, flag)
    }
    #[inline]
    fn testall<F>(
        &self,
        next_f: F,
        count: c_int,
        array_of_requests: *mut mpi_sys::MPI_Request,
        flag: *mut c_int,
        array_of_statuses: *mut mpi_sys::MPI_Status,
    ) -> c_int
    where
        F: FnOnce(c_int, *mut mpi_sys::MPI_Request, *mut c_int, *mut mpi_sys::MPI_Status) -> c_int,
    {
        next_f(count, array_of_requests, flag, array_of_statuses)
    }
    #[inline]
    fn testany<F>(
        &self,
        next_f: F,
        count: c_int,
        array_of_requests: *mut mpi_sys::MPI_Request,
        index: *mut c_int,
        flag: *mut c_int,
        status: *mut mpi_sys::MPI_Status,
    ) -> c_int
    where
        F: FnOnce(
            c_int,
            *mut mpi_sys::MPI_Request,
            *mut c_int,
            *mut c_int,
            *mut mpi_sys::MPI_Status,
        ) -> c_int,
    {
        next_f(count, array_of_requests, index, flag, status)
    }
    #[inline]
    fn testsome<F>(
        &self,
        next_f: F,
        incount: c_int,
        array_of_requests: *mut mpi_sys::MPI_Request,
        outcount: *mut c_int,
        array_of_indices: *mut c_int,
        array_of_statuses: *mut mpi_sys::MPI_Status,
    ) -> c_int
    where
        F: FnOnce(
            c_int,
            *mut mpi_sys::MPI_Request,
            *mut c_int,
            *mut c_int,
            *mut mpi_sys::MPI_Status,
        ) -> c_int,
    {
        next_f(
            incount,
            array_of_requests,
            outcount,
            array_of_indices,
            array_of_statuses,
        )
    }
    #[inline]
    fn topo_test<F>(&self, next_f: F, comm: mpi_sys::MPI_Comm, status: *mut c_int) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Comm, *mut c_int) -> c_int,
    {
        next_f(comm, status)
    }
    #[inline]
    fn type_commit<F>(&self, next_f: F, r#type: *mut mpi_sys::MPI_Datatype) -> c_int
    where
        F: FnOnce(*mut mpi_sys::MPI_Datatype) -> c_int,
    {
        next_f(r#type)
    }
    #[inline]
    fn type_contiguous<F>(
        &self,
        next_f: F,
        count: c_int,
        oldtype: mpi_sys::MPI_Datatype,
        newtype: *mut mpi_sys::MPI_Datatype,
    ) -> c_int
    where
        F: FnOnce(c_int, mpi_sys::MPI_Datatype, *mut mpi_sys::MPI_Datatype) -> c_int,
    {
        next_f(count, oldtype, newtype)
    }
    #[inline]
    fn type_create_darray<F>(
        &self,
        next_f: F,
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
    ) -> c_int
    where
        F: FnOnce(
            c_int,
            c_int,
            c_int,
            *const c_int,
            *const c_int,
            *const c_int,
            *const c_int,
            c_int,
            mpi_sys::MPI_Datatype,
            *mut mpi_sys::MPI_Datatype,
        ) -> c_int,
    {
        next_f(
            size,
            rank,
            ndims,
            gsize_array,
            distrib_array,
            darg_array,
            psize_array,
            order,
            oldtype,
            newtype,
        )
    }
    #[inline]
    fn type_create_f90_complex<F>(
        &self,
        next_f: F,
        p: c_int,
        r: c_int,
        newtype: *mut mpi_sys::MPI_Datatype,
    ) -> c_int
    where
        F: FnOnce(c_int, c_int, *mut mpi_sys::MPI_Datatype) -> c_int,
    {
        next_f(p, r, newtype)
    }
    #[inline]
    fn type_create_f90_integer<F>(
        &self,
        next_f: F,
        r: c_int,
        newtype: *mut mpi_sys::MPI_Datatype,
    ) -> c_int
    where
        F: FnOnce(c_int, *mut mpi_sys::MPI_Datatype) -> c_int,
    {
        next_f(r, newtype)
    }
    #[inline]
    fn type_create_f90_real<F>(
        &self,
        next_f: F,
        p: c_int,
        r: c_int,
        newtype: *mut mpi_sys::MPI_Datatype,
    ) -> c_int
    where
        F: FnOnce(c_int, c_int, *mut mpi_sys::MPI_Datatype) -> c_int,
    {
        next_f(p, r, newtype)
    }
    #[inline]
    fn type_create_hindexed<F>(
        &self,
        next_f: F,
        count: c_int,
        array_of_blocklengths: *const c_int,
        array_of_displacements: *const mpi_sys::MPI_Aint,
        oldtype: mpi_sys::MPI_Datatype,
        newtype: *mut mpi_sys::MPI_Datatype,
    ) -> c_int
    where
        F: FnOnce(
            c_int,
            *const c_int,
            *const mpi_sys::MPI_Aint,
            mpi_sys::MPI_Datatype,
            *mut mpi_sys::MPI_Datatype,
        ) -> c_int,
    {
        next_f(
            count,
            array_of_blocklengths,
            array_of_displacements,
            oldtype,
            newtype,
        )
    }
    #[inline]
    fn type_create_hindexed_block<F>(
        &self,
        next_f: F,
        count: c_int,
        blocklength: c_int,
        array_of_displacements: *const mpi_sys::MPI_Aint,
        oldtype: mpi_sys::MPI_Datatype,
        newtype: *mut mpi_sys::MPI_Datatype,
    ) -> c_int
    where
        F: FnOnce(
            c_int,
            c_int,
            *const mpi_sys::MPI_Aint,
            mpi_sys::MPI_Datatype,
            *mut mpi_sys::MPI_Datatype,
        ) -> c_int,
    {
        next_f(count, blocklength, array_of_displacements, oldtype, newtype)
    }
    #[inline]
    fn type_create_hvector<F>(
        &self,
        next_f: F,
        count: c_int,
        blocklength: c_int,
        stride: mpi_sys::MPI_Aint,
        oldtype: mpi_sys::MPI_Datatype,
        newtype: *mut mpi_sys::MPI_Datatype,
    ) -> c_int
    where
        F: FnOnce(
            c_int,
            c_int,
            mpi_sys::MPI_Aint,
            mpi_sys::MPI_Datatype,
            *mut mpi_sys::MPI_Datatype,
        ) -> c_int,
    {
        next_f(count, blocklength, stride, oldtype, newtype)
    }
    #[inline]
    fn type_create_indexed_block<F>(
        &self,
        next_f: F,
        count: c_int,
        blocklength: c_int,
        array_of_displacements: *const c_int,
        oldtype: mpi_sys::MPI_Datatype,
        newtype: *mut mpi_sys::MPI_Datatype,
    ) -> c_int
    where
        F: FnOnce(
            c_int,
            c_int,
            *const c_int,
            mpi_sys::MPI_Datatype,
            *mut mpi_sys::MPI_Datatype,
        ) -> c_int,
    {
        next_f(count, blocklength, array_of_displacements, oldtype, newtype)
    }
    #[inline]
    fn type_create_keyval<F>(
        &self,
        next_f: F,
        type_copy_attr_fn: *mut mpi_sys::MPI_Type_copy_attr_function,
        type_delete_attr_fn: *mut mpi_sys::MPI_Type_delete_attr_function,
        type_keyval: *mut c_int,
        extra_state: *mut c_void,
    ) -> c_int
    where
        F: FnOnce(
            *mut mpi_sys::MPI_Type_copy_attr_function,
            *mut mpi_sys::MPI_Type_delete_attr_function,
            *mut c_int,
            *mut c_void,
        ) -> c_int,
    {
        next_f(
            type_copy_attr_fn,
            type_delete_attr_fn,
            type_keyval,
            extra_state,
        )
    }
    #[inline]
    fn type_create_resized<F>(
        &self,
        next_f: F,
        oldtype: mpi_sys::MPI_Datatype,
        lb: mpi_sys::MPI_Aint,
        extent: mpi_sys::MPI_Aint,
        newtype: *mut mpi_sys::MPI_Datatype,
    ) -> c_int
    where
        F: FnOnce(
            mpi_sys::MPI_Datatype,
            mpi_sys::MPI_Aint,
            mpi_sys::MPI_Aint,
            *mut mpi_sys::MPI_Datatype,
        ) -> c_int,
    {
        next_f(oldtype, lb, extent, newtype)
    }
    #[inline]
    fn type_create_struct<F>(
        &self,
        next_f: F,
        count: c_int,
        array_of_block_lengths: *const c_int,
        array_of_displacements: *const mpi_sys::MPI_Aint,
        array_of_types: *const mpi_sys::MPI_Datatype,
        newtype: *mut mpi_sys::MPI_Datatype,
    ) -> c_int
    where
        F: FnOnce(
            c_int,
            *const c_int,
            *const mpi_sys::MPI_Aint,
            *const mpi_sys::MPI_Datatype,
            *mut mpi_sys::MPI_Datatype,
        ) -> c_int,
    {
        next_f(
            count,
            array_of_block_lengths,
            array_of_displacements,
            array_of_types,
            newtype,
        )
    }
    #[inline]
    fn type_create_subarray<F>(
        &self,
        next_f: F,
        ndims: c_int,
        size_array: *const c_int,
        subsize_array: *const c_int,
        start_array: *const c_int,
        order: c_int,
        oldtype: mpi_sys::MPI_Datatype,
        newtype: *mut mpi_sys::MPI_Datatype,
    ) -> c_int
    where
        F: FnOnce(
            c_int,
            *const c_int,
            *const c_int,
            *const c_int,
            c_int,
            mpi_sys::MPI_Datatype,
            *mut mpi_sys::MPI_Datatype,
        ) -> c_int,
    {
        next_f(
            ndims,
            size_array,
            subsize_array,
            start_array,
            order,
            oldtype,
            newtype,
        )
    }
    #[inline]
    fn type_delete_attr<F>(
        &self,
        next_f: F,
        r#type: mpi_sys::MPI_Datatype,
        type_keyval: c_int,
    ) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Datatype, c_int) -> c_int,
    {
        next_f(r#type, type_keyval)
    }
    #[inline]
    fn type_dup<F>(
        &self,
        next_f: F,
        r#type: mpi_sys::MPI_Datatype,
        newtype: *mut mpi_sys::MPI_Datatype,
    ) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Datatype, *mut mpi_sys::MPI_Datatype) -> c_int,
    {
        next_f(r#type, newtype)
    }
    #[inline]
    fn type_extent<F>(
        &self,
        next_f: F,
        r#type: mpi_sys::MPI_Datatype,
        extent: *mut mpi_sys::MPI_Aint,
    ) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Datatype, *mut mpi_sys::MPI_Aint) -> c_int,
    {
        next_f(r#type, extent)
    }
    #[inline]
    fn type_free<F>(&self, next_f: F, r#type: *mut mpi_sys::MPI_Datatype) -> c_int
    where
        F: FnOnce(*mut mpi_sys::MPI_Datatype) -> c_int,
    {
        next_f(r#type)
    }
    #[inline]
    fn type_free_keyval<F>(&self, next_f: F, type_keyval: *mut c_int) -> c_int
    where
        F: FnOnce(*mut c_int) -> c_int,
    {
        next_f(type_keyval)
    }
    #[inline]
    fn type_get_attr<F>(
        &self,
        next_f: F,
        r#type: mpi_sys::MPI_Datatype,
        type_keyval: c_int,
        attribute_val: *mut c_void,
        flag: *mut c_int,
    ) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Datatype, c_int, *mut c_void, *mut c_int) -> c_int,
    {
        next_f(r#type, type_keyval, attribute_val, flag)
    }
    #[inline]
    fn type_get_contents<F>(
        &self,
        next_f: F,
        mtype: mpi_sys::MPI_Datatype,
        max_integers: c_int,
        max_addresses: c_int,
        max_datatypes: c_int,
        array_of_integers: *mut c_int,
        array_of_addresses: *mut mpi_sys::MPI_Aint,
        array_of_datatypes: *mut mpi_sys::MPI_Datatype,
    ) -> c_int
    where
        F: FnOnce(
            mpi_sys::MPI_Datatype,
            c_int,
            c_int,
            c_int,
            *mut c_int,
            *mut mpi_sys::MPI_Aint,
            *mut mpi_sys::MPI_Datatype,
        ) -> c_int,
    {
        next_f(
            mtype,
            max_integers,
            max_addresses,
            max_datatypes,
            array_of_integers,
            array_of_addresses,
            array_of_datatypes,
        )
    }
    #[inline]
    fn type_get_envelope<F>(
        &self,
        next_f: F,
        r#type: mpi_sys::MPI_Datatype,
        num_integers: *mut c_int,
        num_addresses: *mut c_int,
        num_datatypes: *mut c_int,
        combiner: *mut c_int,
    ) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Datatype, *mut c_int, *mut c_int, *mut c_int, *mut c_int) -> c_int,
    {
        next_f(r#type, num_integers, num_addresses, num_datatypes, combiner)
    }
    #[inline]
    fn type_get_extent<F>(
        &self,
        next_f: F,
        r#type: mpi_sys::MPI_Datatype,
        lb: *mut mpi_sys::MPI_Aint,
        extent: *mut mpi_sys::MPI_Aint,
    ) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Datatype, *mut mpi_sys::MPI_Aint, *mut mpi_sys::MPI_Aint) -> c_int,
    {
        next_f(r#type, lb, extent)
    }
    #[inline]
    fn type_get_extent_x<F>(
        &self,
        next_f: F,
        r#type: mpi_sys::MPI_Datatype,
        lb: *mut mpi_sys::MPI_Count,
        extent: *mut mpi_sys::MPI_Count,
    ) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Datatype, *mut mpi_sys::MPI_Count, *mut mpi_sys::MPI_Count) -> c_int,
    {
        next_f(r#type, lb, extent)
    }
    #[inline]
    fn type_get_name<F>(
        &self,
        next_f: F,
        r#type: mpi_sys::MPI_Datatype,
        type_name: *mut c_char,
        resultlen: *mut c_int,
    ) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Datatype, *mut c_char, *mut c_int) -> c_int,
    {
        next_f(r#type, type_name, resultlen)
    }
    #[inline]
    fn type_get_true_extent<F>(
        &self,
        next_f: F,
        datatype: mpi_sys::MPI_Datatype,
        true_lb: *mut mpi_sys::MPI_Aint,
        true_extent: *mut mpi_sys::MPI_Aint,
    ) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Datatype, *mut mpi_sys::MPI_Aint, *mut mpi_sys::MPI_Aint) -> c_int,
    {
        next_f(datatype, true_lb, true_extent)
    }
    #[inline]
    fn type_get_true_extent_x<F>(
        &self,
        next_f: F,
        datatype: mpi_sys::MPI_Datatype,
        true_lb: *mut mpi_sys::MPI_Count,
        true_extent: *mut mpi_sys::MPI_Count,
    ) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Datatype, *mut mpi_sys::MPI_Count, *mut mpi_sys::MPI_Count) -> c_int,
    {
        next_f(datatype, true_lb, true_extent)
    }
    #[inline]
    fn type_hindexed<F>(
        &self,
        next_f: F,
        count: c_int,
        array_of_blocklengths: *mut c_int,
        array_of_displacements: *mut mpi_sys::MPI_Aint,
        oldtype: mpi_sys::MPI_Datatype,
        newtype: *mut mpi_sys::MPI_Datatype,
    ) -> c_int
    where
        F: FnOnce(
            c_int,
            *mut c_int,
            *mut mpi_sys::MPI_Aint,
            mpi_sys::MPI_Datatype,
            *mut mpi_sys::MPI_Datatype,
        ) -> c_int,
    {
        next_f(
            count,
            array_of_blocklengths,
            array_of_displacements,
            oldtype,
            newtype,
        )
    }
    #[inline]
    fn type_hvector<F>(
        &self,
        next_f: F,
        count: c_int,
        blocklength: c_int,
        stride: mpi_sys::MPI_Aint,
        oldtype: mpi_sys::MPI_Datatype,
        newtype: *mut mpi_sys::MPI_Datatype,
    ) -> c_int
    where
        F: FnOnce(
            c_int,
            c_int,
            mpi_sys::MPI_Aint,
            mpi_sys::MPI_Datatype,
            *mut mpi_sys::MPI_Datatype,
        ) -> c_int,
    {
        next_f(count, blocklength, stride, oldtype, newtype)
    }
    #[inline]
    fn type_indexed<F>(
        &self,
        next_f: F,
        count: c_int,
        array_of_blocklengths: *const c_int,
        array_of_displacements: *const c_int,
        oldtype: mpi_sys::MPI_Datatype,
        newtype: *mut mpi_sys::MPI_Datatype,
    ) -> c_int
    where
        F: FnOnce(
            c_int,
            *const c_int,
            *const c_int,
            mpi_sys::MPI_Datatype,
            *mut mpi_sys::MPI_Datatype,
        ) -> c_int,
    {
        next_f(
            count,
            array_of_blocklengths,
            array_of_displacements,
            oldtype,
            newtype,
        )
    }
    #[inline]
    fn type_lb<F>(
        &self,
        next_f: F,
        r#type: mpi_sys::MPI_Datatype,
        lb: *mut mpi_sys::MPI_Aint,
    ) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Datatype, *mut mpi_sys::MPI_Aint) -> c_int,
    {
        next_f(r#type, lb)
    }
    #[inline]
    fn type_match_size<F>(
        &self,
        next_f: F,
        typeclass: c_int,
        size: c_int,
        r#type: *mut mpi_sys::MPI_Datatype,
    ) -> c_int
    where
        F: FnOnce(c_int, c_int, *mut mpi_sys::MPI_Datatype) -> c_int,
    {
        next_f(typeclass, size, r#type)
    }
    #[inline]
    fn type_set_attr<F>(
        &self,
        next_f: F,
        r#type: mpi_sys::MPI_Datatype,
        type_keyval: c_int,
        attr_val: *mut c_void,
    ) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Datatype, c_int, *mut c_void) -> c_int,
    {
        next_f(r#type, type_keyval, attr_val)
    }
    #[inline]
    fn type_set_name<F>(
        &self,
        next_f: F,
        r#type: mpi_sys::MPI_Datatype,
        type_name: *const c_char,
    ) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Datatype, *const c_char) -> c_int,
    {
        next_f(r#type, type_name)
    }
    #[inline]
    fn type_size<F>(&self, next_f: F, r#type: mpi_sys::MPI_Datatype, size: *mut c_int) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Datatype, *mut c_int) -> c_int,
    {
        next_f(r#type, size)
    }
    #[inline]
    fn type_size_x<F>(
        &self,
        next_f: F,
        r#type: mpi_sys::MPI_Datatype,
        size: *mut mpi_sys::MPI_Count,
    ) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Datatype, *mut mpi_sys::MPI_Count) -> c_int,
    {
        next_f(r#type, size)
    }
    #[inline]
    fn type_struct<F>(
        &self,
        next_f: F,
        count: c_int,
        array_of_blocklengths: *mut c_int,
        array_of_displacements: *mut mpi_sys::MPI_Aint,
        array_of_types: *mut mpi_sys::MPI_Datatype,
        newtype: *mut mpi_sys::MPI_Datatype,
    ) -> c_int
    where
        F: FnOnce(
            c_int,
            *mut c_int,
            *mut mpi_sys::MPI_Aint,
            *mut mpi_sys::MPI_Datatype,
            *mut mpi_sys::MPI_Datatype,
        ) -> c_int,
    {
        next_f(
            count,
            array_of_blocklengths,
            array_of_displacements,
            array_of_types,
            newtype,
        )
    }
    #[inline]
    fn type_ub<F>(
        &self,
        next_f: F,
        mtype: mpi_sys::MPI_Datatype,
        ub: *mut mpi_sys::MPI_Aint,
    ) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Datatype, *mut mpi_sys::MPI_Aint) -> c_int,
    {
        next_f(mtype, ub)
    }
    #[inline]
    fn type_vector<F>(
        &self,
        next_f: F,
        count: c_int,
        blocklength: c_int,
        stride: c_int,
        oldtype: mpi_sys::MPI_Datatype,
        newtype: *mut mpi_sys::MPI_Datatype,
    ) -> c_int
    where
        F: FnOnce(c_int, c_int, c_int, mpi_sys::MPI_Datatype, *mut mpi_sys::MPI_Datatype) -> c_int,
    {
        next_f(count, blocklength, stride, oldtype, newtype)
    }
    #[inline]
    fn unpack<F>(
        &self,
        next_f: F,
        inbuf: *const c_void,
        insize: c_int,
        position: *mut c_int,
        outbuf: *mut c_void,
        outcount: c_int,
        datatype: mpi_sys::MPI_Datatype,
        comm: mpi_sys::MPI_Comm,
    ) -> c_int
    where
        F: FnOnce(
            *const c_void,
            c_int,
            *mut c_int,
            *mut c_void,
            c_int,
            mpi_sys::MPI_Datatype,
            mpi_sys::MPI_Comm,
        ) -> c_int,
    {
        next_f(inbuf, insize, position, outbuf, outcount, datatype, comm)
    }
    #[inline]
    fn unpack_external<F>(
        &self,
        next_f: F,
        datarep: *const c_char,
        inbuf: *const c_void,
        insize: mpi_sys::MPI_Aint,
        position: *mut mpi_sys::MPI_Aint,
        outbuf: *mut c_void,
        outcount: c_int,
        datatype: mpi_sys::MPI_Datatype,
    ) -> c_int
    where
        F: FnOnce(
            *const c_char,
            *const c_void,
            mpi_sys::MPI_Aint,
            *mut mpi_sys::MPI_Aint,
            *mut c_void,
            c_int,
            mpi_sys::MPI_Datatype,
        ) -> c_int,
    {
        next_f(datarep, inbuf, insize, position, outbuf, outcount, datatype)
    }
    #[inline]
    fn unpublish_name<F>(
        &self,
        next_f: F,
        service_name: *const c_char,
        info: mpi_sys::MPI_Info,
        port_name: *const c_char,
    ) -> c_int
    where
        F: FnOnce(*const c_char, mpi_sys::MPI_Info, *const c_char) -> c_int,
    {
        next_f(service_name, info, port_name)
    }
    #[inline]
    fn wait<F>(
        &self,
        next_f: F,
        request: *mut mpi_sys::MPI_Request,
        status: *mut mpi_sys::MPI_Status,
    ) -> c_int
    where
        F: FnOnce(*mut mpi_sys::MPI_Request, *mut mpi_sys::MPI_Status) -> c_int,
    {
        next_f(request, status)
    }
    #[inline]
    fn waitall<F>(
        &self,
        next_f: F,
        count: c_int,
        array_of_requests: *mut mpi_sys::MPI_Request,
        array_of_statuses: *mut mpi_sys::MPI_Status,
    ) -> c_int
    where
        F: FnOnce(c_int, *mut mpi_sys::MPI_Request, *mut mpi_sys::MPI_Status) -> c_int,
    {
        next_f(count, array_of_requests, array_of_statuses)
    }
    #[inline]
    fn waitany<F>(
        &self,
        next_f: F,
        count: c_int,
        array_of_requests: *mut mpi_sys::MPI_Request,
        index: *mut c_int,
        status: *mut mpi_sys::MPI_Status,
    ) -> c_int
    where
        F: FnOnce(c_int, *mut mpi_sys::MPI_Request, *mut c_int, *mut mpi_sys::MPI_Status) -> c_int,
    {
        next_f(count, array_of_requests, index, status)
    }
    #[inline]
    fn waitsome<F>(
        &self,
        next_f: F,
        incount: c_int,
        array_of_requests: *mut mpi_sys::MPI_Request,
        outcount: *mut c_int,
        array_of_indices: *mut c_int,
        array_of_statuses: *mut mpi_sys::MPI_Status,
    ) -> c_int
    where
        F: FnOnce(
            c_int,
            *mut mpi_sys::MPI_Request,
            *mut c_int,
            *mut c_int,
            *mut mpi_sys::MPI_Status,
        ) -> c_int,
    {
        next_f(
            incount,
            array_of_requests,
            outcount,
            array_of_indices,
            array_of_statuses,
        )
    }
    #[inline]
    fn win_allocate<F>(
        &self,
        next_f: F,
        size: mpi_sys::MPI_Aint,
        disp_unit: c_int,
        info: mpi_sys::MPI_Info,
        comm: mpi_sys::MPI_Comm,
        baseptr: *mut c_void,
        win: *mut mpi_sys::MPI_Win,
    ) -> c_int
    where
        F: FnOnce(
            mpi_sys::MPI_Aint,
            c_int,
            mpi_sys::MPI_Info,
            mpi_sys::MPI_Comm,
            *mut c_void,
            *mut mpi_sys::MPI_Win,
        ) -> c_int,
    {
        next_f(size, disp_unit, info, comm, baseptr, win)
    }
    #[inline]
    fn win_allocate_shared<F>(
        &self,
        next_f: F,
        size: mpi_sys::MPI_Aint,
        disp_unit: c_int,
        info: mpi_sys::MPI_Info,
        comm: mpi_sys::MPI_Comm,
        baseptr: *mut c_void,
        win: *mut mpi_sys::MPI_Win,
    ) -> c_int
    where
        F: FnOnce(
            mpi_sys::MPI_Aint,
            c_int,
            mpi_sys::MPI_Info,
            mpi_sys::MPI_Comm,
            *mut c_void,
            *mut mpi_sys::MPI_Win,
        ) -> c_int,
    {
        next_f(size, disp_unit, info, comm, baseptr, win)
    }
    #[inline]
    fn win_attach<F>(
        &self,
        next_f: F,
        win: mpi_sys::MPI_Win,
        base: *mut c_void,
        size: mpi_sys::MPI_Aint,
    ) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Win, *mut c_void, mpi_sys::MPI_Aint) -> c_int,
    {
        next_f(win, base, size)
    }
    #[inline]
    fn win_call_errhandler<F>(&self, next_f: F, win: mpi_sys::MPI_Win, errorcode: c_int) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Win, c_int) -> c_int,
    {
        next_f(win, errorcode)
    }
    #[inline]
    fn win_complete<F>(&self, next_f: F, win: mpi_sys::MPI_Win) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Win) -> c_int,
    {
        next_f(win)
    }
    #[inline]
    fn win_create<F>(
        &self,
        next_f: F,
        base: *mut c_void,
        size: mpi_sys::MPI_Aint,
        disp_unit: c_int,
        info: mpi_sys::MPI_Info,
        comm: mpi_sys::MPI_Comm,
        win: *mut mpi_sys::MPI_Win,
    ) -> c_int
    where
        F: FnOnce(
            *mut c_void,
            mpi_sys::MPI_Aint,
            c_int,
            mpi_sys::MPI_Info,
            mpi_sys::MPI_Comm,
            *mut mpi_sys::MPI_Win,
        ) -> c_int,
    {
        next_f(base, size, disp_unit, info, comm, win)
    }
    #[inline]
    fn win_create_dynamic<F>(
        &self,
        next_f: F,
        info: mpi_sys::MPI_Info,
        comm: mpi_sys::MPI_Comm,
        win: *mut mpi_sys::MPI_Win,
    ) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Info, mpi_sys::MPI_Comm, *mut mpi_sys::MPI_Win) -> c_int,
    {
        next_f(info, comm, win)
    }
    #[inline]
    fn win_create_errhandler<F>(
        &self,
        next_f: F,
        function: *mut mpi_sys::MPI_Win_errhandler_function,
        errhandler: *mut mpi_sys::MPI_Errhandler,
    ) -> c_int
    where
        F: FnOnce(*mut mpi_sys::MPI_Win_errhandler_function, *mut mpi_sys::MPI_Errhandler) -> c_int,
    {
        next_f(function, errhandler)
    }
    #[inline]
    fn win_create_keyval<F>(
        &self,
        next_f: F,
        win_copy_attr_fn: *mut mpi_sys::MPI_Win_copy_attr_function,
        win_delete_attr_fn: *mut mpi_sys::MPI_Win_delete_attr_function,
        win_keyval: *mut c_int,
        extra_state: *mut c_void,
    ) -> c_int
    where
        F: FnOnce(
            *mut mpi_sys::MPI_Win_copy_attr_function,
            *mut mpi_sys::MPI_Win_delete_attr_function,
            *mut c_int,
            *mut c_void,
        ) -> c_int,
    {
        next_f(
            win_copy_attr_fn,
            win_delete_attr_fn,
            win_keyval,
            extra_state,
        )
    }
    #[inline]
    fn win_delete_attr<F>(&self, next_f: F, win: mpi_sys::MPI_Win, win_keyval: c_int) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Win, c_int) -> c_int,
    {
        next_f(win, win_keyval)
    }
    #[inline]
    fn win_detach<F>(&self, next_f: F, win: mpi_sys::MPI_Win, base: *const c_void) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Win, *const c_void) -> c_int,
    {
        next_f(win, base)
    }
    #[inline]
    fn win_fence<F>(&self, next_f: F, assert: c_int, win: mpi_sys::MPI_Win) -> c_int
    where
        F: FnOnce(c_int, mpi_sys::MPI_Win) -> c_int,
    {
        next_f(assert, win)
    }
    #[inline]
    fn win_flush<F>(&self, next_f: F, rank: c_int, win: mpi_sys::MPI_Win) -> c_int
    where
        F: FnOnce(c_int, mpi_sys::MPI_Win) -> c_int,
    {
        next_f(rank, win)
    }
    #[inline]
    fn win_flush_all<F>(&self, next_f: F, win: mpi_sys::MPI_Win) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Win) -> c_int,
    {
        next_f(win)
    }
    #[inline]
    fn win_flush_local<F>(&self, next_f: F, rank: c_int, win: mpi_sys::MPI_Win) -> c_int
    where
        F: FnOnce(c_int, mpi_sys::MPI_Win) -> c_int,
    {
        next_f(rank, win)
    }
    #[inline]
    fn win_flush_local_all<F>(&self, next_f: F, win: mpi_sys::MPI_Win) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Win) -> c_int,
    {
        next_f(win)
    }
    #[inline]
    fn win_free<F>(&self, next_f: F, win: *mut mpi_sys::MPI_Win) -> c_int
    where
        F: FnOnce(*mut mpi_sys::MPI_Win) -> c_int,
    {
        next_f(win)
    }
    #[inline]
    fn win_free_keyval<F>(&self, next_f: F, win_keyval: *mut c_int) -> c_int
    where
        F: FnOnce(*mut c_int) -> c_int,
    {
        next_f(win_keyval)
    }
    #[inline]
    fn win_get_attr<F>(
        &self,
        next_f: F,
        win: mpi_sys::MPI_Win,
        win_keyval: c_int,
        attribute_val: *mut c_void,
        flag: *mut c_int,
    ) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Win, c_int, *mut c_void, *mut c_int) -> c_int,
    {
        next_f(win, win_keyval, attribute_val, flag)
    }
    #[inline]
    fn win_get_errhandler<F>(
        &self,
        next_f: F,
        win: mpi_sys::MPI_Win,
        errhandler: *mut mpi_sys::MPI_Errhandler,
    ) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Win, *mut mpi_sys::MPI_Errhandler) -> c_int,
    {
        next_f(win, errhandler)
    }
    #[inline]
    fn win_get_group<F>(
        &self,
        next_f: F,
        win: mpi_sys::MPI_Win,
        group: *mut mpi_sys::MPI_Group,
    ) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Win, *mut mpi_sys::MPI_Group) -> c_int,
    {
        next_f(win, group)
    }
    #[inline]
    fn win_get_info<F>(
        &self,
        next_f: F,
        win: mpi_sys::MPI_Win,
        info_used: *mut mpi_sys::MPI_Info,
    ) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Win, *mut mpi_sys::MPI_Info) -> c_int,
    {
        next_f(win, info_used)
    }
    #[inline]
    fn win_get_name<F>(
        &self,
        next_f: F,
        win: mpi_sys::MPI_Win,
        win_name: *mut c_char,
        resultlen: *mut c_int,
    ) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Win, *mut c_char, *mut c_int) -> c_int,
    {
        next_f(win, win_name, resultlen)
    }
    #[inline]
    fn win_lock<F>(
        &self,
        next_f: F,
        lock_type: c_int,
        rank: c_int,
        assert: c_int,
        win: mpi_sys::MPI_Win,
    ) -> c_int
    where
        F: FnOnce(c_int, c_int, c_int, mpi_sys::MPI_Win) -> c_int,
    {
        next_f(lock_type, rank, assert, win)
    }
    #[inline]
    fn win_lock_all<F>(&self, next_f: F, assert: c_int, win: mpi_sys::MPI_Win) -> c_int
    where
        F: FnOnce(c_int, mpi_sys::MPI_Win) -> c_int,
    {
        next_f(assert, win)
    }
    #[inline]
    fn win_post<F>(
        &self,
        next_f: F,
        group: mpi_sys::MPI_Group,
        assert: c_int,
        win: mpi_sys::MPI_Win,
    ) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Group, c_int, mpi_sys::MPI_Win) -> c_int,
    {
        next_f(group, assert, win)
    }
    #[inline]
    fn win_set_attr<F>(
        &self,
        next_f: F,
        win: mpi_sys::MPI_Win,
        win_keyval: c_int,
        attribute_val: *mut c_void,
    ) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Win, c_int, *mut c_void) -> c_int,
    {
        next_f(win, win_keyval, attribute_val)
    }
    #[inline]
    fn win_set_errhandler<F>(
        &self,
        next_f: F,
        win: mpi_sys::MPI_Win,
        errhandler: mpi_sys::MPI_Errhandler,
    ) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Win, mpi_sys::MPI_Errhandler) -> c_int,
    {
        next_f(win, errhandler)
    }
    #[inline]
    fn win_set_info<F>(&self, next_f: F, win: mpi_sys::MPI_Win, info: mpi_sys::MPI_Info) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Win, mpi_sys::MPI_Info) -> c_int,
    {
        next_f(win, info)
    }
    #[inline]
    fn win_set_name<F>(&self, next_f: F, win: mpi_sys::MPI_Win, win_name: *const c_char) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Win, *const c_char) -> c_int,
    {
        next_f(win, win_name)
    }
    #[inline]
    fn win_shared_query<F>(
        &self,
        next_f: F,
        win: mpi_sys::MPI_Win,
        rank: c_int,
        size: *mut mpi_sys::MPI_Aint,
        disp_unit: *mut c_int,
        baseptr: *mut c_void,
    ) -> c_int
    where
        F: FnOnce(
            mpi_sys::MPI_Win,
            c_int,
            *mut mpi_sys::MPI_Aint,
            *mut c_int,
            *mut c_void,
        ) -> c_int,
    {
        next_f(win, rank, size, disp_unit, baseptr)
    }
    #[inline]
    fn win_start<F>(
        &self,
        next_f: F,
        group: mpi_sys::MPI_Group,
        assert: c_int,
        win: mpi_sys::MPI_Win,
    ) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Group, c_int, mpi_sys::MPI_Win) -> c_int,
    {
        next_f(group, assert, win)
    }
    #[inline]
    fn win_sync<F>(&self, next_f: F, win: mpi_sys::MPI_Win) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Win) -> c_int,
    {
        next_f(win)
    }
    #[inline]
    fn win_test<F>(&self, next_f: F, win: mpi_sys::MPI_Win, flag: *mut c_int) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Win, *mut c_int) -> c_int,
    {
        next_f(win, flag)
    }
    #[inline]
    fn win_unlock<F>(&self, next_f: F, rank: c_int, win: mpi_sys::MPI_Win) -> c_int
    where
        F: FnOnce(c_int, mpi_sys::MPI_Win) -> c_int,
    {
        next_f(rank, win)
    }
    #[inline]
    fn win_unlock_all<F>(&self, next_f: F, win: mpi_sys::MPI_Win) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Win) -> c_int,
    {
        next_f(win)
    }
    #[inline]
    fn win_wait<F>(&self, next_f: F, win: mpi_sys::MPI_Win) -> c_int
    where
        F: FnOnce(mpi_sys::MPI_Win) -> c_int,
    {
        next_f(win)
    }
    #[inline]
    fn wtick<F>(&self, next_f: F) -> c_double
    where
        F: FnOnce() -> c_double,
    {
        next_f()
    }
    #[inline]
    fn wtime<F>(&self, next_f: F) -> c_double
    where
        F: FnOnce() -> c_double,
    {
        next_f()
    }
}
