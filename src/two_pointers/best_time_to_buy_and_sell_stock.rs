pub fn max_profit(prices: Vec<i32>) -> i32 {
    // everytime sees a new lower price, record it
    // update the biggest profit with the curr value and the lowest price
    let mut lowest = i32::MAX;
    let mut max = 0;
    for price in prices {
        lowest = lowest.min(price);
        let profit = price - lowest;
        max = max.max(profit);
    }
    max
}
