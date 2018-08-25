#[macro_use]
extern crate lazy_static;
/*
### Cargo.tomlに書き足しておけだぜ☆（＾ｑ＾）
[dependencies]
lazy_static = "1.1.0"

### 実行コマンド
cargo run --release
 */

use std::sync::RwLock;
use std::time::{Duration, Instant};

// グローバル変数。
lazy_static! {
    static ref COUNTER: RwLock<Counter> = RwLock::new(Counter::new());
    static ref DOUBLE_COUNTER: RwLock<DoubleCounter> = RwLock::new(DoubleCounter::new());
}

// 適当な構造体。
pub struct Counter {
    pub count: i64
}
impl Counter {
    pub fn new() -> Counter {
        Counter {
            count: 0
        }
    }
}
impl CounterAccessor for Counter {
    fn get_count(&self) -> i64 {
        self.count
    }

    fn set_count(&mut self, value: i64) {
        self.count = value;
    }
}
trait CounterAccessor {
    fn get_count(&self) -> i64 {
        0
    }

    fn set_count(&mut self, _value: i64) {
    }
}

// 適当な構造体。
pub struct DoubleCounter {
    pub count: i64,
    pub count_a: i64
}
impl DoubleCounter {
    pub fn new() -> DoubleCounter {
        DoubleCounter {
            count: 0,
            count_a: 0
        }
    }
}

// 1を足す関数。
fn count_number(count: i64) -> i64 {
    count + 1
}
// 2つを足す関数。
fn add(count: i64, count_a: i64) -> i64 {
    count + count_a
}
// 構造体を渡す。
fn count_number_b(counter: &mut Counter) {
    counter.count += 1;
}
// ジェネリック引数を渡す関数。
fn count_number_c<T>(counter: &mut T)
    where T : CounterAccessor
{
    let num = counter.get_count();
    counter.set_count(num + 1);
}

fn main() {
    let mut sum;
    let mut sum_time : Duration;
    let mut stopwatch;
    let mut end;

    {
        // 100万回 空ループする。1,000,000回相加平均。
        let time = 1_000_000;
        sum_time = Duration::from_secs(0);
        for _j in 0..time {
            stopwatch = Instant::now();
            for _i in 0..1_000_000 {

            }
            sum_time += stopwatch.elapsed();    
        }
        end = sum_time / time;
        println!("[{}.{:03} sec] 100万回 空ループする。{}回相加平均。", end.as_secs(), end.subsec_nanos() / 1_000_000, time);
        // [0.034 sec]
    }


    {
        // コールバック関数に引数を 100万回渡す。1,000,000回相加平均。
        let time = 1_000_000;
        sum_time = Duration::from_secs(0);
        sum = 0;
        for _j in 0..time {
            stopwatch = Instant::now();
            let callback = count_number;
            for _i in 0..1_000_000 {
                sum = (callback)(sum);
            }
            sum_time += stopwatch.elapsed();    
        }
        end = sum_time / time;
        println!("[{}.{:03} sec] コールバック関数に引数を 100万回渡す。{}回相加平均。 sum: {}.", end.as_secs(), end.subsec_nanos() / 1_000_000, time, sum);
        // [0.034 sec]
    }


    {
        // 関数に引数を 100万回渡す。1,000,000回相加平均。
        let time = 1_000_000;
        sum_time = Duration::from_secs(0);
        sum = 0;
        for _j in 0..time {
            stopwatch = Instant::now();
            for _i in 0..1_000_000 {
                sum = count_number(sum);
            }
            sum_time += stopwatch.elapsed();    
        }
        end = sum_time / time;
        println!("[{}.{:03} sec] 関数に引数を 100万回渡す。{}回相加平均。 sum: {}.", end.as_secs(), end.subsec_nanos() / 1_000_000, time, sum);
        // [0.035 sec]
    }


    {
        // 関数に引数２つを 100万回渡す。1,000,000回相加平均。
        let time = 1_000_000;
        sum_time = Duration::from_secs(0);
        sum = 0;
        for _j in 0..time {
            stopwatch = Instant::now();
            for _i in 0..1_000_000 {
                sum = add(sum,1);
            }
            sum_time += stopwatch.elapsed();    
        }
        end = sum_time / time;
        println!("[{}.{:03} sec] 関数に引数２つを 100万回渡す。{}回相加平均。 sum: {}.", end.as_secs(), end.subsec_nanos() / 1_000_000, time, sum);
        // [0.035 sec]
    }


    {
        // 関数に構造体を 100万回渡して そのメンバを加算する。1,000,000回相加平均。
        let time = 1_000_000;
        sum_time = Duration::from_secs(0);
        sum = 0;
        for _j in 0..time {
            stopwatch = Instant::now();
            let mut counter = Counter::new();
            for _i in 0..1_000_000 {
                count_number_b(&mut counter);
            }
            sum += counter.count;
            sum_time += stopwatch.elapsed();    
        }
        end = sum_time / time;
        println!("[{}.{:03} sec] 関数に構造体を 100万回渡して そのメンバを加算する。{}回相加平均: sum: {}.", end.as_secs(), end.subsec_nanos() / 1_000_000, time, sum);
        // [0.034 sec]
    }


    {
        // ジェネリック関数に構造体を 100万回渡して そのメンバを加算する。1,000,000回相加平均。
        let time = 1_000_000;
        sum_time = Duration::from_secs(0);
        sum = 0;
        for _j in 0..time {
            stopwatch = Instant::now();
            let mut counter = Counter::new();
            for _i in 0..1_000_000 {
                count_number_c::<Counter>(&mut counter);
            }
            sum += counter.count;
            sum_time += stopwatch.elapsed();    
        }
        end = sum_time / time;
        println!("[{}.{:03} sec] ジェネリック関数に構造体を 100万回渡して そのメンバを加算する。{}回相加平均: sum: {}.", end.as_secs(), end.subsec_nanos() / 1_000_000, time, sum);
        // [0.036 sec]
    }


    {
        // グローバル変数に 100万回 書き込む。ロック時間含む。100回相加平均。
        let time = 100;
        sum_time = Duration::from_secs(0);
        {
            COUNTER.try_write().unwrap().count = 0;
        }
        for _j in 0..time {
            stopwatch = Instant::now();
            for _i in 0..1_000_000 {
                COUNTER.try_write().unwrap().count += 1;
            }
            sum_time += stopwatch.elapsed();    
        }
        end = sum_time / time;
        println!("[{}.{:03} sec] グローバル変数に 100万回 書き込む。ロック時間含む。{}回相加平均: sum: {}.", end.as_secs(), end.subsec_nanos() / 1_000_000, time, COUNTER.try_read().unwrap().count);
        // [0.116 sec]
    }


    {
        // グローバル構造体のメンバ変数２つに 100万回ずつ 書き込む。ロック時間含む。100回相加平均。
        let time = 100;
        sum_time = Duration::from_secs(0);
        {
            let mut double_counter = DOUBLE_COUNTER.try_write().unwrap();
            double_counter.count = 0;
            double_counter.count_a = 0;
        }
        for _j in 0..time {
            stopwatch = Instant::now();
            for _i in 0..1_000_000 {
                let mut double_counter = DOUBLE_COUNTER.try_write().unwrap();
                double_counter.count += 1;
                double_counter.count_a += 1;
            }
            sum_time += stopwatch.elapsed();    
        }
        sum = 0;
        {
            let double_counter = DOUBLE_COUNTER.try_read().unwrap();
            sum += double_counter.count;
            sum += double_counter.count_a;
        }
        end = sum_time / time;
        println!("[{}.{:03} sec] グローバル構造体のメンバ変数２つに 100万回ずつ 書き込む。ロック時間含む。{}回相加平均: sum: {}.", end.as_secs(), end.subsec_nanos() / 1_000_000, time, sum);
        // [0.117 sec]
    }
}
