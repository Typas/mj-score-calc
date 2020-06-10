// 分數調整原則：偶然役減三番，暗手役與行為役減一番。
// 偶然役中天和地和為一組調整，其餘四種為一組調整。
// 暗手役為特殊和牌形與暗刻類，不可以鳴牌達成的牌形。
// 行為役排除偶然役，有不求人與全求人。
// 調整魔術數字的基準：使番牌的機率總和符合 10 分級距。

const MAGIC: f64 = 4.0;

fn fan4(probability: f64) -> f64 {
    const LIMIT4: f64 = 5.0;
    let n = (probability.log(MAGIC).abs()*2.0).round()/2.0;
    match n {
        n if n < LIMIT4 => n,
        _ => (n+LIMIT4).round()/2.0,
    }
}

fn score4(probability: f64) -> i32 {
    const SIZE4: usize = 13;
    const HALF4: usize = SIZE4-3;
    //                            0, .5,  1, 1.5,  2, 2.5,  3, 3.5,  4, 4.5,   5, 5.5,   6
    static PTS4: [i32; SIZE4]  = [0,  5, 10,  15, 20,  30, 40,  60, 80, 120, 160, 240, 320];
    let n = (probability.log(MAGIC).abs()*2.0).round() as usize;
    let index = match n {
        1..=HALF4 => n,
        _ => HALF4+(n-HALF4)/2,
    };
    let index = match index >= SIZE4 {
      true =>  SIZE4-1,
      false => index,
    };

    PTS4[index]
}

fn fan3(probability: f64) -> f64 {
    const LIMIT3: f64 = MAGIC;
    let n = (probability.log(MAGIC).abs()*2.0).round()/2.0;
    match n {
        n if n < LIMIT3 => n,
        _ => (n+LIMIT3).round()/2.0,
    }
}

fn score3(probability: f64) -> i32 {
    const SIZE3: usize = 11;
    const HALF3: usize = SIZE3-3;
    //                            0, .5,  1, 1.5,  2, 2.5,  3, 3.5,  4, 4.5,   5
    static PTS3: [i32; SIZE3]  = [0,  5, 10,  15, 20,  30, 40,  60, 80, 120, 160];
    let n = (probability.log(MAGIC).abs()*2.0).round() as usize;
    let index = match n {
        1..=HALF3 => n,
        _ => HALF3+(n-HALF3)/2,
    };
    let index = match index >= SIZE3 {
        true => SIZE3-1,
        false => index,
    };

    PTS3[index]
}

fn main() {
    let p = 0.01/100.;
    let h = "小四喜";
    let s = score4(p);

    let p2 = 0.003/100.;
    let h2 = "三色同刻";
    let s2 = score3(p2);

    println!("四麻役種「{}」應得 {} 分", h, s);
    println!("三麻役種「{}」應得 {} 分", h2, s2);

    let p = 0.3/100.;
    let h = "偶然役平均";
    let s = fan4(p);
    println!("四麻役種「{}」應得 {} 番", h, s);
    
    let p = 1.0/330000.;
    let h = "天和";
    let s = fan4(p);
    println!("四麻役種「{}」應得 {} 番", h, s);
    
    let p2 = 1.0/4500.;
    let h2 = "地和";
    let s2 = fan3(p2);
    println!("三麻役種「{}」應得 {} 番", h2, s2);

    let p = 0.2;
    let h = "番牌";
    let s = fan4(p);
    println!("四麻役種「{}」應得 {} 番", h, s);
    let s = score4(p);
    println!("四麻役種「{}」應得 {} 分", h, s);
}
