pub fn array_sum(nbrs: &Vec<i32>) -> i32 {


    let sum: i32 = 0;

    let sum = nbrs.into_iter()
        .fold(sum, |acc, &x| acc + x);

    //    println!("sum after = {}", &sum);
    return sum;
}
