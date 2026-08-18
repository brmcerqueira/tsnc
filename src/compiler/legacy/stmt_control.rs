use melior::ir::Block;

pub(in crate::compiler) enum StmtControl<'c> {
    Continue,
    Terminated,
    Branch(Block<'c>),
}
