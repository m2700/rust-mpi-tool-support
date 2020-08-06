#include <stddef.h>
#include <mpi.h>

const MPI_Datatype RUST_MPI_DATATYPE_NULL = MPI_DATATYPE_NULL;

const MPI_Datatype RUST_MPI_CHAR = MPI_CHAR;
const MPI_Datatype RUST_MPI_SIGNED_CHAR = MPI_SIGNED_CHAR;
const MPI_Datatype RUST_MPI_UNSIGNED_CHAR = MPI_UNSIGNED_CHAR;
const MPI_Datatype RUST_MPI_BYTE = MPI_BYTE;
const MPI_Datatype RUST_MPI_WCHAR = MPI_WCHAR;
const MPI_Datatype RUST_MPI_SHORT = MPI_SHORT;
const MPI_Datatype RUST_MPI_UNSIGNED_SHORT = MPI_UNSIGNED_SHORT;
const MPI_Datatype RUST_MPI_INT = MPI_INT;
const MPI_Datatype RUST_MPI_UNSIGNED = MPI_UNSIGNED;
const MPI_Datatype RUST_MPI_LONG = MPI_LONG;
const MPI_Datatype RUST_MPI_UNSIGNED_LONG = MPI_UNSIGNED_LONG;
const MPI_Datatype RUST_MPI_LONG_DOUBLE = MPI_LONG_DOUBLE;
const MPI_Datatype RUST_MPI_LONG_LONG_INT = MPI_LONG_LONG_INT;
const MPI_Datatype RUST_MPI_UNSIGNED_LONG_LONG = MPI_UNSIGNED_LONG_LONG;
const MPI_Datatype RUST_MPI_LONG_LONG = MPI_LONG_LONG;

const MPI_Datatype RUST_C_BOOL = MPI_C_BOOL;

const MPI_Datatype RUST_MPI_FLOAT = MPI_FLOAT;
const MPI_Datatype RUST_MPI_DOUBLE = MPI_DOUBLE;

const MPI_Datatype RUST_INT8_T = MPI_INT8_T;
const MPI_Datatype RUST_INT16_T = MPI_INT16_T;
const MPI_Datatype RUST_INT32_T = MPI_INT32_T;
const MPI_Datatype RUST_INT64_T = MPI_INT64_T;

const MPI_Datatype RUST_UINT8_T = MPI_UINT8_T;
const MPI_Datatype RUST_UINT16_T = MPI_UINT16_T;
const MPI_Datatype RUST_UINT32_T = MPI_UINT32_T;
const MPI_Datatype RUST_UINT64_T = MPI_UINT64_T;

const MPI_Datatype RUST_MPI_C_COMPLEX = MPI_C_COMPLEX;
const MPI_Datatype RUST_MPI_C_FLOAT_COMPLEX = MPI_C_FLOAT_COMPLEX;
const MPI_Datatype RUST_MPI_C_DOUBLE_COMPLEX = MPI_C_DOUBLE_COMPLEX;
const MPI_Datatype RUST_MPI_C_LONG_DOUBLE_COMPLEX = MPI_C_LONG_DOUBLE_COMPLEX;

const MPI_Datatype RUST_MPI_LONG_INT = MPI_LONG_INT;
struct LongInt
{
    long l;
    int i;
};

const MPI_Datatype RUST_MPI_DOUBLE_INT = MPI_DOUBLE_INT;
struct DoubleInt
{
    double d;
    int i;
};

const MPI_Datatype RUST_MPI_SHORT_INT = MPI_SHORT_INT;
struct ShortInt
{
    short s;
    int i;
};

const MPI_Datatype RUST_MPI_2INT = MPI_2INT;
struct TwoInt
{
    int i1;
    int i2;
};

const MPI_Datatype RUST_MPI_LONG_DOUBLE_INT = MPI_LONG_DOUBLE_INT;
struct LongDoubleInt
{
    long double ld;
    int i;
};

const MPI_Comm RUST_MPI_COMM_NULL = MPI_COMM_NULL;
const MPI_Comm RUST_MPI_COMM_WORLD = MPI_COMM_WORLD;
const MPI_Comm RUST_MPI_COMM_SELF = MPI_COMM_SELF;

const int RUST_MPI_SUCCESS = MPI_SUCCESS;

const int RUST_MPI_OP_NULL = MPI_OP_NULL;
const int RUST_MPI_MAX = MPI_MAX;
const int RUST_MPI_MIN = MPI_MIN;
const int RUST_MPI_SUM = MPI_SUM;
const int RUST_MPI_PROD = MPI_PROD;
const int RUST_MPI_LAND = MPI_LAND;
const int RUST_MPI_BAND = MPI_BAND;
const int RUST_MPI_LOR = MPI_LOR;
const int RUST_MPI_BOR = MPI_BOR;
const int RUST_MPI_LXOR = MPI_LXOR;
const int RUST_MPI_BXOR = MPI_BXOR;
const int RUST_MPI_MINLOC = MPI_MINLOC;
const int RUST_MPI_MAXLOC = MPI_MAXLOC;
const int RUST_MPI_REPLACE = MPI_REPLACE;

const int RUST_MPI_ANY_TAG = MPI_ANY_TAG;

const MPI_Request RUST_MPI_REQUEST_NULL = MPI_REQUEST_NULL;

const size_t RUST_MPI_STATUS_IGNORE = (size_t)MPI_STATUS_IGNORE;
const size_t RUST_MPI_STATUSES_IGNORE = (size_t)MPI_STATUSES_IGNORE;