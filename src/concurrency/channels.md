# Channels

러스트의 채널은 Sender<T>` 와 `Receiver<T>` 두 부분으로 구성됩니다.
두 부분은 채널을 통해 연결되지만 우리는 종단만을 확인할 수 있습니다.
> Rust channels have two parts: a `Sender<T>` and a `Receiver<T>`. The two parts
> are connected via the channel, but you only see the end-points.

```rust,editable
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    tx.send(10).unwrap();
    tx.send(20).unwrap();

    println!("Received: {:?}", rx.recv());
    println!("Received: {:?}", rx.recv());

    let tx2 = tx.clone();
    tx2.send(30).unwrap();
    println!("Received: {:?}", rx.recv());
}
```
--- 
역주 
- 메세지 패싱하는 transmitter(tx)와 receiver(rx) 입니다. 보통 예시는 흐르는 개울(채널) 상류(tx)에 띄운 고무 오리가 하류(rx)로 가는 흐름을 생각하시면 됩니다.
