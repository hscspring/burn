use crate::graph::{
    converter::Forward2BackwardGraphConverter, grad::Grads, node::BackwardNodeState,
};
use std::sync::Arc;

#[derive(new)]
pub struct BinaryOpsNodeState<'a, Lhs, Rhs, Out> {
    pub left: &'a BackwardNodeState<Lhs>,
    pub right: &'a BackwardNodeState<Rhs>,
    pub output: &'a BackwardNodeState<Out>,
}

#[derive(new)]
pub struct UnaryOpsNodeState<'a, In, Out> {
    pub input: &'a BackwardNodeState<In>,
    pub output: &'a BackwardNodeState<Out>,
}

pub trait BackwardRecordedOps<T>: std::fmt::Debug {
    fn backward_step(&self, state: &BackwardNodeState<T>);
    fn backward_parents(&self) -> Vec<RecordedOpsParentRef>;
}

pub trait ForwardRecordedOps<T>: std::fmt::Debug + Send + Sync {
    fn to_backward(
        &self,
        graph: &mut Forward2BackwardGraphConverter,
    ) -> BackwardRecordedOpsBoxed<T>;
}

pub trait RecordedOpsParent: std::fmt::Debug {
    fn order(&self) -> usize;
    fn id(&self) -> &String;
    fn backward_step(&self);
    fn backward_parents(&self) -> Vec<RecordedOpsParentRef>;
    fn register_grad(&self, grads: &mut Grads);
}

pub type ForwardRecordedOpsBoxed<T> = Box<dyn ForwardRecordedOps<T>>;
pub type BackwardRecordedOpsBoxed<T> = Box<dyn BackwardRecordedOps<T>>;
pub type RecordedOpsParentRef = Arc<dyn RecordedOpsParent>;
