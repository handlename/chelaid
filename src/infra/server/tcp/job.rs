pub(crate) trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F> FnBox for F
where
    F: FnOnce(),
{
    fn call_box(self: Box<Self>) {
        (*self)()
    }
}

pub(crate) type Job = Box<dyn FnBox + Send + 'static>;
