use rayon::prelude::*;
use tokio::sync::oneshot;

/// Computes the sum of a slice of numbers in parallel using Rayon.
///
/// This function spawns a parallel computation task to calculate the sum of all
/// numbers in the provided slice. It uses Rayon's parallel iterator to perform
/// the summation efficiently across multiple threads.
///
/// # Parameters
///
/// * `nums` - A slice of unsigned integers to sum
///
/// # Returns
///
/// The sum of all numbers in the input slice, or 0 if the parallel task fails.
///
/// # Implementation Details
///
/// This function combines Rayon's parallel processing with async/await pattern by:
/// 1. Creating a oneshot channel for receiving the result
/// 2. Spawning a Rayon task to compute the sum in parallel
/// 3. Awaiting the result asynchronously
pub(crate) async fn parallel_sum(nums: &[usize]) -> usize {
  let (tx, rx) = oneshot::channel();

  rayon::scope(|s| {
    s.spawn(|_| {
      let sum = nums.par_iter().sum::<usize>();
      tx.send(sum).unwrap();
    });
  });

  rx.await.unwrap_or(0)
}
