# atomics-performance

Provide a rough idea of the cost of common atomic operations in comparison to non-atomic operations.

## Results

```
access plain            time:   [432.22 ps 432.55 ps 432.84 ps]
access Rc               time:   [650.70 ps 653.99 ps 657.40 ps]
access Arc              time:   [547.54 ps 550.43 ps 553.66 ps]
access Arc<Mutex>       time:   [3.3647 ns 3.3693 ns 3.3746 ns]
access Arc<RwLock>      time:   [4.0301 ns 4.0360 ns 4.0431 ns]
access AtomicU32        time:   [251.47 ps 251.83 ps 252.22 ps]
modify plain            time:   [7.6298 ns 7.7375 ns 7.8371 ns]
modify Rc<RefCell>      time:   [8.2859 ns 8.3512 ns 8.4087 ns]
modify Arc<Mutex>       time:   [8.8253 ns 8.8465 ns 8.8759 ns]
modify Arc<RwLock>      time:   [9.0563 ns 9.0887 ns 9.1402 ns]
modify AtomicU32        time:   [108.21 ps 108.31 ps 108.41 ps]
```