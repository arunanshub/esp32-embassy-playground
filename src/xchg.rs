use embassy_sync::{
    blocking_mutex::raw::CriticalSectionRawMutex,
    channel::{Receiver, Sender},
};

type RawMutexSender<'channel, T, const N: usize> = Sender<'channel, CriticalSectionRawMutex, T, N>;

type RawMutexReceiver<'channel, T, const N: usize> =
    Receiver<'channel, CriticalSectionRawMutex, T, N>;

#[derive(Clone, Copy)]
pub struct XchgPipeTx<'channel, T: Sized, const N: usize> {
    sender: RawMutexSender<'channel, T, N>,
    // other fields
}

impl<'channel, T: Sized, const N: usize> XchgPipeTx<'channel, T, N> {
    pub async fn send(&self, value: T) {
        self.sender.send(value).await;
    }
}

#[derive(Clone, Copy)]
pub struct XchgPipeRx<'channel, T: Sized, const N: usize> {
    reciever: RawMutexReceiver<'channel, T, N>,
    // other fields
}

impl<'channel, T: Sized, const N: usize> XchgPipeRx<'channel, T, N> {
    pub async fn receive(&self) -> T {
        self.reciever.receive().await
    }

    pub async fn recv(&self) -> T {
        self.receive().await
    }
}

pub fn channel<T, const N: usize>(
    channel: &embassy_sync::channel::Channel<CriticalSectionRawMutex, T, N>,
) -> (XchgPipeTx<'_, T, N>, XchgPipeRx<'_, T, N>) {
    let tx = XchgPipeTx {
        sender: channel.sender(),
    };
    let rx = XchgPipeRx {
        reciever: channel.receiver(),
    };
    (tx, rx)
}
