use criterion::{criterion_group, criterion_main, Criterion};
use std::sync::{Arc, Mutex, RwLock};
use std::rc::Rc;
use std::cell::RefCell;
use std::sync::atomic::{AtomicU32, Ordering};

#[derive(Clone)]
struct City {
    name: String,
}

impl City {
    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut city = City {
        name: String::from("Shinjuku"),
    };
    let rc_city = Rc::new(city.clone());
    let arc_city = Arc::new(city.clone());
    let rc_refcell_city = Rc::new(RefCell::new(city.clone()));
    let arc_mutex_city = Arc::new(Mutex::new(city.clone()));
    let arc_rwlock_city = Arc::new(RwLock::new(city.clone()));
    let atomic_u32 = AtomicU32::new(0);
    
    c.bench_function("access Owned", |b| b.iter(||
        city.get_name()
    ));
    
    c.bench_function("access Rc", |b| b.iter(||
        rc_city.get_name()
    ));
    
    c.bench_function("access Arc", |b| b.iter(||
        arc_city.get_name()
    ));
    
    c.bench_function("access Arc<Mutex>", |b| b.iter(|| {
        let city = arc_mutex_city.lock().unwrap();
        city.get_name();
        drop(city)
    }));
    
    c.bench_function("access Arc<RwLock>", |b| b.iter(|| {
        let city = arc_rwlock_city.read().unwrap();
        city.get_name();
        drop(city)
    }));
    
    c.bench_function("access AtomicU32", |b| b.iter(|| {
        atomic_u32.load(Ordering::Relaxed)
    }));
    
    c.bench_function("modify Owned", |b| b.iter(||
        city.name = String::from("Shibuya")
    ));
    
    c.bench_function("modify Rc<RefCell>", |b| b.iter(||
        rc_refcell_city.borrow_mut().name = String::from("Shibuya")
    ));
    
    c.bench_function("modify Arc<Mutex>", |b| b.iter(||
        arc_mutex_city.lock().unwrap().name = String::from("Shibuya")
    ));
    
    c.bench_function("modify Arc<RwLock>", |b| b.iter(||
        arc_rwlock_city.write().unwrap().name = String::from("Shibuya")
    ));
    
    c.bench_function("modify AtomicU32", |b| b.iter(|| {
        atomic_u32.store(1, Ordering::Relaxed)
    }));
}

criterion_group!{
    name = atomics;
    config = Criterion::default().sample_size(100);
    targets = criterion_benchmark
}

criterion_main!(atomics);