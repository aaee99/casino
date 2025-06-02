use rand::prelude::*;
use std::io;
// можно ставить на красное или на черное
// Баланс
// можно ставить на число
// можно крутить слоты где
// морковка семерки какашки вишня
struct Player{
    balance:i32,
    bid:i32,
}
impl Player{
    fn add_balance(&mut self,sum:i32){
        self.balance += sum;
    }
    // fn check_balance(&self) -> i32{
    //     self.balance
    // }

}
fn main() {
    println!("
        
                                                     
                            ,,                       
                            db                       
                                                     
 ,p6*bo   ,6*Yb.  ,pP*Ybd `7MM  `7MMpMMMb.  ,pW*Wq.  
6M'  OO  8)   MM  8I   `*   MM    MM    MM 6W'   `Wb 
8M        ,pm9MM  `YMMMa.   MM    MM    MM 8M     M8 
YM.    , 8M   MM  L.   I8   MM    MM    MM YA.   ,A9 
 YMbmd'  `Moo9^Yo.M9mmmP' .JMML..JMML  JMML.`Ybmd9'  
                                                     
                                                     

    ");
    let mut rng = rand::rng();
    let mut player = Player{balance:100,bid:10,};
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
                println!("ТЫ ВЫБРАЛ 1. Красное/Черное");
                println!("ТЕПЕРЬ ВЫБЕРИ ЦВЕТ");
                let i = red_and_black(&mut rng);
                // match input().trim().parse::<u32>(){
                //     Ok(n) => n,
                //     Err(_) => {
                //         println!("Ошибка");
                //         return;
                //     }
                // }
                println!("1. Красное\n2. Черное\n3. Зеленое");
                match input().as_str() {
                    "1" => {
                        println!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
                        println!("ТЫ ВЫБРАЛ КРАСНОЕ");
                        let k = [
                            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18,
                        ];
                        match k.contains(&i) {
                            true => {
                                println!("ТЫ ВЫИГРАЛ!!!!");
                                player.add_balance(player.bid);
                            }
                            false => {
                                println!("ТЫ ПРОИГРАЛ!!!");
                                player.add_balance(-player.bid);
                            }
                        }
                    }
                    "2" => {
                        println!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
                        println!("ТЫ ВЫБРАЛ ЧЕРНОЕ");
                        let k = [
                            19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36,
                        ];
                        match k.contains(&i) {
                            true => {
                                println!("ТЫ ВЫИГРАЛ!!!!")
                                
                            }
                            false => {
                                println!("ТЫ ПРОИГРАЛ!!!")
                            }
                        }
                    }
                    "3" => {
                        println!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
                        println!("ТЫ ВЫБРАЛ ЗЕЛЕНОЕ");
                        let k = [37];
                        match k.contains(&i) {
                            true => {
                                println!("ТЫ ВЫИГРАЛ!!!!")
                            }
                            false => {
                                println!("ТЫ ПРОИГРАЛ!!!")
                            }
                        }
                    }
                    &_ => continue,
                }
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
fn red_and_black(rng: &mut ThreadRng) -> u32 {
    rng.random_range(1..37)
}
fn hud(player: &Player){
        println!("Баланс: {}",player.balance);
        println!("Cтавка: {}",player.bid);
}