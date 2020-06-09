use crate::App;

pub trait IncomingConnection<A: App>: sealed::IncomingConnectionPriv {
    fn test(&self) {
        self.test_priv();
    }
}

mod sealed {
    use crate::App;

    pub trait IncomingConnectionPriv {
        fn test_priv(&self);
    }
}
