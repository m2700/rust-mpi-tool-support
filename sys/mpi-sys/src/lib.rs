mod c_mpi {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    include!(env!("MPI_BINDGEN_BINDINGS"));
}

mod types_and_consts {
    pub use crate::c_mpi::{
        DoubleInt, LongDoubleInt, LongInt, MPI_Aint, MPI_Comm, MPI_Comm_copy_attr_function,
        MPI_Comm_delete_attr_function, MPI_Comm_errhandler_function, MPI_Copy_function, MPI_Count,
        MPI_Datarep_conversion_function, MPI_Datarep_extent_function, MPI_Datatype,
        MPI_Delete_function, MPI_Errhandler, MPI_File, MPI_File_errhandler_function,
        MPI_Grequest_cancel_function, MPI_Grequest_free_function, MPI_Grequest_query_function,
        MPI_Group, MPI_Handler_function, MPI_Info, MPI_Message, MPI_Offset, MPI_Op, MPI_Request,
        MPI_Status, MPI_Type_copy_attr_function, MPI_Type_delete_attr_function, MPI_User_function,
        MPI_Win, MPI_Win_copy_attr_function, MPI_Win_delete_attr_function,
        MPI_Win_errhandler_function, ShortInt, TwoInt, MPI_UNDEFINED, RUST_C_BOOL as MPI_C_BOOL,
        RUST_INT16_T as MPI_INT16_T, RUST_INT32_T as MPI_INT32_T, RUST_INT64_T as MPI_INT64_T,
        RUST_INT8_T as MPI_INT8_T, RUST_MPI_2INT as MPI_2INT, RUST_MPI_ANY_TAG as MPI_ANY_TAG,
        RUST_MPI_BAND as MPI_BAND, RUST_MPI_BOR as MPI_BOR, RUST_MPI_BXOR as MPI_BXOR,
        RUST_MPI_BYTE as MPI_BYTE, RUST_MPI_CHAR as MPI_CHAR, RUST_MPI_COMM_NULL as MPI_COMM_NULL,
        RUST_MPI_COMM_SELF as MPI_COMM_SELF, RUST_MPI_COMM_WORLD as MPI_COMM_WORLD,
        RUST_MPI_C_COMPLEX as MPI_C_COMPLEX, RUST_MPI_C_DOUBLE_COMPLEX as MPI_C_DOUBLE_COMPLEX,
        RUST_MPI_C_FLOAT_COMPLEX as MPI_C_FLOAT_COMPLEX,
        RUST_MPI_C_LONG_DOUBLE_COMPLEX as MPI_C_LONG_DOUBLE_COMPLEX,
        RUST_MPI_DATATYPE_NULL as MPI_DATATYPE_NULL, RUST_MPI_DOUBLE as MPI_DOUBLE,
        RUST_MPI_DOUBLE_INT as MPI_DOUBLE_INT, RUST_MPI_FLOAT as MPI_FLOAT,
        RUST_MPI_INT as MPI_INT, RUST_MPI_LAND as MPI_LAND, RUST_MPI_LONG as MPI_LONG,
        RUST_MPI_LONG_DOUBLE as MPI_LONG_DOUBLE, RUST_MPI_LONG_DOUBLE_INT as MPI_LONG_DOUBLE_INT,
        RUST_MPI_LONG_INT as MPI_LONG_INT, RUST_MPI_LONG_LONG as MPI_LONG_LONG,
        RUST_MPI_LONG_LONG_INT as MPI_LONG_LONG_INT, RUST_MPI_LOR as MPI_LOR,
        RUST_MPI_LXOR as MPI_LXOR, RUST_MPI_MAX as MPI_MAX, RUST_MPI_MAXLOC as MPI_MAXLOC,
        RUST_MPI_MIN as MPI_MIN, RUST_MPI_MINLOC as MPI_MINLOC, RUST_MPI_OP_NULL as MPI_OP_NULL,
        RUST_MPI_PROD as MPI_PROD, RUST_MPI_REPLACE as MPI_REPLACE,
        RUST_MPI_REQUEST_NULL as MPI_REQUEST_NULL, RUST_MPI_SHORT as MPI_SHORT,
        RUST_MPI_SHORT_INT as MPI_SHORT_INT, RUST_MPI_SIGNED_CHAR as MPI_SIGNED_CHAR,
        RUST_MPI_SUCCESS as MPI_SUCCESS, RUST_MPI_SUM as MPI_SUM,
        RUST_MPI_UNSIGNED as MPI_UNSIGNED, RUST_MPI_UNSIGNED_CHAR as MPI_UNSIGNED_CHAR,
        RUST_MPI_UNSIGNED_LONG as MPI_UNSIGNED_LONG,
        RUST_MPI_UNSIGNED_LONG_LONG as MPI_UNSIGNED_LONG_LONG,
        RUST_MPI_UNSIGNED_SHORT as MPI_UNSIGNED_SHORT, RUST_MPI_WCHAR as MPI_WCHAR,
        RUST_UINT16_T as MPI_UINT16_T, RUST_UINT32_T as MPI_UINT32_T,
        RUST_UINT64_T as MPI_UINT64_T, RUST_UINT8_T as MPI_UINT8_T,
    };
}
pub use types_and_consts::*;

#[cfg(feature = "mpi_functions")]
pub use c_mpi::{
    MPI_Abort, MPI_Accumulate, MPI_Add_error_class, MPI_Add_error_code, MPI_Add_error_string,
    MPI_Address, MPI_Allgather, MPI_Allgatherv, MPI_Alloc_mem, MPI_Allreduce, MPI_Alltoall,
    MPI_Alltoallv, MPI_Alltoallw, MPI_Attr_delete, MPI_Attr_get, MPI_Attr_put, MPI_Barrier,
    MPI_Bcast, MPI_Bsend, MPI_Bsend_init, MPI_Buffer_attach, MPI_Buffer_detach, MPI_Cancel,
    MPI_Cart_coords, MPI_Cart_create, MPI_Cart_get, MPI_Cart_map, MPI_Cart_rank, MPI_Cart_shift,
    MPI_Cart_sub, MPI_Cartdim_get, MPI_Close_port, MPI_Comm_accept, MPI_Comm_call_errhandler,
    MPI_Comm_compare, MPI_Comm_connect, MPI_Comm_create, MPI_Comm_create_errhandler,
    MPI_Comm_create_group, MPI_Comm_create_keyval, MPI_Comm_delete_attr, MPI_Comm_disconnect,
    MPI_Comm_dup, MPI_Comm_dup_with_info, MPI_Comm_free, MPI_Comm_free_keyval, MPI_Comm_get_attr,
    MPI_Comm_get_errhandler, MPI_Comm_get_info, MPI_Comm_get_name, MPI_Comm_get_parent,
    MPI_Comm_group, MPI_Comm_idup, MPI_Comm_join, MPI_Comm_rank, MPI_Comm_remote_group,
    MPI_Comm_remote_size, MPI_Comm_set_attr, MPI_Comm_set_errhandler, MPI_Comm_set_info,
    MPI_Comm_set_name, MPI_Comm_size, MPI_Comm_split, MPI_Comm_split_type, MPI_Comm_test_inter,
    MPI_Compare_and_swap, MPI_Dims_create, MPI_Dist_graph_create, MPI_Dist_graph_create_adjacent,
    MPI_Dist_graph_neighbors, MPI_Dist_graph_neighbors_count, MPI_Errhandler_create,
    MPI_Errhandler_free, MPI_Errhandler_get, MPI_Errhandler_set, MPI_Error_class, MPI_Error_string,
    MPI_Exscan, MPI_Fetch_and_op, MPI_File_call_errhandler, MPI_File_close,
    MPI_File_create_errhandler, MPI_File_delete, MPI_File_get_amode, MPI_File_get_atomicity,
    MPI_File_get_byte_offset, MPI_File_get_errhandler, MPI_File_get_group, MPI_File_get_info,
    MPI_File_get_position, MPI_File_get_position_shared, MPI_File_get_size,
    MPI_File_get_type_extent, MPI_File_get_view, MPI_File_iread, MPI_File_iread_all,
    MPI_File_iread_at, MPI_File_iread_at_all, MPI_File_iread_shared, MPI_File_iwrite,
    MPI_File_iwrite_all, MPI_File_iwrite_at, MPI_File_iwrite_at_all, MPI_File_iwrite_shared,
    MPI_File_open, MPI_File_preallocate, MPI_File_read, MPI_File_read_all, MPI_File_read_all_begin,
    MPI_File_read_all_end, MPI_File_read_at, MPI_File_read_at_all, MPI_File_read_at_all_begin,
    MPI_File_read_at_all_end, MPI_File_read_ordered, MPI_File_read_ordered_begin,
    MPI_File_read_ordered_end, MPI_File_read_shared, MPI_File_seek, MPI_File_seek_shared,
    MPI_File_set_atomicity, MPI_File_set_errhandler, MPI_File_set_info, MPI_File_set_size,
    MPI_File_set_view, MPI_File_sync, MPI_File_write, MPI_File_write_all, MPI_File_write_all_begin,
    MPI_File_write_all_end, MPI_File_write_at, MPI_File_write_at_all, MPI_File_write_at_all_begin,
    MPI_File_write_at_all_end, MPI_File_write_ordered, MPI_File_write_ordered_begin,
    MPI_File_write_ordered_end, MPI_File_write_shared, MPI_Finalize, MPI_Finalized, MPI_Free_mem,
    MPI_Gather, MPI_Gatherv, MPI_Get, MPI_Get_accumulate, MPI_Get_address, MPI_Get_count,
    MPI_Get_elements, MPI_Get_elements_x, MPI_Get_library_version, MPI_Get_processor_name,
    MPI_Get_version, MPI_Graph_create, MPI_Graph_get, MPI_Graph_map, MPI_Graph_neighbors,
    MPI_Graph_neighbors_count, MPI_Graphdims_get, MPI_Grequest_complete, MPI_Grequest_start,
    MPI_Group_compare, MPI_Group_difference, MPI_Group_excl, MPI_Group_free, MPI_Group_incl,
    MPI_Group_intersection, MPI_Group_range_excl, MPI_Group_range_incl, MPI_Group_rank,
    MPI_Group_size, MPI_Group_translate_ranks, MPI_Group_union, MPI_Iallgather, MPI_Iallgatherv,
    MPI_Iallreduce, MPI_Ialltoall, MPI_Ialltoallv, MPI_Ialltoallw, MPI_Ibarrier, MPI_Ibcast,
    MPI_Ibsend, MPI_Iexscan, MPI_Igather, MPI_Igatherv, MPI_Improbe, MPI_Imrecv,
    MPI_Ineighbor_allgather, MPI_Ineighbor_allgatherv, MPI_Ineighbor_alltoall,
    MPI_Ineighbor_alltoallv, MPI_Ineighbor_alltoallw, MPI_Info_create, MPI_Info_delete,
    MPI_Info_dup, MPI_Info_free, MPI_Info_get, MPI_Info_get_nkeys, MPI_Info_get_nthkey,
    MPI_Info_get_valuelen, MPI_Info_set, MPI_Init, MPI_Init_thread, MPI_Initialized,
    MPI_Intercomm_create, MPI_Intercomm_merge, MPI_Iprobe, MPI_Irecv, MPI_Ireduce,
    MPI_Ireduce_scatter, MPI_Ireduce_scatter_block, MPI_Irsend, MPI_Is_thread_main, MPI_Iscan,
    MPI_Iscatter, MPI_Iscatterv, MPI_Isend, MPI_Issend, MPI_Keyval_create, MPI_Keyval_free,
    MPI_Lookup_name, MPI_Mprobe, MPI_Mrecv, MPI_Neighbor_allgather, MPI_Neighbor_allgatherv,
    MPI_Neighbor_alltoall, MPI_Neighbor_alltoallv, MPI_Neighbor_alltoallw, MPI_Op_commutative,
    MPI_Op_create, MPI_Op_free, MPI_Open_port, MPI_Pack, MPI_Pack_external, MPI_Pack_external_size,
    MPI_Pack_size, MPI_Pcontrol, MPI_Probe, MPI_Publish_name, MPI_Put, MPI_Query_thread,
    MPI_Raccumulate, MPI_Recv, MPI_Recv_init, MPI_Reduce, MPI_Reduce_local, MPI_Reduce_scatter,
    MPI_Reduce_scatter_block, MPI_Register_datarep, MPI_Request_free, MPI_Request_get_status,
    MPI_Rget, MPI_Rget_accumulate, MPI_Rput, MPI_Rsend, MPI_Rsend_init, MPI_Scan, MPI_Scatter,
    MPI_Scatterv, MPI_Send, MPI_Send_init, MPI_Sendrecv, MPI_Sendrecv_replace, MPI_Ssend,
    MPI_Ssend_init, MPI_Start, MPI_Startall, MPI_Status_set_cancelled, MPI_Status_set_elements,
    MPI_Status_set_elements_x, MPI_Test, MPI_Test_cancelled, MPI_Testall, MPI_Testany,
    MPI_Testsome, MPI_Topo_test, MPI_Type_commit, MPI_Type_contiguous, MPI_Type_create_darray,
    MPI_Type_create_f90_complex, MPI_Type_create_f90_integer, MPI_Type_create_f90_real,
    MPI_Type_create_hindexed, MPI_Type_create_hindexed_block, MPI_Type_create_hvector,
    MPI_Type_create_indexed_block, MPI_Type_create_keyval, MPI_Type_create_resized,
    MPI_Type_create_struct, MPI_Type_create_subarray, MPI_Type_delete_attr, MPI_Type_dup,
    MPI_Type_extent, MPI_Type_free, MPI_Type_free_keyval, MPI_Type_get_attr, MPI_Type_get_contents,
    MPI_Type_get_envelope, MPI_Type_get_extent, MPI_Type_get_extent_x, MPI_Type_get_name,
    MPI_Type_get_true_extent, MPI_Type_get_true_extent_x, MPI_Type_hindexed, MPI_Type_hvector,
    MPI_Type_indexed, MPI_Type_lb, MPI_Type_match_size, MPI_Type_set_attr, MPI_Type_set_name,
    MPI_Type_size, MPI_Type_size_x, MPI_Type_struct, MPI_Type_ub, MPI_Type_vector, MPI_Unpack,
    MPI_Unpack_external, MPI_Unpublish_name, MPI_Wait, MPI_Waitall, MPI_Waitany, MPI_Waitsome,
    MPI_Win_allocate, MPI_Win_allocate_shared, MPI_Win_attach, MPI_Win_call_errhandler,
    MPI_Win_complete, MPI_Win_create, MPI_Win_create_dynamic, MPI_Win_create_errhandler,
    MPI_Win_create_keyval, MPI_Win_delete_attr, MPI_Win_detach, MPI_Win_fence, MPI_Win_flush,
    MPI_Win_flush_all, MPI_Win_flush_local, MPI_Win_flush_local_all, MPI_Win_free,
    MPI_Win_free_keyval, MPI_Win_get_attr, MPI_Win_get_errhandler, MPI_Win_get_group,
    MPI_Win_get_info, MPI_Win_get_name, MPI_Win_lock, MPI_Win_lock_all, MPI_Win_post,
    MPI_Win_set_attr, MPI_Win_set_errhandler, MPI_Win_set_info, MPI_Win_set_name,
    MPI_Win_shared_query, MPI_Win_start, MPI_Win_sync, MPI_Win_test, MPI_Win_unlock,
    MPI_Win_unlock_all, MPI_Win_wait, MPI_Wtick, MPI_Wtime,
};

#[cfg(feature = "tool_mode")]
pub mod pmpi {
    #[cfg(feature = "mpi_functions")]
    pub use crate::c_mpi::{
        PMPI_Abort as MPI_Abort, PMPI_Accumulate as MPI_Accumulate,
        PMPI_Add_error_class as MPI_Add_error_class, PMPI_Add_error_code as MPI_Add_error_code,
        PMPI_Add_error_string as MPI_Add_error_string, PMPI_Address as MPI_Address,
        PMPI_Allgather as MPI_Allgather, PMPI_Allgatherv as MPI_Allgatherv,
        PMPI_Alloc_mem as MPI_Alloc_mem, PMPI_Allreduce as MPI_Allreduce,
        PMPI_Alltoall as MPI_Alltoall, PMPI_Alltoallv as MPI_Alltoallv,
        PMPI_Alltoallw as MPI_Alltoallw, PMPI_Attr_delete as MPI_Attr_delete,
        PMPI_Attr_get as MPI_Attr_get, PMPI_Attr_put as MPI_Attr_put, PMPI_Barrier as MPI_Barrier,
        PMPI_Bcast as MPI_Bcast, PMPI_Bsend as MPI_Bsend, PMPI_Bsend_init as MPI_Bsend_init,
        PMPI_Buffer_attach as MPI_Buffer_attach, PMPI_Buffer_detach as MPI_Buffer_detach,
        PMPI_Cancel as MPI_Cancel, PMPI_Cart_coords as MPI_Cart_coords,
        PMPI_Cart_create as MPI_Cart_create, PMPI_Cart_get as MPI_Cart_get,
        PMPI_Cart_map as MPI_Cart_map, PMPI_Cart_rank as MPI_Cart_rank,
        PMPI_Cart_shift as MPI_Cart_shift, PMPI_Cart_sub as MPI_Cart_sub,
        PMPI_Cartdim_get as MPI_Cartdim_get, PMPI_Close_port as MPI_Close_port,
        PMPI_Comm_accept as MPI_Comm_accept, PMPI_Comm_call_errhandler as MPI_Comm_call_errhandler,
        PMPI_Comm_compare as MPI_Comm_compare, PMPI_Comm_connect as MPI_Comm_connect,
        PMPI_Comm_create as MPI_Comm_create,
        PMPI_Comm_create_errhandler as MPI_Comm_create_errhandler,
        PMPI_Comm_create_group as MPI_Comm_create_group,
        PMPI_Comm_create_keyval as MPI_Comm_create_keyval,
        PMPI_Comm_delete_attr as MPI_Comm_delete_attr, PMPI_Comm_disconnect as MPI_Comm_disconnect,
        PMPI_Comm_dup as MPI_Comm_dup, PMPI_Comm_dup_with_info as MPI_Comm_dup_with_info,
        PMPI_Comm_free as MPI_Comm_free, PMPI_Comm_free_keyval as MPI_Comm_free_keyval,
        PMPI_Comm_get_attr as MPI_Comm_get_attr,
        PMPI_Comm_get_errhandler as MPI_Comm_get_errhandler,
        PMPI_Comm_get_info as MPI_Comm_get_info, PMPI_Comm_get_name as MPI_Comm_get_name,
        PMPI_Comm_get_parent as MPI_Comm_get_parent, PMPI_Comm_group as MPI_Comm_group,
        PMPI_Comm_idup as MPI_Comm_idup, PMPI_Comm_join as MPI_Comm_join,
        PMPI_Comm_rank as MPI_Comm_rank, PMPI_Comm_remote_group as MPI_Comm_remote_group,
        PMPI_Comm_remote_size as MPI_Comm_remote_size, PMPI_Comm_set_attr as MPI_Comm_set_attr,
        PMPI_Comm_set_errhandler as MPI_Comm_set_errhandler,
        PMPI_Comm_set_info as MPI_Comm_set_info, PMPI_Comm_set_name as MPI_Comm_set_name,
        PMPI_Comm_size as MPI_Comm_size, PMPI_Comm_split as MPI_Comm_split,
        PMPI_Comm_split_type as MPI_Comm_split_type, PMPI_Comm_test_inter as MPI_Comm_test_inter,
        PMPI_Compare_and_swap as MPI_Compare_and_swap, PMPI_Dims_create as MPI_Dims_create,
        PMPI_Dist_graph_create as MPI_Dist_graph_create,
        PMPI_Dist_graph_create_adjacent as MPI_Dist_graph_create_adjacent,
        PMPI_Dist_graph_neighbors as MPI_Dist_graph_neighbors,
        PMPI_Dist_graph_neighbors_count as MPI_Dist_graph_neighbors_count,
        PMPI_Errhandler_create as MPI_Errhandler_create,
        PMPI_Errhandler_free as MPI_Errhandler_free, PMPI_Errhandler_get as MPI_Errhandler_get,
        PMPI_Errhandler_set as MPI_Errhandler_set, PMPI_Error_class as MPI_Error_class,
        PMPI_Error_string as MPI_Error_string, PMPI_Exscan as MPI_Exscan,
        PMPI_Fetch_and_op as MPI_Fetch_and_op,
        PMPI_File_call_errhandler as MPI_File_call_errhandler, PMPI_File_close as MPI_File_close,
        PMPI_File_create_errhandler as MPI_File_create_errhandler,
        PMPI_File_delete as MPI_File_delete, PMPI_File_get_amode as MPI_File_get_amode,
        PMPI_File_get_atomicity as MPI_File_get_atomicity,
        PMPI_File_get_byte_offset as MPI_File_get_byte_offset,
        PMPI_File_get_errhandler as MPI_File_get_errhandler,
        PMPI_File_get_group as MPI_File_get_group, PMPI_File_get_info as MPI_File_get_info,
        PMPI_File_get_position as MPI_File_get_position,
        PMPI_File_get_position_shared as MPI_File_get_position_shared,
        PMPI_File_get_size as MPI_File_get_size,
        PMPI_File_get_type_extent as MPI_File_get_type_extent,
        PMPI_File_get_view as MPI_File_get_view, PMPI_File_iread as MPI_File_iread,
        PMPI_File_iread_all as MPI_File_iread_all, PMPI_File_iread_at as MPI_File_iread_at,
        PMPI_File_iread_at_all as MPI_File_iread_at_all,
        PMPI_File_iread_shared as MPI_File_iread_shared, PMPI_File_iwrite as MPI_File_iwrite,
        PMPI_File_iwrite_all as MPI_File_iwrite_all, PMPI_File_iwrite_at as MPI_File_iwrite_at,
        PMPI_File_iwrite_at_all as MPI_File_iwrite_at_all,
        PMPI_File_iwrite_shared as MPI_File_iwrite_shared, PMPI_File_open as MPI_File_open,
        PMPI_File_preallocate as MPI_File_preallocate, PMPI_File_read as MPI_File_read,
        PMPI_File_read_all as MPI_File_read_all,
        PMPI_File_read_all_begin as MPI_File_read_all_begin,
        PMPI_File_read_all_end as MPI_File_read_all_end, PMPI_File_read_at as MPI_File_read_at,
        PMPI_File_read_at_all as MPI_File_read_at_all,
        PMPI_File_read_at_all_begin as MPI_File_read_at_all_begin,
        PMPI_File_read_at_all_end as MPI_File_read_at_all_end,
        PMPI_File_read_ordered as MPI_File_read_ordered,
        PMPI_File_read_ordered_begin as MPI_File_read_ordered_begin,
        PMPI_File_read_ordered_end as MPI_File_read_ordered_end,
        PMPI_File_read_shared as MPI_File_read_shared, PMPI_File_seek as MPI_File_seek,
        PMPI_File_seek_shared as MPI_File_seek_shared,
        PMPI_File_set_atomicity as MPI_File_set_atomicity,
        PMPI_File_set_errhandler as MPI_File_set_errhandler,
        PMPI_File_set_info as MPI_File_set_info, PMPI_File_set_size as MPI_File_set_size,
        PMPI_File_set_view as MPI_File_set_view, PMPI_File_sync as MPI_File_sync,
        PMPI_File_write as MPI_File_write, PMPI_File_write_all as MPI_File_write_all,
        PMPI_File_write_all_begin as MPI_File_write_all_begin,
        PMPI_File_write_all_end as MPI_File_write_all_end, PMPI_File_write_at as MPI_File_write_at,
        PMPI_File_write_at_all as MPI_File_write_at_all,
        PMPI_File_write_at_all_begin as MPI_File_write_at_all_begin,
        PMPI_File_write_at_all_end as MPI_File_write_at_all_end,
        PMPI_File_write_ordered as MPI_File_write_ordered,
        PMPI_File_write_ordered_begin as MPI_File_write_ordered_begin,
        PMPI_File_write_ordered_end as MPI_File_write_ordered_end,
        PMPI_File_write_shared as MPI_File_write_shared, PMPI_Finalize as MPI_Finalize,
        PMPI_Finalized as MPI_Finalized, PMPI_Free_mem as MPI_Free_mem, PMPI_Gather as MPI_Gather,
        PMPI_Gatherv as MPI_Gatherv, PMPI_Get as MPI_Get,
        PMPI_Get_accumulate as MPI_Get_accumulate, PMPI_Get_address as MPI_Get_address,
        PMPI_Get_count as MPI_Get_count, PMPI_Get_elements as MPI_Get_elements,
        PMPI_Get_elements_x as MPI_Get_elements_x,
        PMPI_Get_library_version as MPI_Get_library_version,
        PMPI_Get_processor_name as MPI_Get_processor_name, PMPI_Get_version as MPI_Get_version,
        PMPI_Graph_create as MPI_Graph_create, PMPI_Graph_get as MPI_Graph_get,
        PMPI_Graph_map as MPI_Graph_map, PMPI_Graph_neighbors as MPI_Graph_neighbors,
        PMPI_Graph_neighbors_count as MPI_Graph_neighbors_count,
        PMPI_Graphdims_get as MPI_Graphdims_get, PMPI_Grequest_complete as MPI_Grequest_complete,
        PMPI_Grequest_start as MPI_Grequest_start, PMPI_Group_compare as MPI_Group_compare,
        PMPI_Group_difference as MPI_Group_difference, PMPI_Group_excl as MPI_Group_excl,
        PMPI_Group_free as MPI_Group_free, PMPI_Group_incl as MPI_Group_incl,
        PMPI_Group_intersection as MPI_Group_intersection,
        PMPI_Group_range_excl as MPI_Group_range_excl,
        PMPI_Group_range_incl as MPI_Group_range_incl, PMPI_Group_rank as MPI_Group_rank,
        PMPI_Group_size as MPI_Group_size, PMPI_Group_translate_ranks as MPI_Group_translate_ranks,
        PMPI_Group_union as MPI_Group_union, PMPI_Iallgather as MPI_Iallgather,
        PMPI_Iallgatherv as MPI_Iallgatherv, PMPI_Iallreduce as MPI_Iallreduce,
        PMPI_Ialltoall as MPI_Ialltoall, PMPI_Ialltoallv as MPI_Ialltoallv,
        PMPI_Ialltoallw as MPI_Ialltoallw, PMPI_Ibarrier as MPI_Ibarrier,
        PMPI_Ibcast as MPI_Ibcast, PMPI_Ibsend as MPI_Ibsend, PMPI_Iexscan as MPI_Iexscan,
        PMPI_Igather as MPI_Igather, PMPI_Igatherv as MPI_Igatherv, PMPI_Improbe as MPI_Improbe,
        PMPI_Imrecv as MPI_Imrecv, PMPI_Ineighbor_allgather as MPI_Ineighbor_allgather,
        PMPI_Ineighbor_allgatherv as MPI_Ineighbor_allgatherv,
        PMPI_Ineighbor_alltoall as MPI_Ineighbor_alltoall,
        PMPI_Ineighbor_alltoallv as MPI_Ineighbor_alltoallv,
        PMPI_Ineighbor_alltoallw as MPI_Ineighbor_alltoallw, PMPI_Info_create as MPI_Info_create,
        PMPI_Info_delete as MPI_Info_delete, PMPI_Info_dup as MPI_Info_dup,
        PMPI_Info_free as MPI_Info_free, PMPI_Info_get as MPI_Info_get,
        PMPI_Info_get_nkeys as MPI_Info_get_nkeys, PMPI_Info_get_nthkey as MPI_Info_get_nthkey,
        PMPI_Info_get_valuelen as MPI_Info_get_valuelen, PMPI_Info_set as MPI_Info_set,
        PMPI_Init as MPI_Init, PMPI_Init_thread as MPI_Init_thread,
        PMPI_Initialized as MPI_Initialized, PMPI_Intercomm_create as MPI_Intercomm_create,
        PMPI_Intercomm_merge as MPI_Intercomm_merge, PMPI_Iprobe as MPI_Iprobe,
        PMPI_Irecv as MPI_Irecv, PMPI_Ireduce as MPI_Ireduce,
        PMPI_Ireduce_scatter as MPI_Ireduce_scatter,
        PMPI_Ireduce_scatter_block as MPI_Ireduce_scatter_block, PMPI_Irsend as MPI_Irsend,
        PMPI_Is_thread_main as MPI_Is_thread_main, PMPI_Iscan as MPI_Iscan,
        PMPI_Iscatter as MPI_Iscatter, PMPI_Iscatterv as MPI_Iscatterv, PMPI_Isend as MPI_Isend,
        PMPI_Issend as MPI_Issend, PMPI_Keyval_create as MPI_Keyval_create,
        PMPI_Keyval_free as MPI_Keyval_free, PMPI_Lookup_name as MPI_Lookup_name,
        PMPI_Mprobe as MPI_Mprobe, PMPI_Mrecv as MPI_Mrecv,
        PMPI_Neighbor_allgather as MPI_Neighbor_allgather,
        PMPI_Neighbor_allgatherv as MPI_Neighbor_allgatherv,
        PMPI_Neighbor_alltoall as MPI_Neighbor_alltoall,
        PMPI_Neighbor_alltoallv as MPI_Neighbor_alltoallv,
        PMPI_Neighbor_alltoallw as MPI_Neighbor_alltoallw,
        PMPI_Op_commutative as MPI_Op_commutative, PMPI_Op_create as MPI_Op_create,
        PMPI_Op_free as MPI_Op_free, PMPI_Open_port as MPI_Open_port, PMPI_Pack as MPI_Pack,
        PMPI_Pack_external as MPI_Pack_external, PMPI_Pack_external_size as MPI_Pack_external_size,
        PMPI_Pack_size as MPI_Pack_size, PMPI_Pcontrol as MPI_Pcontrol, PMPI_Probe as MPI_Probe,
        PMPI_Publish_name as MPI_Publish_name, PMPI_Put as MPI_Put,
        PMPI_Query_thread as MPI_Query_thread, PMPI_Raccumulate as MPI_Raccumulate,
        PMPI_Recv as MPI_Recv, PMPI_Recv_init as MPI_Recv_init, PMPI_Reduce as MPI_Reduce,
        PMPI_Reduce_local as MPI_Reduce_local, PMPI_Reduce_scatter as MPI_Reduce_scatter,
        PMPI_Reduce_scatter_block as MPI_Reduce_scatter_block,
        PMPI_Register_datarep as MPI_Register_datarep, PMPI_Request_free as MPI_Request_free,
        PMPI_Request_get_status as MPI_Request_get_status, PMPI_Rget as MPI_Rget,
        PMPI_Rget_accumulate as MPI_Rget_accumulate, PMPI_Rput as MPI_Rput,
        PMPI_Rsend as MPI_Rsend, PMPI_Rsend_init as MPI_Rsend_init, PMPI_Scan as MPI_Scan,
        PMPI_Scatter as MPI_Scatter, PMPI_Scatterv as MPI_Scatterv, PMPI_Send as MPI_Send,
        PMPI_Send_init as MPI_Send_init, PMPI_Sendrecv as MPI_Sendrecv,
        PMPI_Sendrecv_replace as MPI_Sendrecv_replace, PMPI_Ssend as MPI_Ssend,
        PMPI_Ssend_init as MPI_Ssend_init, PMPI_Start as MPI_Start, PMPI_Startall as MPI_Startall,
        PMPI_Status_set_cancelled as MPI_Status_set_cancelled,
        PMPI_Status_set_elements as MPI_Status_set_elements,
        PMPI_Status_set_elements_x as MPI_Status_set_elements_x, PMPI_Test as MPI_Test,
        PMPI_Test_cancelled as MPI_Test_cancelled, PMPI_Testall as MPI_Testall,
        PMPI_Testany as MPI_Testany, PMPI_Testsome as MPI_Testsome,
        PMPI_Topo_test as MPI_Topo_test, PMPI_Type_commit as MPI_Type_commit,
        PMPI_Type_contiguous as MPI_Type_contiguous,
        PMPI_Type_create_darray as MPI_Type_create_darray,
        PMPI_Type_create_f90_complex as MPI_Type_create_f90_complex,
        PMPI_Type_create_f90_integer as MPI_Type_create_f90_integer,
        PMPI_Type_create_f90_real as MPI_Type_create_f90_real,
        PMPI_Type_create_hindexed as MPI_Type_create_hindexed,
        PMPI_Type_create_hindexed_block as MPI_Type_create_hindexed_block,
        PMPI_Type_create_hvector as MPI_Type_create_hvector,
        PMPI_Type_create_indexed_block as MPI_Type_create_indexed_block,
        PMPI_Type_create_keyval as MPI_Type_create_keyval,
        PMPI_Type_create_resized as MPI_Type_create_resized,
        PMPI_Type_create_struct as MPI_Type_create_struct,
        PMPI_Type_create_subarray as MPI_Type_create_subarray,
        PMPI_Type_delete_attr as MPI_Type_delete_attr, PMPI_Type_dup as MPI_Type_dup,
        PMPI_Type_extent as MPI_Type_extent, PMPI_Type_free as MPI_Type_free,
        PMPI_Type_free_keyval as MPI_Type_free_keyval, PMPI_Type_get_attr as MPI_Type_get_attr,
        PMPI_Type_get_contents as MPI_Type_get_contents,
        PMPI_Type_get_envelope as MPI_Type_get_envelope,
        PMPI_Type_get_extent as MPI_Type_get_extent,
        PMPI_Type_get_extent_x as MPI_Type_get_extent_x, PMPI_Type_get_name as MPI_Type_get_name,
        PMPI_Type_get_true_extent as MPI_Type_get_true_extent,
        PMPI_Type_get_true_extent_x as MPI_Type_get_true_extent_x,
        PMPI_Type_hindexed as MPI_Type_hindexed, PMPI_Type_hvector as MPI_Type_hvector,
        PMPI_Type_indexed as MPI_Type_indexed, PMPI_Type_lb as MPI_Type_lb,
        PMPI_Type_match_size as MPI_Type_match_size, PMPI_Type_set_attr as MPI_Type_set_attr,
        PMPI_Type_set_name as MPI_Type_set_name, PMPI_Type_size as MPI_Type_size,
        PMPI_Type_size_x as MPI_Type_size_x, PMPI_Type_struct as MPI_Type_struct,
        PMPI_Type_ub as MPI_Type_ub, PMPI_Type_vector as MPI_Type_vector,
        PMPI_Unpack as MPI_Unpack, PMPI_Unpack_external as MPI_Unpack_external,
        PMPI_Unpublish_name as MPI_Unpublish_name, PMPI_Wait as MPI_Wait,
        PMPI_Waitall as MPI_Waitall, PMPI_Waitany as MPI_Waitany, PMPI_Waitsome as MPI_Waitsome,
        PMPI_Win_allocate as MPI_Win_allocate, PMPI_Win_allocate_shared as MPI_Win_allocate_shared,
        PMPI_Win_attach as MPI_Win_attach, PMPI_Win_call_errhandler as MPI_Win_call_errhandler,
        PMPI_Win_complete as MPI_Win_complete, PMPI_Win_create as MPI_Win_create,
        PMPI_Win_create_dynamic as MPI_Win_create_dynamic,
        PMPI_Win_create_errhandler as MPI_Win_create_errhandler,
        PMPI_Win_create_keyval as MPI_Win_create_keyval,
        PMPI_Win_delete_attr as MPI_Win_delete_attr, PMPI_Win_detach as MPI_Win_detach,
        PMPI_Win_fence as MPI_Win_fence, PMPI_Win_flush as MPI_Win_flush,
        PMPI_Win_flush_all as MPI_Win_flush_all, PMPI_Win_flush_local as MPI_Win_flush_local,
        PMPI_Win_flush_local_all as MPI_Win_flush_local_all, PMPI_Win_free as MPI_Win_free,
        PMPI_Win_free_keyval as MPI_Win_free_keyval, PMPI_Win_get_attr as MPI_Win_get_attr,
        PMPI_Win_get_errhandler as MPI_Win_get_errhandler, PMPI_Win_get_group as MPI_Win_get_group,
        PMPI_Win_get_info as MPI_Win_get_info, PMPI_Win_get_name as MPI_Win_get_name,
        PMPI_Win_lock as MPI_Win_lock, PMPI_Win_lock_all as MPI_Win_lock_all,
        PMPI_Win_post as MPI_Win_post, PMPI_Win_set_attr as MPI_Win_set_attr,
        PMPI_Win_set_errhandler as MPI_Win_set_errhandler, PMPI_Win_set_info as MPI_Win_set_info,
        PMPI_Win_set_name as MPI_Win_set_name, PMPI_Win_shared_query as MPI_Win_shared_query,
        PMPI_Win_start as MPI_Win_start, PMPI_Win_sync as MPI_Win_sync,
        PMPI_Win_test as MPI_Win_test, PMPI_Win_unlock as MPI_Win_unlock,
        PMPI_Win_unlock_all as MPI_Win_unlock_all, PMPI_Win_wait as MPI_Win_wait,
        PMPI_Wtick as MPI_Wtick, PMPI_Wtime as MPI_Wtime,
    };
    pub use crate::types_and_consts::*;
}
