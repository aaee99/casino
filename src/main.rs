use rand::prelude::*;
use std::io;
// можно ставить на красное или на черное
// Баланс
// можно ставить на число
// можно крутить слоты где
// морковка семерки какашки вишня
struct Player {
    balance: i32,
    bid: i32,
}
impl Player {
    fn add_balance(&mut self, sum: i32) {
        self.balance += sum;
    }
    // fn check_balance(&self) -> i32{
    //     self.balance
    // }
}
fn main() {
    println!(
        "
        
                                                     
                            ,,                       
                            db                       
                                                     
 ,p6*bo   ,6*Yb.  ,pP*Ybd `7MM  `7MMpMMMb.  ,pW*Wq.  
6M'  OO  8)   MM  8I   `*   MM    MM    MM 6W'   `Wb 
8M        ,pm9MM  `YMMMa.   MM    MM    MM 8M     M8 
YM.    , 8M   MM  L.   I8   MM    MM    MM YA.   ,A9 
 YMbmd'  `Moo9^Yo.M9mmmP' .JMML..JMML  JMML.`Ybmd9'  
                                                     
                                                     

    "
    );
    let mut rng = rand::rng();
    let mut player = Player {
        balance: 100,
        bid: 10,
    };
    // loop {
    //
    //     // let n = random(&mut rng);
    //     // println!("{n}");
    // };

    loop {
        // println!("\n\n\n\n\n\n\n");

        hud(&player);
        println!("во что играть");
        println!("1. Красное/Черное");
        println!("2. Слоты");
        println!("3. Ставить на число");
        match input().as_str() {
            "1" => {
                // println!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
                hud(&player);
                // println!("ТЫ ВЫБРАЛ 1. Красное/Черное");

                // let i = red_and_black(&mut rng);
                // match input().trim().parse::<u32>(){
                //     Ok(n) => n,
                //     Err(_) => {
                //         println!("Ошибка");
                //         return;
                //     }
                // }
                red_and_black(&mut rng, &mut player);
            }
            &_ => continue,
        };
    }
}
fn random(rng: &mut ThreadRng) -> u32 {
    rng.random_range(1..10000)
}
fn input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Ошибка ввода");
    input.trim().to_string()
}
fn red_and_black(rng: &mut ThreadRng, player: &mut Player) {
    let result = rng.random_range(1..=37);
    let mut mult: f32 =1.0;
    println!("ТЕПЕРЬ ВЫБЕРИ ЦВЕТ");
    println!("1. Красное\n2. Черное\n3. Зеленое");
    let win = match input().as_str() {
        "1" => {
            player.add_balance(-player.bid);
            println!("ТЫ ВЫБРАЛ КРАСНОЕ");
            mult=1.0;
            (1..=18).contains(&result)
        },

        "2" => {
            player.add_balance(-player.bid);
            println!("ТЫ ВЫБРАЛ ЧЕРНОЕ");
            mult=1.0;
            (18..=36).contains(&result)
        },
        "3" => {
            player.add_balance(-player.bid);
            println!("ТЫ ВЫБРАЛ ЗЕЛЕНОЕ");
            mult=20.0;
            result == 37
        },
        &_ => return,
    };

    if win {
        println!("ТЫ ВЫИГРАЛ!!!!");
        let sum = (player.bid as f32 * mult).round() as i32;
        player.add_balance(sum);
    } else {
        println!("ТЫ ПРОИГРАЛ!!!");
    }
}
fn hud(player: &Player) {
    println!("Баланс: {}", player.balance);
    println!("Cтавка: {}", player.bid);
}
