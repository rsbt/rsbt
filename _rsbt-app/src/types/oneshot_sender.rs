pub trait OneshotSender<M> {
    fn send(self, message: M) -> Result<(), M>;
}
