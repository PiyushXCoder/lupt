use super::*;

//################################################## Helper ##################################################
#[derive(Debug)]
pub enum Resp {
    Ok,
    Err(String), 
    None
}

impl<A, M> MessageResponse<A, M> for Resp
where
    A: Actor,
    M: Message<Result = Resp>,
{
    fn handle<R: ResponseChannel<M>>(self, _: &mut A::Context, tx: Option<R>) {
        if let Some(tx) = tx {
            tx.send(self);
        }
    }
}