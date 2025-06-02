fn main() {
    println!("Hello, world!");
}

#[test]
fn hello_test() {
    println!("Hello World")
}

#[test]
fn test_variable() {
    let name = "Rizqi";
    let greet = "Hello";

    println!("{} {}", greet, name)
}

#[test]
fn test_mutable() {
    let mut name = "Rizqi";
    let greet = "Hello";

    println!("{} {}", greet, name);

    name = "Seira";
    println!("{} {}", greet, name)
}

#[test]
fn shadowing() {
    let name = "Rizqi";
    let greet = "Hello";

    println!("{} {}", greet, name);

    let name = 10;
    println!("{} {}", greet, name)
}

#[test]
fn explciit() {
    let age: i32 = 20;
    println!("{}", age)
}

#[test]
fn number() {
    let a: i8 = 10;
    println!("{}", a);

    let b: f32 = 10.5;
    println!("{}", b)
}

#[test]
fn number_convertion() {
    let a: i8 = 10;
    println!("{}", a);

    let b: i16 = a as i16;
    println!("{}", b);

    let c: i32 = b as i32;
    println!("{}", c);

    let d: i64 = 1000000000;
    let e: i8 = d as i8;
    println!("{}", e)
}

#[test]
fn numeric() {
    let a = 10;
    let b = 32;

    let c = a * b;
    println!("Hasil dari {} + {} = {}", a, b, c)
}

#[test]
fn augmented_assignment() {
    let mut a = 10;

    a += 10;

    println!("Hasilnya {}", a)
}

#[test]
fn boolean() {
    let a = true;
    let b: bool = false;

    println!("{} {}", a, b)
}

#[test]
fn comparison() {
    let a = 10;
    let b = 20;
    let result = a > b;

    println!("{}", result);
}

#[test]
fn boolean_operator() {
    let absen = 75;
    let nilai_akhir = 80;

    let cek_absen = absen >= 70;
    let cek_nilai_akhir = nilai_akhir >= 70;

    let result = if cek_absen && cek_nilai_akhir {
        "Lulus"
    } else {
        "Tidak Lulus"
    };

    println!("{}", result)
}

#[test]
fn char_type() {
    let char1 = 'a';
    let char2 = 'b';

    println!("{} {}", char1, char2)
}

#[test]
fn tuple() {
    let mut data: (i32, &str, bool) = (10, "Hello", true);

    println!("{:?}", data);

    //    let a = data.0;
    //    let b = data.1;
    //    let c = data.2;

    let (a, b, _) = data;

    println!("Data Index 0 adalah {}", a);
    println!("Data Index 1 adalah {}", b);
    //    println!("Data Index 2 adalah {}", c);

    data.0 = 12;
    data.1 = "Seira";
    data.2 = false;

    println!("{:?}", data);
}

// fn unit(){
//     println!("Hello");
// }

// #[test]
// fn test_unit(){
//     let result = unit();
//     println!("{:?}", result)
// }

#[test]
fn array() {
    let mut array = [1, 2, 3, 4, 5];
    println!("{:?}", array);

    let a = array[0];
    let b = array[1];

    println!("{} {}", a, b);

    array[0] = 12;
    array[1] = 14;

    println!("{:?}", array);
    println!("{:?}", array.len() - 1);
}

#[test]
fn two_dimensional_array() {
    let matrix = [[1, 2], [2, 3]];

    println!("{:?}", matrix);
    println!("{:?}", matrix[0]);
    println!("{:?}", matrix[1]);
    println!("{:?}", matrix[0][0]);
    println!("{:?}", matrix[1][1]);
    println!("{:?}", matrix[1][0]);
    println!("{:?}", matrix[1][1]);
}

#[test]
fn constant() {
    const MAXIMUM: i32 = 20000;

    println!("{}", MAXIMUM)
}

#[test]
fn variable_scope() {
    let a = 10;

    {
        println!("{}", a);

        let b = 20;

        print!("{}", b)
    }

    // println!("{}", b) ===> Tidak bisa di gunakan karena ada di luar scope variable yang di definisikan
}

#[test]
fn stack_heap() {
    function_a();
    function_b();
}

fn function_a() {
    let a = 10;
    let b = String::from("Seira");

    println!("{} {}", a, b)
}

fn function_b() {
    let a = 12;
    let b = String::from("Nava");

    println!("{} {}", a, b)
}

#[test]
fn string() {
    let name = " Seira ";
    let trim = name.trim();

    println!("{} {}", name, trim);
}

#[test]
fn string_type() {
    let mut name: String = String::from("Rizqi");
    println!("{}", name);

    name.push_str(" Sabilla");
    println!("{}", name);

    let nava = name.replace("Rizqi", "Nava");
    println!("{}", nava)
}

#[test]
fn ownership_rules() {
    let a = 10;

    {
        let b = 20;
        println!("{}", b)
    }
    println!("{}", a)
}

#[test]
fn data_copy() {
    let a = 10;
    let mut b = a;

    b = 20;

    println!("{}", a);
    println!("{}", b)
}

#[test]
fn ownership_movement() {
    let name1 = String::from("Rizqi");
    println!("{}", name1);

    let name2 = name1;
    println!("{}", name2);
}

#[test]
fn clone() {
    let name1 = String::from("Rizqi");
    println!("{}", name1);

    let name2 = name1.clone();
    println!("{}", name2);
}
