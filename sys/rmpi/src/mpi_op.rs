local_mod!(
    use mpi_sys::{
        MPI_Op, MPI_BAND, MPI_BOR, MPI_BXOR, MPI_LAND, MPI_LOR, MPI_LXOR, MPI_MAX, MPI_MAXLOC,
        MPI_MIN, MPI_MINLOC, MPI_OP_NULL, MPI_PROD, MPI_REPLACE, MPI_SUM,
    };
);

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
    UserDefined(UserOperation),
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
            mpi_op => Self::UserDefined(UserOperation(mpi_op)),
        }
    }
}
impl Into<MPI_Op> for MpiOp {
    #[inline]
    fn into(self) -> MPI_Op {
        match self {
            Self::Null => MPI_OP_NULL,
            Self::Max => MPI_MAX,
            Self::Min => MPI_MIN,
            Self::Sum => MPI_SUM,
            Self::Prod => MPI_PROD,
            Self::Land => MPI_LAND,
            Self::Band => MPI_BAND,
            Self::Lor => MPI_LOR,
            Self::Bor => MPI_BOR,
            Self::Lxor => MPI_LXOR,
            Self::Bxor => MPI_BXOR,
            Self::Minloc => MPI_MINLOC,
            Self::Maxloc => MPI_MAXLOC,
            Self::Replace => MPI_REPLACE,
            Self::UserDefined(UserOperation(mpi_op)) => mpi_op,
        }
    }
}

pub struct UserOperation(MPI_Op);
