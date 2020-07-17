use mpi_sys::{
    MPI_Op, MPI_BAND, MPI_BOR, MPI_BXOR, MPI_LAND, MPI_LOR, MPI_LXOR, MPI_MAX, MPI_MAXLOC, MPI_MIN,
    MPI_MINLOC, MPI_OP_NULL, MPI_PROD, MPI_REPLACE, MPI_SUM,
};

pub enum MpiOp {
    Null,
    Max,
    Min,
    Sum,
    Prod,
    Land,
    Band,
    Lor,
    Bor,
    Lxor,
    Bxor,
    Minloc,
    Maxloc,
    Replace,
}
impl From<MPI_Op> for MpiOp {
    #[inline]
    fn from(src: MPI_Op) -> Self {
        match src {
            MPI_OP_NULL => Self::Null,
            MPI_MAX => Self::Max,
            MPI_MIN => Self::Min,
            MPI_SUM => Self::Sum,
            MPI_PROD => Self::Prod,
            MPI_LAND => Self::Land,
            MPI_BAND => Self::Band,
            MPI_LOR => Self::Lor,
            MPI_BOR => Self::Bor,
            MPI_LXOR => Self::Lxor,
            MPI_BXOR => Self::Bxor,
            MPI_MINLOC => Self::Minloc,
            MPI_MAXLOC => Self::Maxloc,
            MPI_REPLACE => Self::Replace,
            mpi_op => panic!("unexpected mpi operation is: {}", mpi_op),
        }
    }
}
