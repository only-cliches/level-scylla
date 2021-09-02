use bloom::BloomFilter;

fn main() {
    // 16,384 items, 0.005 false positive, 16 byte bloom filter
    let mut filter:BloomFilter = BloomFilter::with_rate(0.005, 2^14); 
    println!("{:?}", filter.num_bits() / 8);
}
