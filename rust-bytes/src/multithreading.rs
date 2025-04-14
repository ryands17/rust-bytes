use rayon::prelude::*;
use tokio::sync::oneshot;

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
