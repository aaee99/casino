use colored::*;
use rand::prelude::*;
use std::io;
use std::thread;
use std::time::Duration;
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
    fn add_bid(&mut self, sum: i32) {
        self.bid += sum;
    }
}
fn main() {
    control::set_virtual_terminal(true).expect("Could not enable ANSI colors");
    println!(
        "{}",
        "
                            ,,                       
                            db                       
                                                     
 ,p6*bo   ,6*Yb.  ,pP*Ybd `7MM  `7MMpMMMb.  ,pW*Wq.  
6M'  OO  8)   MM  8I   `*   MM    MM    MM 6W'   `Wb 
8M        ,pm9MM  `YMMMa.   MM    MM    MM 8M     M8 
YM.    , 8M   MM  L.   I8   MM    MM    MM YA.   ,A9 
 YMbmd'  `Moo9^Yo.M9mmmP' .JMML..JMML  JMML.`Ybmd9'  
                                                     
                                                     "
        .bright_yellow().bold()
    );
    
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
        println!(
            "
        1. {}/{}
           2. Слоты
             3. Ставить на число
        ",
            "красное".red(),
            "черное".black()
        );
        match input().as_str() {
            "1" => {
                // println!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
                // hud(&player);
                // println!("ТЫ ВЫБРАЛ 1. Красное/Черное");

                // let i = red_and_black(&mut rng);
                // match input().trim().parse::<u32>(){
                //     Ok(n) => n,
                //     Err(_) => {
                //         println!("Ошибка");
                //         return;
                //     }
                // }
                if player.balance >= player.bid && player.bid > 0 {
                    red_and_black(&mut player);
                } else {
                    casino_error();
                    continue;
                }
            }
            "2"=>{
                // hud(&player);
                if player.balance >= player.bid && player.bid > 0 {
                    slots(&mut player);
                } else {
                    casino_error();
                    continue;
                }
            }
            "w" | "W" | "+" => {
                player.add_bid(10);
            }
            "s" | "S" | "-" => {
                player.add_bid(-10);
            }
            &_ => continue,
        };
    }
}
fn casino_error(){
    println!("У ТЕБЯ НЕТУ БАЛАНСА ЛИБО СТАВКА НЕПРАВИЛЬНАЯ");
}
fn input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Ошибка ввода");
    input.trim().to_string()
}
fn slots(player: &mut Player){
    let mut rng = rand::rng();
    let items = [
        "🍒",
        "🍒",
        "💩",
        "💩",
        "💩",
        "7️⃣",
        "🥕",
        "🥕",
        "💎",
        "💎",
        "🔔",
        "🔔",
        ];
    let mut result_items = vec!["";3];
    for i in 0..3{
        let random_item = rng.random_range(0..items.len());
        result_items[i] = items[random_item];
        
        thread::sleep(Duration::from_millis(200+i as u64*200));
        println!("{}",result_items.join(" "));
    }
    
}
fn red_and_black(player: &mut Player) {
    
    let mut rng = rand::rng();
    let result = rng.random_range(1..=37);
    let mult: f32;
    println!("ТЕПЕРЬ ВЫБЕРИ ЦВЕТ");
    println!(
        "
    1 {}
      2 {}
       3 {}
    ",
        "красное".on_red(),
        "черное".on_black(),
        "зеленое".on_green()
    );
    let win = match input().as_str() {
        "1" => {
            player.add_balance(-player.bid);
            mult = 2.0;
            (1..=18).contains(&result)
        }

        "2" => {
            player.add_balance(-player.bid);
            mult = 2.0;
            (18..=36).contains(&result)
        }
        "3" => {
            player.add_balance(-player.bid);
            mult = 20.0;
            result == 37
        }
        &_ => return,
    };

    if win {
        print_result("win");
        let sum = (player.bid as f32 * mult).round() as i32;
        player.add_balance(sum);
    } else {
        print_result("loose");
    }
}
fn hud(player: &Player) {
    println!("Баланс: {}", player.balance.to_string().bright_yellow());
    println!("Cтавка: {}", player.bid.to_string().bright_blue());
    println!(
        "
        {} Увеличить ставку [+]
            {}  Уменьшить ставку [-]
    ",
        "[W]".bright_green(), "[S]".bright_red()
    )
}
fn print_result(w: &'static str) {
    let result = match w {
        "win" => r#"
 ######  ##   ## ##   ## ####### ######     ####   ##### 
 ##   ## ##   ## ##  ### ##      ##   ##   ## ##  ##  ## 
 ##   ## ##   ## ##  ### ##      ##   ##  ##  ##  ##  ## 
 ######  ####  # ## # ## ##      ######  ##   ##  ##  ## 
 ##   ## ##  # # ###  ## ##      ##      #######  ##  ## 
 ##   ## ##  # # ##   ## ##      ##      ##   ##  ##  ## 
 ######  ####  # ##   ## ##      ##      ##   ## ##   ## 
        "#
        .green()
        .bold(),
        "loose" => r#"
 ####### ######   #####  ##   ## ####### ######     ####   ##### 
 ##   ## ##   ## ##   ## ##  ### ##      ##   ##   ## ##  ##  ## 
 ##   ## ##   ## ##   ## ##  ### ##      ##   ##  ##  ##  ##  ## 
 ##   ## ######  ##   ## ## # ## ##      ######  ##   ##  ##  ## 
 ##   ## ##      ##   ## ###  ## ##      ##      #######  ##  ## 
 ##   ## ##      ##   ## ##   ## ##      ##      ##   ##  ##  ## 
 ##   ## ##       #####  ##   ## ##      ##      ##   ## ##   ## 
        "#
        .red()
        .bold(),
        _ => return,
    };
    println!("{result}");
}
