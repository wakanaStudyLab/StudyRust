//! ============================================================================
//! モジュール 07: 並行処理 & スレッド安全 (Concurrency & Thread Safety)
//! ============================================================================
//!
//! 【他言語経験者向け要点】
//! 1. "Fearless Concurrency" (恐れなき並行性):
//!    - `Send` と `Sync` の2大マーカートレイトにより、データ競合のある並行コードは
//!      コンパイルエラーとして弾かれる。
//!
//! 2. スコープ付きスレッド (std::thread::scope - Rust 1.63+):
//!    - スレッドのライフタイムがスコープ内に収まることをコンパイラが保証するため、
//!      `Arc` や `'static` なしで親スタックの参照 (`&T`, `&mut T`) を直接安全に共有可能！
//!
//! 3. メッセージパッシング (mpsc channel):
//!    - "Do not communicate by sharing memory; instead, share memory by communicating."
//!
//! 4. 共有メモリ並行性:
//!    - `Arc<Mutex<T>>`: Mutex ガードがスコープを抜けると自動でロック解放（Unlock忘れ不可能）。

use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

pub fn run() {
    demo_thread_spawn_and_join();
    demo_scoped_threads();
    demo_mpsc_channels();
    demo_arc_mutex_shared_state();
}

fn demo_thread_spawn_and_join() {
    println!("=== 1. Basic Thread Spawning & Join ===");

    let handle = thread::spawn(|| {
        for i in 1..=3 {
            println!("  [Worker Thread] Working step {}...", i);
            thread::sleep(Duration::from_millis(15));
        }
        "Worker completed"
    });

    let result = handle.join().unwrap();
    println!("Main thread received: {}", result);
}

fn demo_scoped_threads() {
    println!("\n=== 2. Scoped Threads (std::thread::scope - Rust 1.63+) ===");

    let mut numbers = vec![1, 2, 3];

    // 'static 制約なし！ローカル配列への参照を別スレッドに渡して並行処理できる
    thread::scope(|s| {
        s.spawn(|| {
            println!("  [Scoped Thread 1] Reading numbers: {:?}", numbers);
        });

        s.spawn(|| {
            println!("  [Scoped Thread 2] Numbers length: {}", numbers.len());
        });
    }); // ここで全スコープ付きスレッドが自動的に join される

    numbers.push(4);
    println!("Numbers after scoped threads: {:?}", numbers);
}

fn demo_mpsc_channels() {
    println!("\n=== 3. Message Passing with MPSC Channels (Go-like) ===");

    let (tx, rx) = mpsc::channel();

    // 送信側スレッド
    thread::spawn(move || {
        let messages = ["task_started", "task_processing", "task_done"];
        for msg in messages {
            tx.send(msg).unwrap();
            thread::sleep(Duration::from_millis(10));
        }
    });

    // 受信側 (メインスレッド)
    for received in rx {
        println!("  [Channel Receiver]: Got message '{}'", received);
    }
}

fn demo_arc_mutex_shared_state() {
    println!("\n=== 4. Shared State Concurrency (Arc<Mutex<T>>) ===");

    // 複数スレッドで共有するスレッドセーフなカウンタ
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..5 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            // lock() で MutexGuard を取得。スコープを抜けると自動 Unlock
            let mut num = counter_clone.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final shared counter value: {}", *counter.lock().unwrap());
}
