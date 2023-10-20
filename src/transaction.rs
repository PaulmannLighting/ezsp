use std::fmt::Debug;

#[derive(Debug, Eq, PartialEq)]
pub enum Transaction<C, R>
where
    C: Debug + Eq,
    R: Debug + Eq,
{
    Command(C),
    Response(R),
}
