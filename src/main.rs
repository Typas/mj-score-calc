fn score4(probability: f64) -> i32 {
    const SIZE4: usize = 15;
    const LAST4: usize = SIZE4 - 1;
    //                           0, .5,  1, 1.5,  2, 2.5,  3, 3.5,  4, 4.5,   5, 5.5,   6, 6.5,   7
    static PTS4: [i32; SIZE4]  = [0,  5, 10,  15, 20,  30, 40,  60, 80, 120, 160, 240, 320, 400, 480];
    let n = (probability.log(4.0).abs()*2.0).round() as usize;
    let index = match n {
        1..=10 => n,
        11..=19 => 10+(n-10)/2,
        _ => LAST4,
    };

    PTS4[index]
}

fn score3(probability: f64) -> i32 {
    const SIZE3: usize = 13;
    const LAST3: usize = SIZE3 - 1;
    //                           0, .5,  1, 1.5,  2, 2.5,  3, 3.5,  4, 4.5,   5, 5.5,   6
    static PTS3: [i32; SIZE3]  = [0,  5, 10,  15, 20,  30, 40,  60, 80, 120, 160, 200, 240];
    let n = (probability.log(4.0).abs()*2.0).round() as usize;
    let index = match n {
        1..=8 => n,
        9..=17 => 8+(n-8)/2,
        _ => LAST3,
    };

    PTS3[index]
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
