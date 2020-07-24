type_replacements = {}
macro_type_replacements = {}

type_map = {
    "void" : "c_void",
    "char" : "c_char",
    "int" : "c_int",
    "double" : "c_double",
}
macro_type_map = {
    "void" : "::std::os::raw::c_void",
    "char" : "::std::os::raw::c_char",
    "int" : "::std::os::raw::c_int",
    "double" : "::std::os::raw::c_double",
}

mpi_types = {
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
}

qmpi_types = {
    "vector",
}

rust_keywords = {
    "type"
}

macro_mpi_func_id_prefix = "$crate::qmpi_sys::_MPI_funcs::_MPI_"


for qmpi_type in qmpi_types:
    type_map[qmpi_type] = "qmpi_sys::%s" % qmpi_type
for mpi_type in mpi_types:
    type_map[mpi_type] = "mpi_sys::%s" % mpi_type

for qmpi_type in qmpi_types:
    macro_type_map[qmpi_type] = "$crate::qmpi_sys::%s" % qmpi_type
for mpi_type in mpi_types:
    macro_type_map[mpi_type] = "$crate::mpi_sys::%s" % mpi_type


for gen_type_map, gen_type_replacements in \
[(type_map, type_replacements), (macro_type_map, macro_type_replacements)]:
    for c_type, rust_type in gen_type_map.items():
        gen_type_replacements.update({
            (c_type, "") : rust_type,
            ("const %s" % c_type, "") : rust_type,

            ("%s *" % c_type, "") : "*mut %s" % rust_type,
            ("%s ***" % c_type, "") : "*mut *mut *mut %s" % rust_type,
            ("%s" % c_type, "[]") : "*mut %s" % rust_type,
            ("%s" % c_type, "[][3]") : "*mut [%s; 3]" % rust_type,
            ("const %s *" % c_type, "") : "*const %s" % rust_type,
            ("const %s" % c_type, "[]") : "*const %s" % rust_type,
        })
