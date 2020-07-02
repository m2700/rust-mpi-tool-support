use std::{
    env,
    fs::metadata,
    path::{Path, PathBuf},
    process::Command,
    str::from_utf8,
};

macro_rules! mtime {
    ($fp:expr) => {
        metadata($fp).unwrap().modified().unwrap()
    };
}

const MPI_TYPES: [&str; 33] = [
    "MPI_Comm",
    "MPI_Datatype",
    "MPI_Aint",
    "MPI_Op",
    "MPI_Win",
    "MPI_Info",
    "MPI_Request",
    "MPI_Group",
    "MPI_Comm_errhandler_function",
    "MPI_Errhandler",
    "MPI_Comm_copy_attr_function",
    "MPI_Comm_delete_attr_function",
    "MPI_Handler_function",
    "MPI_File",
    "MPI_File_errhandler_function",
    "MPI_Offset",
    "MPI_Status",
    "MPI_Count",
    "MPI_Grequest_query_function",
    "MPI_Grequest_free_function",
    "MPI_Grequest_cancel_function",
    "MPI_Message",
    "MPI_Copy_function",
    "MPI_Delete_function",
    "MPI_User_function",
    "MPI_Datarep_conversion_function",
    "MPI_Datarep_extent_function",
    "MPI_Type_copy_attr_function",
    "MPI_Type_delete_attr_function",
    "MPI_Win_errhandler_function",
    "MPI_Win_copy_attr_function",
    "MPI_Win_delete_attr_function",
    "MPI_Comm",
];
#[cfg(feature = "mpi_functions")]
const MPI_FUNCTIONS: [&str; 360] = [
    "MPI_Abort",
    "MPI_Accumulate",
    "MPI_Add_error_class",
    "MPI_Add_error_code",
    "MPI_Add_error_string",
    "MPI_Address",
    "MPI_Allgather",
    "MPI_Allgatherv",
    "MPI_Alloc_mem",
    "MPI_Allreduce",
    "MPI_Alltoall",
    "MPI_Alltoallv",
    "MPI_Alltoallw",
    "MPI_Attr_delete",
    "MPI_Attr_get",
    "MPI_Attr_put",
    "MPI_Barrier",
    "MPI_Bcast",
    "MPI_Bsend",
    "MPI_Bsend_init",
    "MPI_Buffer_attach",
    "MPI_Buffer_detach",
    "MPI_Cancel",
    "MPI_Cart_coords",
    "MPI_Cart_create",
    "MPI_Cart_get",
    "MPI_Cart_map",
    "MPI_Cart_rank",
    "MPI_Cart_shift",
    "MPI_Cart_sub",
    "MPI_Cartdim_get",
    "MPI_Close_port",
    "MPI_Comm_accept",
    "MPI_Comm_call_errhandler",
    "MPI_Comm_compare",
    "MPI_Comm_connect",
    "MPI_Comm_create",
    "MPI_Comm_create_errhandler",
    "MPI_Comm_create_group",
    "MPI_Comm_create_keyval",
    "MPI_Comm_delete_attr",
    "MPI_Comm_disconnect",
    "MPI_Comm_dup",
    "MPI_Comm_dup_with_info",
    "MPI_Comm_free",
    "MPI_Comm_free_keyval",
    "MPI_Comm_get_attr",
    "MPI_Comm_get_errhandler",
    "MPI_Comm_get_info",
    "MPI_Comm_get_name",
    "MPI_Comm_get_parent",
    "MPI_Comm_group",
    "MPI_Comm_idup",
    "MPI_Comm_join",
    "MPI_Comm_rank",
    "MPI_Comm_remote_group",
    "MPI_Comm_remote_size",
    "MPI_Comm_set_attr",
    "MPI_Comm_set_errhandler",
    "MPI_Comm_set_info",
    "MPI_Comm_set_name",
    "MPI_Comm_size",
    "MPI_Comm_split",
    "MPI_Comm_split_type",
    "MPI_Comm_test_inter",
    "MPI_Compare_and_swap",
    "MPI_Dims_create",
    "MPI_Dist_graph_create",
    "MPI_Dist_graph_create_adjacent",
    "MPI_Dist_graph_neighbors",
    "MPI_Dist_graph_neighbors_count",
    "MPI_Errhandler_create",
    "MPI_Errhandler_free",
    "MPI_Errhandler_get",
    "MPI_Errhandler_set",
    "MPI_Error_class",
    "MPI_Error_string",
    "MPI_Exscan",
    "MPI_Fetch_and_op",
    "MPI_File_call_errhandler",
    "MPI_File_close",
    "MPI_File_create_errhandler",
    "MPI_File_delete",
    "MPI_File_get_amode",
    "MPI_File_get_atomicity",
    "MPI_File_get_byte_offset",
    "MPI_File_get_errhandler",
    "MPI_File_get_group",
    "MPI_File_get_info",
    "MPI_File_get_position",
    "MPI_File_get_position_shared",
    "MPI_File_get_size",
    "MPI_File_get_type_extent",
    "MPI_File_get_view",
    "MPI_File_iread",
    "MPI_File_iread_all",
    "MPI_File_iread_at",
    "MPI_File_iread_at_all",
    "MPI_File_iread_shared",
    "MPI_File_iwrite",
    "MPI_File_iwrite_all",
    "MPI_File_iwrite_at",
    "MPI_File_iwrite_at_all",
    "MPI_File_iwrite_shared",
    "MPI_File_open",
    "MPI_File_preallocate",
    "MPI_File_read",
    "MPI_File_read_all",
    "MPI_File_read_all_begin",
    "MPI_File_read_all_end",
    "MPI_File_read_at",
    "MPI_File_read_at_all",
    "MPI_File_read_at_all_begin",
    "MPI_File_read_at_all_end",
    "MPI_File_read_ordered",
    "MPI_File_read_ordered_begin",
    "MPI_File_read_ordered_end",
    "MPI_File_read_shared",
    "MPI_File_seek",
    "MPI_File_seek_shared",
    "MPI_File_set_atomicity",
    "MPI_File_set_errhandler",
    "MPI_File_set_info",
    "MPI_File_set_size",
    "MPI_File_set_view",
    "MPI_File_sync",
    "MPI_File_write",
    "MPI_File_write_all",
    "MPI_File_write_all_begin",
    "MPI_File_write_all_end",
    "MPI_File_write_at",
    "MPI_File_write_at_all",
    "MPI_File_write_at_all_begin",
    "MPI_File_write_at_all_end",
    "MPI_File_write_ordered",
    "MPI_File_write_ordered_begin",
    "MPI_File_write_ordered_end",
    "MPI_File_write_shared",
    "MPI_Finalize",
    "MPI_Finalized",
    "MPI_Free_mem",
    "MPI_Gather",
    "MPI_Gatherv",
    "MPI_Get",
    "MPI_Get_accumulate",
    "MPI_Get_address",
    "MPI_Get_count",
    "MPI_Get_elements",
    "MPI_Get_elements_x",
    "MPI_Get_library_version",
    "MPI_Get_processor_name",
    "MPI_Get_version",
    "MPI_Graph_create",
    "MPI_Graph_get",
    "MPI_Graph_map",
    "MPI_Graph_neighbors",
    "MPI_Graph_neighbors_count",
    "MPI_Graphdims_get",
    "MPI_Grequest_complete",
    "MPI_Grequest_start",
    "MPI_Group_compare",
    "MPI_Group_difference",
    "MPI_Group_excl",
    "MPI_Group_free",
    "MPI_Group_incl",
    "MPI_Group_intersection",
    "MPI_Group_range_excl",
    "MPI_Group_range_incl",
    "MPI_Group_rank",
    "MPI_Group_size",
    "MPI_Group_translate_ranks",
    "MPI_Group_union",
    "MPI_Iallgather",
    "MPI_Iallgatherv",
    "MPI_Iallreduce",
    "MPI_Ialltoall",
    "MPI_Ialltoallv",
    "MPI_Ialltoallw",
    "MPI_Ibarrier",
    "MPI_Ibcast",
    "MPI_Ibsend",
    "MPI_Iexscan",
    "MPI_Igather",
    "MPI_Igatherv",
    "MPI_Improbe",
    "MPI_Imrecv",
    "MPI_Ineighbor_allgather",
    "MPI_Ineighbor_allgatherv",
    "MPI_Ineighbor_alltoall",
    "MPI_Ineighbor_alltoallv",
    "MPI_Ineighbor_alltoallw",
    "MPI_Info_create",
    "MPI_Info_delete",
    "MPI_Info_dup",
    "MPI_Info_free",
    "MPI_Info_get",
    "MPI_Info_get_nkeys",
    "MPI_Info_get_nthkey",
    "MPI_Info_get_valuelen",
    "MPI_Info_set",
    "MPI_Init",
    "MPI_Init_thread",
    "MPI_Initialized",
    "MPI_Intercomm_create",
    "MPI_Intercomm_merge",
    "MPI_Iprobe",
    "MPI_Irecv",
    "MPI_Ireduce",
    "MPI_Ireduce_scatter",
    "MPI_Ireduce_scatter_block",
    "MPI_Irsend",
    "MPI_Is_thread_main",
    "MPI_Iscan",
    "MPI_Iscatter",
    "MPI_Iscatterv",
    "MPI_Isend",
    "MPI_Issend",
    "MPI_Keyval_create",
    "MPI_Keyval_free",
    "MPI_Lookup_name",
    "MPI_Mprobe",
    "MPI_Mrecv",
    "MPI_Neighbor_allgather",
    "MPI_Neighbor_allgatherv",
    "MPI_Neighbor_alltoall",
    "MPI_Neighbor_alltoallv",
    "MPI_Neighbor_alltoallw",
    "MPI_Op_commutative",
    "MPI_Op_create",
    "MPI_Op_free",
    "MPI_Open_port",
    "MPI_Pack",
    "MPI_Pack_external",
    "MPI_Pack_external_size",
    "MPI_Pack_size",
    "MPI_Pcontrol",
    "MPI_Probe",
    "MPI_Publish_name",
    "MPI_Put",
    "MPI_Query_thread",
    "MPI_Raccumulate",
    "MPI_Recv",
    "MPI_Recv_init",
    "MPI_Reduce",
    "MPI_Reduce_local",
    "MPI_Reduce_scatter",
    "MPI_Reduce_scatter_block",
    "MPI_Register_datarep",
    "MPI_Request_free",
    "MPI_Request_get_status",
    "MPI_Rget",
    "MPI_Rget_accumulate",
    "MPI_Rput",
    "MPI_Rsend",
    "MPI_Rsend_init",
    "MPI_Scan",
    "MPI_Scatter",
    "MPI_Scatterv",
    "MPI_Send",
    "MPI_Send_init",
    "MPI_Sendrecv",
    "MPI_Sendrecv_replace",
    "MPI_Ssend",
    "MPI_Ssend_init",
    "MPI_Start",
    "MPI_Startall",
    "MPI_Status_set_cancelled",
    "MPI_Status_set_elements",
    "MPI_Status_set_elements_x",
    "MPI_Test",
    "MPI_Test_cancelled",
    "MPI_Testall",
    "MPI_Testany",
    "MPI_Testsome",
    "MPI_Topo_test",
    "MPI_Type_commit",
    "MPI_Type_contiguous",
    "MPI_Type_create_darray",
    "MPI_Type_create_f90_complex",
    "MPI_Type_create_f90_integer",
    "MPI_Type_create_f90_real",
    "MPI_Type_create_hindexed",
    "MPI_Type_create_hindexed_block",
    "MPI_Type_create_hvector",
    "MPI_Type_create_indexed_block",
    "MPI_Type_create_keyval",
    "MPI_Type_create_resized",
    "MPI_Type_create_struct",
    "MPI_Type_create_subarray",
    "MPI_Type_delete_attr",
    "MPI_Type_dup",
    "MPI_Type_extent",
    "MPI_Type_free",
    "MPI_Type_free_keyval",
    "MPI_Type_get_attr",
    "MPI_Type_get_contents",
    "MPI_Type_get_envelope",
    "MPI_Type_get_extent",
    "MPI_Type_get_extent_x",
    "MPI_Type_get_name",
    "MPI_Type_get_true_extent",
    "MPI_Type_get_true_extent_x",
    "MPI_Type_hindexed",
    "MPI_Type_hvector",
    "MPI_Type_indexed",
    "MPI_Type_lb",
    "MPI_Type_match_size",
    "MPI_Type_set_attr",
    "MPI_Type_set_name",
    "MPI_Type_size",
    "MPI_Type_size_x",
    "MPI_Type_struct",
    "MPI_Type_ub",
    "MPI_Type_vector",
    "MPI_Unpack",
    "MPI_Unpack_external",
    "MPI_Unpublish_name",
    "MPI_Wait",
    "MPI_Waitall",
    "MPI_Waitany",
    "MPI_Waitsome",
    "MPI_Win_allocate",
    "MPI_Win_allocate_shared",
    "MPI_Win_attach",
    "MPI_Win_call_errhandler",
    "MPI_Win_complete",
    "MPI_Win_create",
    "MPI_Win_create_dynamic",
    "MPI_Win_create_errhandler",
    "MPI_Win_create_keyval",
    "MPI_Win_delete_attr",
    "MPI_Win_detach",
    "MPI_Win_fence",
    "MPI_Win_flush",
    "MPI_Win_flush_all",
    "MPI_Win_flush_local",
    "MPI_Win_flush_local_all",
    "MPI_Win_free",
    "MPI_Win_free_keyval",
    "MPI_Win_get_attr",
    "MPI_Win_get_errhandler",
    "MPI_Win_get_group",
    "MPI_Win_get_info",
    "MPI_Win_get_name",
    "MPI_Win_lock",
    "MPI_Win_lock_all",
    "MPI_Win_post",
    "MPI_Win_set_attr",
    "MPI_Win_set_errhandler",
    "MPI_Win_set_info",
    "MPI_Win_set_name",
    "MPI_Win_shared_query",
    "MPI_Win_start",
    "MPI_Win_sync",
    "MPI_Win_test",
    "MPI_Win_unlock",
    "MPI_Win_unlock_all",
    "MPI_Win_wait",
    "MPI_Wtick",
    "MPI_Wtime",
];
const MPI_VARS: [&str; 30] = [
    "RUST_MPI_CHAR",
    "RUST_MPI_SIGNED_CHAR",
    "RUST_MPI_UNSIGNED_CHAR",
    "RUST_MPI_BYTE",
    "RUST_MPI_WCHAR",
    "RUST_MPI_SHORT",
    "RUST_MPI_UNSIGNED_SHORT",
    "RUST_MPI_INT",
    "RUST_MPI_UNSIGNED",
    "RUST_MPI_LONG",
    "RUST_MPI_UNSIGNED_LONG",
    "RUST_MPI_FLOAT",
    "RUST_MPI_DOUBLE",
    "RUST_MPI_LONG_DOUBLE",
    "RUST_MPI_LONG_LONG_INT",
    "RUST_MPI_UNSIGNED_LONG_LONG",
    "RUST_MPI_LONG_LONG",
    "RUST_MPI_COMM_NULL",
    "RUST_MPI_COMM_WORLD",
    "RUST_MPI_COMM_SELF",
    "RUST_C_BOOL",
    "RUST_INT8_T",
    "RUST_INT16_T",
    "RUST_INT32_T",
    "RUST_INT64_T",
    "RUST_UINT8_T",
    "RUST_UINT16_T",
    "RUST_UINT32_T",
    "RUST_UINT64_T",
    "RUST_MPI_SUCCESS",
];

fn main() {
    let out_dir: PathBuf = env::var("OUT_DIR").unwrap().into();
    let src_dir = Path::new("src");

    let mpi_include_h_path = src_dir.join("mpi_include.h");
    let mpi_bindings_path = out_dir.join("mpi_bindings.rs");

    println!(
        "cargo:rustc-env=MPI_BINDGEN_BINDINGS={}",
        mpi_bindings_path.display()
    );
    println!("cargo:rerun-if-changed={}", mpi_include_h_path.display());

    if !mpi_bindings_path.exists() || mtime!(&mpi_bindings_path) < mtime!("build.rs") {
        let mpicc_output = Command::new("mpicc")
            .arg("-show")
            .output()
            .expect("failed to execute process 'mpicc'");
        let mpicc_args = from_utf8(if mpicc_output.stdout.starts_with(b"clang") {
            &mpicc_output.stdout[b"clang ".len()..]
        } else if mpicc_output.stdout.starts_with(b"gcc") {
            &mpicc_output.stdout[b"gcc ".len()..]
        } else {
            panic!(from_utf8(Box::leak(mpicc_output.stdout.into_boxed_slice())))
        })
        .unwrap();

        #[cfg(feature = "link_mpi")]
        for mpicc_arg in mpicc_args.split(' ') {
            if mpicc_arg.starts_with("-L") {
                println!("cargo:rustc-link-search={}", &mpicc_arg[2..])
            } else if mpicc_arg.starts_with("-l") {
                println!("cargo:rustc-link-lib={}", &mpicc_arg[2..]);
            } else {
                //TODO
            }
        }

        let mut mpi_bindings = bindgen::builder()
            .header(mpi_include_h_path.to_string_lossy())
            .clang_args(mpicc_args.split(' '))
            .default_enum_style(bindgen::EnumVariation::Rust {
                non_exhaustive: true,
            });
        for &mpi_type in &MPI_TYPES[..] {
            mpi_bindings = mpi_bindings.whitelist_type(mpi_type);
        }
        #[cfg(feature = "mpi_functions")]
        for &mpi_function in &MPI_FUNCTIONS[..] {
            mpi_bindings = mpi_bindings.whitelist_function(mpi_function);
            #[cfg(feature = "tool_mode")]
            {
                mpi_bindings = mpi_bindings.whitelist_function("P".to_string() + mpi_function);
            }
        }
        for &mpi_var in &MPI_VARS[..] {
            mpi_bindings = mpi_bindings.whitelist_var(mpi_var);
        }
        mpi_bindings
            .generate()
            .unwrap()
            .write_to_file(mpi_bindings_path)
            .unwrap();
    }
}
