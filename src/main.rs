fn score4(probability: f64) -> i32 {
    const SIZE: usize = 19;
    const LAST: usize = SIZE - 1;
    //                       0, .5,  1, 1.5,  2, 2.5,  3, 3.5,  4, 4.5,   5, 5.5,   6, 6.5,   7, 7.5,   8, 8.5,   9
    let pts: [i32; SIZE]  = [0,  5, 10,  10, 20,  30, 40,  60, 80, 120, 160, 160, 240, 240, 320, 320, 400, 400, 480];
    let orign = (probability.log(4.0).abs()*2.0).round() as usize;
    let n = match orign {
        1..=LAST => orign,
        _ => LAST,
    };
    
    let pt = pts[n];
    pt
}

fn score3(probability: f64) -> i32 {
    const SIZE: usize = 17;
    const LAST: usize = SIZE - 1;
    //                       0, .5,  1, 1.5,  2, 2.5,  3, 3.5,  4, 4.5,   5, 5.5,   6, 6.5,   7, 7.5,   8
    let pts: [i32; SIZE]  = [0,  5, 10,  10, 20,  30, 40,  60, 80,  80, 120, 120, 160, 160, 200, 200, 240];
    let orign = (probability.log(4.0).abs()*2.0).round() as usize;
    let n = match orign {
        1..=LAST => orign,
        _ => LAST,
    };
    
    let pt = pts[n];
    pt
}

fn main() {
    let p = 0.00931/100.;
    let h = "小四喜";
    let s = score4(p);

    let p2 = 0.00026/100.;
    let h2 = "九蓮寶燈";
    let s2 = score3(p2);

    println!("四麻役種「{}」應得 {} 分", h, s);
    println!("三麻役種「{}」應得 {} 分", h2, s2);
}
