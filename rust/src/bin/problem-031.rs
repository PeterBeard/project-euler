// Copyright 2016 Peter Beard
// Distributed under the GNU GPL v2. For full terms, see the LICENSE file.
//
// In England the currency is made up of pound, £, and pence, p, and there are
// eight coins in general circulation:
// 
//     1p, 2p, 5p, 10p, 20p, 50p, £1 (100p) and £2 (200p).
// 
// It is possible to make £2 in the following way:
// 
//     1×£1 + 1×50p + 2×20p + 1×5p + 1×2p + 3×1p
// 
// How many different ways can £2 be made using any number of coins?
//

fn main() {
    // Values of coins in pence
    const POUND: u32 = 100;
    const FIFTY_P: u32 = 50;
    const TWENTY_P: u32 = 20;
    const TEN_P: u32 = 10;
    const FIVE_P: u32 = 5;
    const TWO_P: u32 = 2;
    const ONE_P: u32 = 1;
    const GOAL: u32 = 200;

    // Obviously 1x£2 coin works, so we'll include that from the start.
    let mut count = 1;

    for pounds in 0..GOAL/POUND + 1 {
        let value = pounds * POUND;

        for fifty in 0..(GOAL - value)/FIFTY_P + 1 {
            let value = value + fifty * FIFTY_P;

            for twenty in 0..(GOAL - value)/TWENTY_P + 1 {
                let value = value + twenty * TWENTY_P;

                for ten in 0..(GOAL - value)/TEN_P + 1 {
                    let value = value + ten * TEN_P;

                    for five in 0..(GOAL - value)/FIVE_P + 1 {
                        let value = value + five * FIVE_P;

                        for two in 0..(GOAL - value)/TWO_P + 1 {
                            let value = value + two * TWO_P;

                            for p in 0..(GOAL - value)/ONE_P + 1 {
                                let value = value + p * ONE_P;
                                if value == 200 {
                                    count += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    println!("There are {} ways to make £2", count);
}
