# 1m-task-perf-test

Inspired by Piotr Kołaczkowski's blog post "How Much Memory Do You Need to Run 1 Million Concurrent Tasks?", I would like to retry that with Node and Rust...

## Result

> running on MacBook Air M1 (2020) with 16GB RAM and Asahi Linux (Arch Linux ARM)

### Node.js

```
1000000 items pushed into target.
    Command being timed: "node nodejs/index.js"
    User time (seconds): 0.47
    System time (seconds): 0.04
    Percent of CPU this job got: 158%
    Elapsed (wall clock) time (h:mm:ss or m:ss): 0:00.32
    Average shared text size (kbytes): 0
    Average unshared data size (kbytes): 0
    Average stack size (kbytes): 0
    Average total size (kbytes): 0
    Maximum resident set size (kbytes): 290112
    Average resident set size (kbytes): 0
    Major (requiring I/O) page faults: 0
    Minor (reclaiming a frame) page faults: 20006
    Voluntary context switches: 1491
    Involuntary context switches: 2
    Swaps: 0
    File system inputs: 0
    File system outputs: 0
    Socket messages sent: 0
    Socket messages received: 0
    Signals delivered: 0
    Page size (bytes): 16384
    Exit status: 0
```

### Rust (Tokio, with default configuration)

```
1000000 items pushed into target.
    Command being timed: "rust-multithread/target/release/rust"
    User time (seconds): 2.98
    System time (seconds): 1.54
    Percent of CPU this job got: 599%
    Elapsed (wall clock) time (h:mm:ss or m:ss): 0:00.75
    Average shared text size (kbytes): 0
    Average unshared data size (kbytes): 0
    Average stack size (kbytes): 0
    Average total size (kbytes): 0
    Maximum resident set size (kbytes): 264016
    Average resident set size (kbytes): 0
    Major (requiring I/O) page faults: 0
    Minor (reclaiming a frame) page faults: 17033
    Voluntary context switches: 251548
    Involuntary context switches: 164696
    Swaps: 0
    File system inputs: 0
    File system outputs: 0
    Socket messages sent: 0
    Socket messages received: 0
    Signals delivered: 0
    Page size (bytes): 16384
    Exit status: 0
```

### Rust (Tokio, with `flavor = "current_thread"`)

```
1000000 items pushed into target.
    Command being timed: "rust/target/release/rust"
    User time (seconds): 0.28
    System time (seconds): 0.01
    Percent of CPU this job got: 99%
    Elapsed (wall clock) time (h:mm:ss or m:ss): 0:00.30
    Average shared text size (kbytes): 0
    Average unshared data size (kbytes): 0
    Average stack size (kbytes): 0
    Average total size (kbytes): 0
    Maximum resident set size (kbytes): 236032
    Average resident set size (kbytes): 0
    Major (requiring I/O) page faults: 0
    Minor (reclaiming a frame) page faults: 14745
    Voluntary context switches: 1
    Involuntary context switches: 0
    Swaps: 0
    File system inputs: 0
    File system outputs: 0
    Socket messages sent: 0
    Socket messages received: 0
    Signals delivered: 0
    Page size (bytes): 16384
    Exit status: 0
```
