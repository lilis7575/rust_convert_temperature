use std::io;

fn main() {
    println!("섭씨 화씨 변환기");
    
    loop {
        println!("");
        println!("변환할 방향을 선택하세요.");
        println!("1. 섭씨 -> 화씨 (1)");
        println!("2. 화씨 -> 섭씨 (2)");
        
        // let mut direct = String::new();
        // io::stdin().read_line(&mut direct).expect("입력값 오류.");
        let direct = input_type();

        if direct.trim().eq("1") {
            // println!(">> {}", direct);
            convert_to_fahrenheit();
    
        } else if direct.trim().eq("2") {
            // println!(">> {}", direct);
            convert_to_celsius();
    
        } else {
            println!("종료합니다.");
            break;
        }
    }
}

fn convert_to_celsius() {
    println!("==== 화씨 -> 섭씨");
    println!("변환할 온도를 입력하세요. (-100 ~ 100)");
    
    // let mut target = String::new();
    // io::stdin().read_line(&mut target).expect("입력값 오류.");
    let target = input_type();
    
    let target:f64 = target.trim().parse().expect("변환오류");

    let convert = (target - 32.0) / 1.8;
    println!("화씨 : {}°F -> 섭씨 : {}°C", target, convert);
}

fn convert_to_fahrenheit() {
    println!("==== 섭씨 -> 화씨");
    println!("변환할 온도를 입력하세요. (-100 ~ 100)");

    // let mut target = String::new();
    // io::stdin().read_line(&mut target).expect("입력값 오류.");
    let target = input_type();

    let target:f64 = target.trim().parse().expect("변환오류");

    let convert = (target * 1.8) + 32.0;
    println!("섭씨 : {}°C -> 화씨 : {}°F", target, convert);
}

fn input_type() -> String {
    let mut rtn = String::new();

    io::stdin().read_line(&mut rtn).expect("입력값 오류.");

    rtn
}