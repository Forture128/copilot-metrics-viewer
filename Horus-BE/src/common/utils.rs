use anyhow::Result;
use std::future::Future;

pub async fn process_in_chunks<I, O, F, Fut>(
    items: &[I],
    chunk_size: usize,
    process_item: F,
) -> Result<Vec<O>>
where
    F: Fn(&I) -> Fut + Send + Sync + Clone,
    Fut: Future<Output = octocrab::Result<octocrab::Page<O>>> + Send,
    I: Send + Sync,
    O: Send + Sync,
{
    let mut all_items = Vec::new();

    for chunk in items.chunks(chunk_size) {
        let futures: Vec<_> = chunk.iter().map(process_item.clone()).collect();
        let results = futures::future::join_all(futures).await;
        for result in results.into_iter().filter_map(|r| r.ok()) {
            all_items.extend(result.items);
        }
    }

    Ok(all_items)
}
