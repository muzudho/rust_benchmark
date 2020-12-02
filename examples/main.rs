use std::time::{Duration, Instant};
/*
### 実行コマンド。
cargo run --example main
 */

/// 次のどちらが高速か？
///
/// 例: 2,000回で相加平均。1,000,000回実行。
///
/// - [0.031 sec] (a) 関数 add() に引数として構造体を渡し、メンバ変数 count にアクセスする。
/// - [0.034 sec] (ac) 関数 add() に引数として構造体を渡し、コールバック関数を使ってメンバ変数 count にアクセスする。
/// - [0.033 sec] (g) 関数 add<T>() にジェネリック引数として構造体を渡し、コールバック関数を使ってメンバ変数 count にアクセスする。
/// - [0.031 sec] (m) 構造体のメンバ関数 add() が、メンバ変数 self.count にアクセスする。
///
/// - [0.041 sec] (a10) 関数 add() に引数として構造体を渡し、メンバ変数 count にアクセスする。
/// - [0.070 sec] (ac10) 関数 add() に引数として構造体を渡し、コールバック関数を使ってメンバ変数 count にアクセスする。
/// - [0.071 sec] (g10) 関数 add<T>() にジェネリック引数として構造体を渡し、コールバック関数を使ってメンバ変数 count にアクセスする。
/// - [0.039 sec] (m10) 構造体のメンバ関数 add() が、メンバ変数 self.count にアクセスする。
///
/// 遅さ
/// 引数・ジェネリック引数・変数　＜　コールバック関数
fn main() {
    let mut sum_time : Duration;
    let mut stopwatch;
    let mut end;
    let mut sum;

    println!("計測開始。");

    // time 回相加平均。loop_count 回ループする。
    let time = 2_000;
    let loop_count = 1_000_000;

    // (a)
    {
        sum_time = Duration::from_secs(0);
        sum = 0;
        for _j in 0..time {
            stopwatch = Instant::now();
            let mut counter_a = CounterA::new();
            for _i in 0..loop_count {
                add_a(&mut counter_a);
            }
            sum += counter_a.count;
            sum_time += stopwatch.elapsed();    
        }
        end = sum_time / time;
        println!("[{}.{:03} sec] {}回ループする。{}回相加平均。(a) 関数 add() に引数として構造体を渡し、メンバ変数 count にアクセスする。 sum: {}", end.as_secs(), end.subsec_nanos() / 1_000_000, loop_count, time, sum);
    }

    // (ac)
    {
        sum_time = Duration::from_secs(0);
        sum = 0;
        for _j in 0..time {
            stopwatch = Instant::now();
            let mut counter_a = CounterA::new();
            for _i in 0..loop_count {
                add_ac(&mut counter_a, |counter_a|{counter_a.count+=1;});
            }
            sum += counter_a.count;
            sum_time += stopwatch.elapsed();    
        }
        end = sum_time / time;
        println!("[{}.{:03} sec] {}回ループする。{}回相加平均。(ac) 関数 add() に引数として構造体を渡し、コールバック関数を使ってメンバ変数 count にアクセスする。 sum: {}", end.as_secs(), end.subsec_nanos() / 1_000_000, loop_count, time, sum);
    }

    // (g)
    {
        sum_time = Duration::from_secs(0);
        sum = 0;
        for _j in 0..time {
            stopwatch = Instant::now();
            let mut counter_a = CounterA::new();
            for _i in 0..loop_count {
                add_g(&mut counter_a, |t|{t.count += 1;});
            }
            sum += counter_a.count;
            sum_time += stopwatch.elapsed();    
        }
        end = sum_time / time;
        println!("[{}.{:03} sec] {}回ループする。{}回相加平均。(g) 関数 add<T>() にジェネリック引数として構造体を渡し、コールバック関数を使ってメンバ変数 count にアクセスする。 sum: {}", end.as_secs(), end.subsec_nanos() / 1_000_000, loop_count, time, sum);
    }

    // (m)
    {
        sum_time = Duration::from_secs(0);
        sum = 0;
        for _j in 0..time {
            stopwatch = Instant::now();
            let mut counter_m = CounterM::new();
            for _i in 0..loop_count {
                counter_m.add_m();
            }
            sum += counter_m.count;
            sum_time += stopwatch.elapsed();    
        }
        end = sum_time / time;
        println!("[{}.{:03} sec] {}回ループする。{}回相加平均。(m) 構造体のメンバ関数 add() が、メンバ変数 self.count にアクセスする。 sum: {}", end.as_secs(), end.subsec_nanos() / 1_000_000, loop_count, time, sum);
    }

    // (a10)
    {
        sum_time = Duration::from_secs(0);
        sum = 0;
        for _j in 0..time {
            stopwatch = Instant::now();
            let mut counter_a0 = CounterA::new();
            let mut counter_a1 = CounterA::new();
            let mut counter_a2 = CounterA::new();
            let mut counter_a3 = CounterA::new();
            let mut counter_a4 = CounterA::new();
            let mut counter_a5 = CounterA::new();
            let mut counter_a6 = CounterA::new();
            let mut counter_a7 = CounterA::new();
            let mut counter_a8 = CounterA::new();
            let mut counter_a9 = CounterA::new();
            for _i in 0..loop_count {
                add_a10(&mut counter_a0, &mut counter_a1, &mut counter_a2, &mut counter_a3, &mut counter_a4,
                    &mut counter_a5, &mut counter_a6, &mut counter_a7, &mut counter_a8, &mut counter_a9);
            }
            sum += counter_a0.count + counter_a1.count + counter_a2.count + counter_a3.count + counter_a4.count +
                counter_a5.count + counter_a6.count + counter_a7.count + counter_a8.count + counter_a9.count;
            sum_time += stopwatch.elapsed();    
        }
        end = sum_time / time;
        println!("[{}.{:03} sec] {}回ループする。{}回相加平均。(a10) 関数 add() に引数として構造体を渡し、メンバ変数 count にアクセスする。 sum: {}", end.as_secs(), end.subsec_nanos() / 1_000_000, loop_count, time, sum);
    }

    // (ac10)
    {
        sum_time = Duration::from_secs(0);
        sum = 0;
        for _j in 0..time {
            stopwatch = Instant::now();
            let mut counter_a0 = CounterA::new();
            let mut counter_a1 = CounterA::new();
            let mut counter_a2 = CounterA::new();
            let mut counter_a3 = CounterA::new();
            let mut counter_a4 = CounterA::new();
            let mut counter_a5 = CounterA::new();
            let mut counter_a6 = CounterA::new();
            let mut counter_a7 = CounterA::new();
            let mut counter_a8 = CounterA::new();
            let mut counter_a9 = CounterA::new();
            for _i in 0..loop_count {
                add_ac10(&mut counter_a0, &mut counter_a1, &mut counter_a2, &mut counter_a3, &mut counter_a4,
                    &mut counter_a5, &mut counter_a6, &mut counter_a7, &mut counter_a8, &mut counter_a9,
                    |a|{a.count += 1;}, |b|{b.count += 1;}, |c|{c.count += 1;}, |d|{d.count += 1;}, |e|{e.count += 1;},
                    |f|{f.count += 1;}, |g|{g.count += 1;}, |h|{h.count += 1;}, |i|{i.count += 1;}, |j|{j.count += 1;}
                    );
            }
            sum += counter_a0.count + counter_a1.count + counter_a2.count + counter_a3.count + counter_a4.count +
                counter_a5.count + counter_a6.count + counter_a7.count + counter_a8.count + counter_a9.count;
            sum_time += stopwatch.elapsed();    
        }
        end = sum_time / time;
        println!("[{}.{:03} sec] {}回ループする。{}回相加平均。(ac10) 関数 add() に引数として構造体を渡し、コールバック関数を使ってメンバ変数 count にアクセスする。 sum: {}", end.as_secs(), end.subsec_nanos() / 1_000_000, loop_count, time, sum);
    }

    // (g10)
    {
        sum_time = Duration::from_secs(0);
        sum = 0;
        for _j in 0..time {
            stopwatch = Instant::now();
            let mut counter_a0 = CounterA::new();
            let mut counter_a1 = CounterA::new();
            let mut counter_a2 = CounterA::new();
            let mut counter_a3 = CounterA::new();
            let mut counter_a4 = CounterA::new();
            let mut counter_a5 = CounterA::new();
            let mut counter_a6 = CounterA::new();
            let mut counter_a7 = CounterA::new();
            let mut counter_a8 = CounterA::new();
            let mut counter_a9 = CounterA::new();
            for _i in 0..loop_count {
                add_g10(&mut counter_a0, &mut counter_a1, &mut counter_a2, &mut counter_a3, &mut counter_a4,
                    &mut counter_a5, &mut counter_a6, &mut counter_a7, &mut counter_a8, &mut counter_a9,
                    |a|{a.count += 1;}, |b|{b.count += 1;}, |c|{c.count += 1;}, |d|{d.count += 1;}, |e|{e.count += 1;},
                    |f|{f.count += 1;}, |g|{g.count += 1;}, |h|{h.count += 1;}, |i|{i.count += 1;}, |j|{j.count += 1;}
                    );
            }
            sum += counter_a0.count + counter_a1.count + counter_a2.count + counter_a3.count + counter_a4.count +
                counter_a5.count + counter_a6.count + counter_a7.count + counter_a8.count + counter_a9.count;
            sum_time += stopwatch.elapsed();    
        }
        end = sum_time / time;
        println!("[{}.{:03} sec] {}回ループする。{}回相加平均。(g10) 関数 add<T>() にジェネリック引数として構造体を渡し、コールバック関数を使ってメンバ変数 count にアクセスする。 sum: {}", end.as_secs(), end.subsec_nanos() / 1_000_000, loop_count, time, sum);
    }

    // (m10)
    {
        sum_time = Duration::from_secs(0);
        sum = 0;
        for _j in 0..time {
            stopwatch = Instant::now();
            let mut counter_m10 = CounterM10::new();
            for _i in 0..loop_count {
                counter_m10.add_m10();
            }
            sum += counter_m10.count_m10();
            sum_time += stopwatch.elapsed();    
        }
        end = sum_time / time;
        println!("[{}.{:03} sec] {}回ループする。{}回相加平均。(m10) 構造体のメンバ関数 add() が、メンバ変数 self.count にアクセスする。 sum: {}", end.as_secs(), end.subsec_nanos() / 1_000_000, loop_count, time, sum);
    }
}

struct CounterA {
    count: i64,
}
impl CounterA {
    fn new() -> CounterA {
        CounterA {
            count: 0,
        }
    }
}
fn add_a(counter_a: &mut CounterA) {
    counter_a.count += 1;
}
fn add_ac(counter_a: &mut CounterA, callback: fn(&mut CounterA)) {
    (callback)(counter_a);
}

fn add_g<T>(t: &mut T, callback: fn(&mut T)) {
    (callback)(t);
}

struct CounterM {
    count: i64,
}
impl CounterM {
    fn new() -> CounterM {
        CounterM {
            count: 0,
        }
    }

    fn add_m(&mut self) {
        self.count += 1;
    }
}

fn add_a10(counter_a0: &mut CounterA, counter_a1: &mut CounterA, counter_a2: &mut CounterA, counter_a3: &mut CounterA, counter_a4: &mut CounterA,
    counter_a5: &mut CounterA, counter_a6: &mut CounterA, counter_a7: &mut CounterA, counter_a8: &mut CounterA, counter_a9: &mut CounterA,
) {
    counter_a0.count += 1;
    counter_a1.count += 1;
    counter_a2.count += 1;
    counter_a3.count += 1;
    counter_a4.count += 1;
    counter_a5.count += 1;
    counter_a6.count += 1;
    counter_a7.count += 1;
    counter_a8.count += 1;
    counter_a9.count += 1;
}
fn add_ac10(a: &mut CounterA, b: &mut CounterA, c: &mut CounterA, d: &mut CounterA, e: &mut CounterA,
    f: &mut CounterA, g: &mut CounterA, h: &mut CounterA, i: &mut CounterA, j: &mut CounterA,
    callback_a: fn(&mut CounterA), callback_b: fn(&mut CounterA), callback_c: fn(&mut CounterA), callback_d: fn(&mut CounterA), callback_e: fn(&mut CounterA), 
    callback_f: fn(&mut CounterA), callback_g: fn(&mut CounterA), callback_h: fn(&mut CounterA), callback_i: fn(&mut CounterA), callback_j: fn(&mut CounterA),
) {
    callback_a(a);
    callback_b(b);
    callback_c(c);
    callback_d(d);
    callback_e(e);
    callback_f(f);
    callback_g(g);
    callback_h(h);
    callback_i(i);
    callback_j(j);
}

fn add_g10<A, B, C, D, E, F, G, H, I, J>(a: &mut A, b: &mut B, c: &mut C, d: &mut D, e: &mut E,
    f: &mut F, g: &mut G, h: &mut H, i: &mut I, j: &mut J,
    callback_a: fn(&mut A), callback_b: fn(&mut B), callback_c: fn(&mut C), callback_d: fn(&mut D), callback_e: fn(&mut E), 
    callback_f: fn(&mut F), callback_g: fn(&mut G), callback_h: fn(&mut H), callback_i: fn(&mut I), callback_j: fn(&mut J)
) {
    callback_a(a);
    callback_b(b);
    callback_c(c);
    callback_d(d);
    callback_e(e);
    callback_f(f);
    callback_g(g);
    callback_h(h);
    callback_i(i);
    callback_j(j);
}

struct CounterM10 {
    count0: i64,
    count1: i64,
    count2: i64,
    count3: i64,
    count4: i64,
    count5: i64,
    count6: i64,
    count7: i64,
    count8: i64,
    count9: i64,
}
impl CounterM10 {
    fn new() -> CounterM10 {
        CounterM10 {
            count0: 0,
            count1: 0,
            count2: 0,
            count3: 0,
            count4: 0,
            count5: 0,
            count6: 0,
            count7: 0,
            count8: 0,
            count9: 0,
        }
    }

    fn add_m10(&mut self) {
        self.count0 += 1;
        self.count1 += 1;
        self.count2 += 1;
        self.count3 += 1;
        self.count4 += 1;
        self.count5 += 1;
        self.count6 += 1;
        self.count7 += 1;
        self.count8 += 1;
        self.count9 += 1;
    }

    fn count_m10(&mut self) -> i64 {
        self.count0 + self.count1 + self.count2 + self.count3 + self.count4 +
        self.count5 + self.count6 + self.count7 + self.count8 + self.count9
    }
}
