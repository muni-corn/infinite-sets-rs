mod infinite_set;
mod sets;

use infinite_set::InfiniteSet;

fn main() {
    {
        let primes = sets::InfinitePrimes::new();
        let odds = sets::InfiniteOdds::new();

        let intersection: Vec<u128> = primes.intersect(odds).take(10).collect();

        println!("{:?}", intersection);
    }

    {
        let evens = sets::InfiniteEvens::new();
        let odds = sets::InfiniteOdds::new();

        let union: Vec<u128> = evens.union(odds).take(10).collect();

        println!("{:?}", union);
    }

    // {
    //     let evens = sets::InfiniteEvens::new();
    //     let odds = sets::InfiniteOdds::new();

    //     // we would expect this operation to last forever
    //     // i tested it; and it does!
    //     let never_intersect: Vec<u128> = evens.intersect(odds).take(10).collect();

    //     println!("{:?}", never_intersect);
    // }
    
    {
        let powers_of_two = sets::InfiniteTwoPowers::new();
        let odds = sets::InfiniteOdds::new();

        let union: Vec<u128> = powers_of_two.union(odds).take(30).collect();

        println!("{:?}", union);
    }
}
