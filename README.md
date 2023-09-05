# BasicAlgorithms

## 复杂度

时间换空间 / 空间换时间

### 时间复杂度



### 空间复杂度


### 大O表示法
1. 用常数1取代运行时间中的所有加法常数
2. 在修改后的运行次数函数中，只保留最高阶项
3. 如果最高阶项存在且不是1，则去除与这个项相乘的常数

### 常见的时间复杂度
O(1) < O(logn) < O(n) < O(nlogn) < O(n^2) < O(n^3) < O(2^n) < O(n!)

#### O(n)
```rust
for i in 0..n {
    println!("{}", i);
}
```

#### O(n^2)
```rust
for i in 0..n {
    for j in 0..n {
        println!("{} {}", i, j);
    }
}
```

#### O(logn)
```rust
let mut i = 1;
while i < n {
    println!("{}", i);
    i *= 2;
}
```

#### O(nm)
```rust
for i in 0..n {
    for j in 0..m {
        println!("{} {}", i, j);
    }
}
```

#### O(nlogn)
```rust
for i in 0..n {
    let mut j = 1;
    while j < n {
        println!("{} {}", i, j);
        j *= 2;
    }
}
```

#### O(sqrt(n))
```rust
let mut i = 1;
while i * i < n {
    println!("{}", i);
    i += 1;
}
```

#### O(2^n)
```rust
fn fib(n: i32) -> i32 {
    if n == 1 || n == 2 {
        return 1;
    }
    fib(n - 1) + fib(n - 2)
}
```

#### O(n!)
```rust
fn permute(a: &mut Vec<i32>, start: usize, end: usize) {
    if start == end {
        println!("{:?}", a);
    } else {
        for i in start..end {
            a.swap(start, i);
            permute(a, start + 1, end);
            a.swap(start, i);
        }
    }
}
```


