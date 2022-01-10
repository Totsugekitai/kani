use crate::print;
use conquer_once::spin::OnceCell;
use core::{
    pin::Pin,
    task::{Context, Poll},
};
use crossbeam::queue::ArrayQueue;
use futures_util::{stream::Stream, task::AtomicWaker, StreamExt};
use log::warn;

static ASCII_QUEUE: OnceCell<ArrayQueue<u8>> = OnceCell::uninit();
static WAKER: AtomicWaker = AtomicWaker::new();

pub(crate) fn add_ascii(ascii: u8) {
    if let Ok(queue) = ASCII_QUEUE.try_get() {
        if let Err(_) = queue.push(ascii) {
            warn!("ascii queue full; dropping serial input");
        } else {
            WAKER.wake();
        }
    } else {
        warn!("ascii queue uninitialized");
    }
}

pub struct AsciiStream {
    _private: (),
}

impl AsciiStream {
    pub fn new() -> Self {
        ASCII_QUEUE
            .try_init_once(|| ArrayQueue::new(100))
            .expect("AsciiStream::new should only be called once");
        AsciiStream { _private: () }
    }
}

impl Stream for AsciiStream {
    type Item = u8;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let queue = ASCII_QUEUE.try_get().expect("not initialized");

        if let Some(ascii) = queue.pop() {
            return Poll::Ready(Some(ascii));
        }

        WAKER.register(&cx.waker());
        match queue.pop() {
            Some(ascii) => {
                WAKER.take();
                Poll::Ready(Some(ascii))
            }
            None => Poll::Pending,
        }
    }
}

pub async fn print_keypresses() {
    let mut ascii_codes = AsciiStream::new();
    while let Some(ascii) = ascii_codes.next().await {
        print!("{}", ascii as char);
    }
}
