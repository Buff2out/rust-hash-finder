use rayon::prelude::*;
use sha2::{Digest, Sha256};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use tracing::{debug, info, warn, instrument};

#[cfg(feature = "crossbeam")]
use crossbeam_channel::bounded;

#[instrument(skip_all, fields(num = %num))]
pub fn compute_hash(num: u64) -> String {
    let mut hasher = Sha256::new();
    hasher.update(num.to_string().as_bytes());
    let result = hasher.finalize();
    format!("{:x}", result)
}

pub fn hash_ends_with_zeros(hash: &str, zeros: usize) -> bool {
    if zeros == 0 || zeros > hash.len() {
        return false;
    }
    hash.ends_with(&"0".repeat(zeros))
}

#[cfg(feature = "atomics")]
#[instrument(skip_all, fields(zeros = %zeros, max_results = %max_results))]
pub fn find_hashes(zeros: usize, max_results: usize) -> Vec<(u64, String)> {
    info!("Starting hash search with atomics implementation");
    
    let found_count = Arc::new(AtomicUsize::new(0));
    let found_count_clone = Arc::clone(&found_count);
    let results: Arc<std::sync::Mutex<Vec<(u64, String)>>> = Arc::new(std::sync::Mutex::new(Vec::new()));
    let results_clone = Arc::clone(&results);
    let suffix = "0".repeat(zeros);
    
    debug!("Searching for hashes ending with {} zeros", zeros);
    
    (1u64..)
        .par_bridge()
        .find_any(|&num| {
            if found_count_clone.load(Ordering::Relaxed) >= max_results {
                return true;
            }
            
            let hash = compute_hash(num);
            
            if hash.ends_with(&suffix) {
                let current = found_count_clone.fetch_add(1, Ordering::SeqCst);
                
                if current < max_results {
                    debug!("Found hash: num={}, hash={}", num, hash);
                    results_clone.lock().unwrap().push((num, hash));
                }
                
                if current + 1 >= max_results {
                    info!("Reached target of {} results", max_results);
                    return true;
                }
            }
            
            false
        });
    
    let results = Arc::try_unwrap(results).unwrap().into_inner().unwrap();
    info!("Search completed, found {} results", results.len());
    results
}

#[cfg(feature = "crossbeam")]
#[instrument(skip_all, fields(zeros = %zeros, max_results = %max_results))]
pub fn find_hashes(zeros: usize, max_results: usize) -> Vec<(u64, String)> {
    info!("Starting hash search with crossbeam-channel implementation");
    
    let (tx, rx) = bounded::<(u64, String)>(100);
    let found_count = Arc::new(AtomicUsize::new(0));
    let found_count_clone = Arc::clone(&found_count);
    let suffix = "0".repeat(zeros);
    
    debug!("Searching for hashes ending with {} zeros", zeros);
    
    let consumer = std::thread::spawn(move || {
        let mut results = Vec::new();
        for (num, hash) in rx {
            debug!("Received hash: num={}, hash={}", num, hash);
            results.push((num, hash));
        }
        results
    });
    
    (1u64..)
        .par_bridge()
        .find_any(|&num| {
            if found_count_clone.load(Ordering::Relaxed) >= max_results {
                return true;
            }
            
            let hash = compute_hash(num);
            
            if hash.ends_with(&suffix) {
                let current = found_count_clone.fetch_add(1, Ordering::SeqCst);
                
                if current < max_results {
                    debug!("Found hash: num={}, hash={}", num, hash);
                    let _ = tx.send((num, hash));
                }
                
                if current + 1 >= max_results {
                    info!("Reached target of {} results", max_results);
                    return true;
                }
            }
            
            false
        });
    
    drop(tx);
    let results = consumer.join().unwrap();
    info!("Search completed, found {} results", results.len());
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_hash_known_values() {
        let hash1 = compute_hash(1);
        assert_eq!(hash1.len(), 64);
        
        let hash1_again = compute_hash(1);
        assert_eq!(hash1, hash1_again);
    }

    #[test]
    fn test_hash_ends_with_zeros() {
        assert!(hash_ends_with_zeros("abc000", 3));
        assert!(!hash_ends_with_zeros("abc001", 3));
        assert!(!hash_ends_with_zeros("", 1));
    }

    #[test]
    fn test_find_hashes_count() {
        let results = find_hashes(3, 2);
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_find_hashes_validity() {
        let results = find_hashes(3, 1);
        assert_eq!(results.len(), 1);
        assert!(results[0].1.ends_with("000"));
    }
}
