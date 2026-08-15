use melior::ir::Block;

pub(super) enum StmtControl<'c> {
    Continue,
    Terminated,
    Branch(Block<'c>),
}
