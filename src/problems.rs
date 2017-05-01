pub mod problem1 {

    use std::collections::HashMap;

    /// bday candle warmup problem
    pub fn bday_candles(n: i32, height_string: &String) -> i32 {

        println!("n = {}", &n);

        // n = number of candles
        // height_string = string containing the heights of the candles
        let mut heights: Vec<i32> = vec![];

        for height in height_string.split(" ") {

            let height: i32 = match height.trim().parse() {

                Ok(v) => v,
                Err(_) => {
                    println!("Error");
                    break;
                }
            };

            heights.push(height);
        }

        // max is the maximum
        let mut max: i32 = -1;
        let mut max_count_vec: Vec<i32> = vec![];

        for height in &heights {

            if height > &mut max {

                max = *height;
            }
        }

        for height in &heights {

            if height == &max {

                &mut max_count_vec.push(*height);
            }
        }

        println!("The number of candles = {}", max_count_vec.len());

        return max_count_vec.len() as i32;
    }



    // using std library hashmap implementations
    pub fn bday_candles_hashmap(_: i32, height_string: &String) {

        let mut heights: Vec<i32> = vec![];
        let mut height_map: HashMap<i32, i32> = HashMap::new();
        let mut max: i32 = -1;

        for height in height_string.trim().split(" ") {

            let height: i32 = match height.trim().parse() {

                Ok(n) => n,
                _ => {
                    println!("Err");
                    return;
                }
            };

            if height > *(&mut max) {

                max = height;
            }

            (&mut heights).push(height);
        }

        for height in &heights {

            if !*(&mut height_map.contains_key(height)) {

                height_map.insert(*height, 1);
            } else {

                let prev_value = match height_map.get(height) {
                    Option::Some(n) => *n,
                    _ => 0,
                };
                height_map.insert(*height, prev_value + 1);
            }
        }

        println!("max_value = {}, max = {}",
                 &max,
                 &height_map.get(&max).expect("err"));
    }
}
