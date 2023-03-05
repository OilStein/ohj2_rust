
fn main() {
    println!("{}", keskiarvo(vec![12, 0, 42, 14, 99, 12, 55], 0, 99));
    neliojuuret();
    
}

// Tehtävä B1, Karkkien heittely
fn keskiarvo(taulukko: Vec<i32>, vialliset: i32, lopetus: i32) -> f32{
    let mut ka: f32 = 0.0;
    let mut jako: i32 = 0;
    for num in taulukko {
        if num >= lopetus {
            break;
        }
        if num > vialliset {
            ka += num as f32;
            jako += 1;
        }
    }

    if jako != 0 {
        ka = ka / jako as f32
    }
    else {
        return vialliset as f32
    }

    ka
}

fn neliojuuret() {
    let mut i: i32 = 0;
    while i <= 1000{
        if f32::sqrt(i as f32) % 1.0 == 0.0{
            print!("{}, ", i as i32);
        }
        i += 1;
    }
}
