use std::collections::BTreeSet;

use cosdata::storage::inverted_index::InvertedIndex;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::{rngs::StdRng, Rng, SeedableRng};

// Function to generate multiple random sparse vectors
fn generate_random_sparse_vectors(
    num_records: usize,
    max_index: usize,
    min_nonzero: usize,
    max_nonzero: usize,
) -> Vec<Vec<f32>> {
    let mut rng = StdRng::seed_from_u64(2024);
    let mut records: Vec<Vec<f32>> = vec![];

    for _ in 0..num_records {
        let num_nonzero: usize = rng.gen_range(min_nonzero..=max_nonzero);
        let mut record = vec![0.0; max_index];

        // BTreeSet is used to store unique indices of nonzero values in sorted order
        let mut unique_indices = BTreeSet::new();
        while unique_indices.len() < num_nonzero as usize {
            // Generate a random index
            let index = rng.gen_range(0..max_index);
            unique_indices.insert(index);
        }

        // Generate random values for the nonzero indices
        for index in unique_indices {
            record[index] = rng.gen();
        }

        records.push(record);
    }

    records
}

fn benchmark_inserts(c: &mut Criterion) {
    let max_index = 10;
    let num_vectors = 100;
    let min_nonzero = 1;
    let max_nonzero = 4;

    // Generate random sparse vectors
    let records = generate_random_sparse_vectors(num_vectors, max_index, min_nonzero, max_nonzero);

    // Create new inverted index
    let inverted_index: InvertedIndex<f32> = InvertedIndex::new();

    let test_name = format!(
        "Insert {} sparse vectors of dimensionality {}",
        num_vectors, max_index
    );
    c.bench_with_input(
        BenchmarkId::new(test_name, num_vectors),
        &records,
        |b, records| {
            b.iter(|| {
                for (id, record) in records.iter().enumerate() {
                    let _ = inverted_index.add_sparse_vector(record.to_vec(), id as u32);
                }
            });
        },
    );
}

criterion_group!(benches, benchmark_inserts);
criterion_main!(benches);