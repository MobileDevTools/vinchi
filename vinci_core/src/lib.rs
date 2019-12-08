mod errors;
use std::sync::atomic::{Ordering, AtomicUsize, AtomicI32};
use std::sync::{Arc, Mutex};
use std::fs::read_dir;

pub use {
    errors::{ CompileError },
    futures::*,
    async_std::task::*,
};

//todo check error if no files permissions!!!
pub async fn async_walk_and_find_all_files(dir: &str) -> Result<Vec<String>, ()> {
    let mut current_path = std::path::Path::new(dir);
    if !current_path.exists() && current_path.is_file() { return Err(()); };
    let mut read_dir: std::fs::ReadDir = read_dir(current_path).unwrap();
    let vec = Arc::new(Mutex::new(vec![read_dir]));
    let count = Arc::new(AtomicI32::new(0));
    let parent_task_count = Arc::new(AtomicUsize::new(0));
    let task_count = Arc::new(AtomicUsize::new(0));
    let mut counter = 0;
    loop {
        if task_count.load(Ordering::SeqCst) > 133 /*magic number*/{
            continue;
        }
        if let Some(mut root_dir) = vec.lock().unwrap().pop() {
            let next = vec.clone();
            let count = count.clone();
            let task_count = task_count.clone();
            task_count.fetch_add(1, Ordering::SeqCst);
            async_std::task::spawn(async move {
                while let Some(Ok(mut dir)) = root_dir.next() {
                    let next_t = next.clone();
                    let task_count = task_count.clone();
                    count.fetch_add(1, Ordering::SeqCst);
                    loop {
                        let f_type = dir.file_type();
                        match f_type {
                            Ok(f) if f.is_symlink() => { break; }
                            Ok(f) if f.is_dir() => {
                                task_count.fetch_add(1, Ordering::SeqCst);
                                async_std::task::spawn(async move {
                                    loop {
                                        let read = std::fs::read_dir(dir.path());
                                        if let Err(dir) = read {
                                            continue;
                                        } else if let Ok(dir) = read {
                                            let mut vec = next_t.lock().unwrap();
                                            vec.push(dir);
                                            break;
                                        }
                                    }
                                    task_count.fetch_sub(1, Ordering::SeqCst);
                                });
                                break;
                            }
                            Ok(_) => { break; }
                            Err(_) => { continue; }
                        }
                    }
                }
                task_count.fetch_sub(1, Ordering::SeqCst);
            });
        } else if task_count.load(Ordering::SeqCst) == 0 {
            break;
        }
    }
    let c = count.load(Ordering::SeqCst);
    Ok(vec![c.to_string()])
}