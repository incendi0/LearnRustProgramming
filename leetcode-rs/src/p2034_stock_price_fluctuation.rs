use std::{
    cmp::max,
    collections::{BTreeMap, HashMap},
};

struct StockPrice {
    max_timestamp: i32,
    timestamp_price_map: HashMap<i32, i32>,
    price_count: BTreeMap<i32, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl StockPrice {
    fn new() -> Self {
        StockPrice {
            max_timestamp: 0,
            timestamp_price_map: HashMap::new(),
            price_count: BTreeMap::new(),
        }
    }

    fn update(&mut self, timestamp: i32, price: i32) {
        self.max_timestamp = max(self.max_timestamp, timestamp);
        if self.timestamp_price_map.contains_key(&timestamp) {
            let older_price = self.timestamp_price_map[&timestamp];
            let older_count = self.price_count.entry(older_price).or_insert(0);
            *older_count -= 1;
            if *older_count == 0 {
                self.price_count.remove(&older_price);
            }
        }
        self.timestamp_price_map.insert(timestamp, price);
        *self.price_count.entry(price).or_insert(0) += 1;
    }

    fn current(&self) -> i32 {
        self.timestamp_price_map[&self.max_timestamp]
    }

    fn maximum(&self) -> i32 {
        *self.price_count.iter().next_back().unwrap().0
    }

    fn minimum(&self) -> i32 {
        *self.price_count.iter().next().unwrap().0
    }
}
