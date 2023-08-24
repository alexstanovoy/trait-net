use core::future::Future;

pub trait Stream<Request> {
    type Response;
    type Function: Future<Output = Self::Response>;

    fn next(&self, request: Request) -> Self::Function;
}

impl<T, Function, Arg1> Stream<(Arg1,)> for T
where
    T: Fn(Arg1) -> Function,
    Function: Future,
{
    type Response = Function::Output;
    type Function = Function;

    fn next(&self, request: (Arg1,)) -> Self::Function {
        (self)(request.0)
    }
}

impl<T, Function, Arg1, Arg2> Stream<(Arg1, Arg2)> for T
where
    T: Fn(Arg1, Arg2) -> Function,
    Function: Future,
{
    type Response = Function::Output;
    type Function = Function;

    fn next(&self, request: (Arg1, Arg2)) -> Self::Function {
        (self)(request.0, request.1)
    }
}

impl<T, Function, Arg1, Arg2, Arg3> Stream<(Arg1, Arg2, Arg3)> for T
where
    T: Fn(Arg1, Arg2, Arg3) -> Function,
    Function: Future,
{
    type Response = Function::Output;
    type Function = Function;

    fn next(&self, request: (Arg1, Arg2, Arg3)) -> Self::Function {
        (self)(request.0, request.1, request.2)
    }
}
