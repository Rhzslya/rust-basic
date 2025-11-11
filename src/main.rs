mod first;
mod model;
mod second;
mod third;

use first::say_hello;
use second::say_hello as say_hello_second;

#[test]
fn test_use() {
    say_hello();
    say_hello_second();
    first::second::third::say_hello();
}

#[test]
fn test_module() {
    let user: model::User = model::User {
        first_name: String::from("Rizqi"),
        last_name: String::from("Sabilla"),
        age: 20,
        email: String::from("rizqisabilla@gmail.com"),
    };

    user.say_hello("Seira");
}

use std::result;

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

    b += 10;

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

#[test]
fn if_expression() {
    let value = 8;
    let result = if value > 10 { "Lulus" } else { "Tidak Lulus" };

    println!("{}", result)
}

#[test]
fn loop_expression() {
    let mut counter = 0;
    loop {
        counter += 1;

        if counter > 10 {
            break;
        } else if counter % 2 == 0 {
            continue;
        }

        println!("Counter : {}", counter);
    }
}

#[test]
fn loop_expression_with_index() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter > 10 {
            break counter * 2;
        }
    };

    println!("Counter : {}", result)
}

#[test]
fn loop_label() {
    let mut number = 1;
    'outer: loop {
        let mut i = 1;
        loop {
            if number > 10 {
                break 'outer;
            }
            println!("{} x {} = {}", number, i, number * i);
            i += 1;

            if i > 10 {
                break;
            }
        }
        number += 1;
    }
}

#[test]
fn while_loop() {
    let mut counter = 0;
    while counter <= 10 {
        if counter % 2 == 0 {
            println!("Counter : {}", counter);
        }
        counter += 1;
    }
}

#[test]
fn array_iteration() {
    let array = ["Rizqi", "Seira", "Nava"];
    let mut index = 0;

    while index < array.len() {
        println!("Value : {}", array[index]);
        index += 1;
    }
}

#[test]
fn array_iteration_for_loop() {
    let array = ["Rizqi", "Seira", "Nava"];

    for valuer in array {
        println!("Value : {}", valuer);
    }
}

#[test]
fn range() {
    let array = ["Rizqi", "Seira", "Nava"];

    for i in 0..3 {
        println!("Value : {}", array[i]);
    }
}

#[test]
fn range_inclusive() {
    let array = ["Rizqi", "Seira", "Nava"];
    let range = 0..=2;

    println!("Value : {}", range.start());
    println!("Value : {}", range.end());

    for i in range {
        println!("Value : {}", array[i]);
    }
}

// fn say_hello(name: &str) {
//     println!("Hello {}", name);
// }

// #[test]
// fn test_say_hello() {
//     say_hello("Seira");
// }

fn factorial(n: i32) -> i32 {
    if n < 0 {
        return 0;
    }

    let mut result = 1;

    for i in 1..=n {
        result *= i;
    }

    result
}

#[test]
fn factorial_test() {
    let result = factorial(5);
    println!("{}", result);

    let result = factorial(-5);
    println!("{}", result)
}

fn text_print(value: String, times: u32) {
    if times == 0 {
        return;
    } else {
        println!("{}", value);
    }

    text_print(value, times - 1);
}

#[test]
fn text_print_test() {
    text_print(String::from("Hello"), 5);
}

fn factorial_recursive(n: u32) -> u32 {
    if n <= 1 {
        return 1;
    }

    n * factorial_recursive(n - 1)
}

#[test]
fn factorial_recursive_test() {
    let result = factorial_recursive(5);
    println!("{}", result);
}

fn print_number(number: i32) {
    println!("Number {}", number)
}

fn hi(name: String) {
    println!("Hi {}", name)
}

#[test]
fn test_hi() {
    let number = 10;
    print_number(number); //print_number(10);
    println!("{}", number);

    let name = String::from("Rizqi");
    hi(name);
    // println!("{}", name) Tidak bisa digunakan karena ownership sudah pindah
}

fn full_name(first_name: &String, last_name: &String) -> String {
    format!("{} {}", first_name, last_name)
}

fn full_name_tuple(first_name: String, last_name: String) -> (String, String, String) {
    let full_name = format!("{} {}", first_name, last_name);

    (first_name, last_name, full_name)
}

#[test]
fn test_fullname() {
    let first_name = String::from("Rizqi");
    let last_name = String::from("Sabillsa");

    let fullname = full_name(&first_name, &last_name);
    println!("{}", fullname);
    println!("{}", first_name);
    println!("{}", last_name);
    // println!("{} {}", first_name, last_name) //Tidak bisa digunakan karena ownership sudah pindah
}

#[test]
fn test_fullname_tuple() {
    let first_name = String::from("Rizqi");
    let last_name = String::from("Sabilla");

    let (first_name, last_name, fullname) = full_name_tuple(first_name, last_name);
    println!("{}", fullname);
    println!("{}", first_name);
    println!("{}", last_name);
}

fn change_value(value: &mut String) {
    value.push_str(" Sabilla");
}

#[test]
fn test_change_value() {
    let mut value = String::from("Rizqi");

    let value_borrowed = &mut value;

    //Tidak Bisa dilakukan
    // let value_borrowed1 = &mut value;
    // let value_borrowed2 = &mut value;

    // change_value(value_borrowed1);
    // change_value(value_borrowed2);

    //Ini Bisa dilakukan
    // change_value(&mut value);
    // change_value(&mut value);
    // change_value(&mut value);

    change_value(value_borrowed);
    change_value(value_borrowed);
    change_value(value_borrowed);
    println!("{}", value)
}

fn get_full_name(first_name: &String, last_name: &String) -> String {
    let name = format!("{} {}", first_name, last_name);
    return name;
}

#[test]
fn test_get_fullname() {
    let first_name = String::from("Rizqi");
    let last_name = String::from("Sabilla");

    let fullname = get_full_name(&first_name, &last_name);
    println!("{}", fullname);
    println!("{}", first_name);
    println!("{}", last_name);
    // println!("{} {}", first_name, last_name) //Tidak bisa digunakan karena ownership sudah pindah
}

#[test]
fn slice_reference() {
    let array: &[i32; 10] = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let slice1: &[i32] = &array[..];
    println!("{:?}", slice1);

    let slice2: &[i32] = &array[0..5];
    println!("{:?}", slice2);

    let slice3: &[i32] = &array[5..];
    println!("{:?}", slice3);

    let slice4 = slice3;
    println!("{:?}", slice4);
}

#[test]
fn string_slice() {
    let name = String::from("Rizqi Sabilla");

    let first_name: &str = &name[..5];
    println!("{}", first_name);
    println!("{}", first_name.len());

    let last_name: &str = &name[6..];
    println!("{}", last_name);
}

struct Person {
    first_name: String,
    middle_name: String,
    last_name: String,
    age: u8,
}

impl Person {
    fn say_hello(&self, name: &str) {
        println!("Hello {}, my name is {}", name, self.middle_name);
    }
}

fn print_person(person: &Person) {
    println!(
        "First Name: {}, Last Name: {}, Age: {}",
        person.first_name, person.last_name, person.age
    );
}

#[test]
fn struct_person() {
    let first_name = String::from("Rizqi");
    let middle_name = String::from("Aulia");
    let last_name = String::from("Sabilla");

    let person = Person {
        first_name,
        middle_name,
        last_name,
        age: 20,
    };

    // println!("{}", first_name); //Tidak bisa digunakan karena ownership sudah pindah ke struct Person
    print_person(&person);

    let person2 = Person {
        first_name: person.first_name.clone(),
        middle_name: person.middle_name.clone(),
        last_name: person.last_name.clone(),

        ..person
    };

    print_person(&person2);
    print_person(&person);
}

struct GeoPoint(f64, f64);

impl GeoPoint {
    fn new(long: f64, lat: f64) -> GeoPoint {
        GeoPoint(long, lat)
    }
}

#[test]
fn test_associated_function() {
    let geo_point = GeoPoint::new(1.234, 5.678);
    println!("{}", geo_point.0);
    println!("{}", geo_point.1);
}

#[test]
fn test_tuple_struct() {
    let geo_point = GeoPoint(1.234, 5.678);
    println!("{}", geo_point.0);
    println!("{}", geo_point.1);
}

struct Nothing;

#[test]
fn test_unit_struct() {
    let _nothing1 = Nothing;
    let _nothing2 = Nothing;
}

#[test]
fn test_method() {
    let person = Person {
        first_name: String::from("Rizqi"),
        middle_name: String::from("Aulia"),
        last_name: String::from("Sabilla"),
        age: 20,
    };

    person.say_hello("Seira");
}

enum Level {
    Regular,
    Premium,
    Platinum,
}

#[test]
fn test_enum() {
    let level = Level::Premium;

    match level {
        Level::Regular => {
            println!("Level Regular");
        }
        Level::Premium => {
            println!("Level Premium");
        }
        Level::Platinum => {
            println!("Level Platinum");
        }
    }
}

enum Payment {
    CreditCard(String),
    BankTransfer(String, String),
    Ewallet(String, String),
}

impl Payment {
    fn pay(&self, amount: u32) {
        match self {
            Payment::CreditCard(number) => {
                println!("Paying with credit card {} amount {}", number, amount);
            }
            Payment::BankTransfer(bank, number) => {
                println!(
                    "Paying with bank transfer {} {} amount {}",
                    bank, number, amount,
                );
            }
            Payment::Ewallet(wallet, number) => {
                println!(
                    "Paying with bank transfer {} {} amount {}",
                    wallet, number, amount,
                );
            }
        }
    }
}

#[test]
fn test_payment() {
    let _payment1 = Payment::CreditCard(String::from("1234-5678-9012-3456"));
    _payment1.pay(1000);

    let _payment2 = Payment::BankTransfer(String::from("Bank ABC"), String::from("9876543210"));
    _payment2.pay(2000);

    let _payment3 = Payment::Ewallet(String::from("Dana"), String::from("1234567890"));
    _payment3.pay(3000);
}

#[test]
fn test_match_value() {
    let name = "Joko";

    match name {
        "Joko" => {
            println!("Hello Joko");
        }
        "Widodo" => {
            println!("Hello Widodo");
        }
        other => {
            println!("Hello {}", other);
        }
    }

    match name {
        "Joko" | "Widodo" => {
            println!("Hello Bos {}", name);
        }

        other => {
            println!("Hello {}", other);
        }
    }

    println!("Selamat datang {}", name);
}

#[test]
fn test_range_value() {
    let value = 20;

    match value {
        75..=100 => {
            println!("Nilai A");
        }
        65..=74 => {
            println!("Nilai B");
        }
        55..=64 => {
            println!("Nilai C");
        }
        other => {
            println!("Nilai D {}", other);
        }
    }
}

#[test]
fn test_struct_patterns() {
    let point = GeoPoint(2.0, 0.4);

    match point {
        GeoPoint(long, 0.0) => {
            println!("long {}", long);
        }
        GeoPoint(0.0, lat) => {
            println!("lat {}", lat);
        }
        GeoPoint(long, lat) => {
            println!("long {} lat {}", long, lat);
        }
    }

    let person = Person {
        first_name: String::from("Rizqi"),
        middle_name: String::from("Aulia"),
        last_name: String::from("Sabilla"),
        age: 20,
    };

    match person {
        Person {
            first_name,
            last_name,
            ..
        } => {
            println!("First Name: {}, Last Name: {}", first_name, last_name);
        }
    }
}

#[test]
fn test_ignorin() {
    let point = GeoPoint(2.0, 0.4);

    match point {
        GeoPoint(long, _) => {
            println!("long {}", long);
        }
    }
}

#[test]
fn test_range_value_ignoring() {
    let value = 20;

    match value {
        75..=100 => {
            println!("Nilai A");
        }
        65..=74 => {
            println!("Nilai B");
        }
        55..=64 => {
            println!("Nilai C");
        }
        _ => {
            println!("Nilai D");
        }
    }
}

#[test]
fn test_match_expression() {
    let value = 2;
    let result = match value {
        1 => "Lulus",
        2 => "Tidak Lulus",
        _ => "Tidak Diketahui",
    };

    println!("{}", result)
}

type Age = u8;
type IdentityNumber = String;

struct User {
    name: String,
    age: Age,
    identity_number: IdentityNumber,
}

#[test]
fn test_user() {
    let user = User {
        name: String::from("Rizqi"),
        age: 20,
        identity_number: String::from("1234567890"),
    };

    println!("Name: {}", user.name);
    println!("Age: {}", user.age);
    println!("Identity Number: {}", user.identity_number);
}

trait CanSayHello {
    fn hello(&self) {
        println!("Hello");
    }
    fn say_hello(&self) -> String;
    fn say_hello_me(&self) -> String;
    fn say_hello_to(&self, name: &str) -> String;
}

trait CanSayGoodbye {
    fn goodbye(&self) {
        println!("Goodbye");
    }
    fn say_goodbye_to(&self, name: &str) -> String;
    fn say_goodbye_me(&self) -> String;
}

impl CanSayHello for Person {
    fn say_hello_me(&self) -> String {
        format!("Hello, my name is {}", self.first_name)
    }

    fn say_hello(&self) -> String {
        format!("Hello, my name is {}", self.first_name)
    }

    fn say_hello_to(&self, name: &str) -> String {
        format!("Hello {}, my name is {}", name, self.first_name)
    }
}

impl CanSayGoodbye for Person {
    fn say_goodbye_me(&self) -> String {
        format!("Goodbye, my name is {}", self.first_name)
    }

    fn say_goodbye_to(&self, name: &str) -> String {
        format!("Goodbye {}, my name is {}", name, self.first_name)
    }
}

fn say_hello_trait(value: &impl CanSayHello) {
    println!("{}", value.say_hello_me());
}

#[test]
fn test_trait() {
    let person = Person {
        first_name: String::from("Rizqi"),
        middle_name: String::from("Aulia"),
        last_name: String::from("Sabilla"),
        age: 20,
    };

    // let result = person.say_hello_to("Seira");
    // println!("{}", result);

    // let result = person.say_hello_me();
    // println!("{}", result);

    // person.hello();

    // say_hello_trait(&person);

    // let result = person.say_goodbye_to("Seira");
    // println!("{}", result);

    CanSayHello::say_hello(&person);
    Person::say_hello(&person, "Seira");

    hello_and_goodbye(&person);
}

fn hello_and_goodbye(value: &(impl CanSayHello + CanSayGoodbye)) {
    println!("{}", value.say_hello_me());
    println!("{}", value.say_goodbye_me());
}

struct SimplePerson {
    name: String,
}

impl CanSayGoodbye for SimplePerson {
    fn say_goodbye_me(&self) -> String {
        format!("Goodbye {}", self.name)
    }

    fn say_goodbye_to(&self, name: &str) -> String {
        format!("Goodbye {} {}", name, self.name)
    }
}

fn create_person(name: String) -> impl CanSayGoodbye {
    SimplePerson { name }
}

#[test]
fn test_impl_trait() {
    let person = create_person(String::from("Rizqi"));

    println!("{}", person.say_goodbye_me());
}

trait CanSay: CanSayHello + CanSayGoodbye {}

impl CanSayHello for SimpleMan {
    fn say_hello(&self) -> String {
        format!("Hello {}", self.name)
    }

    fn say_hello_to(&self, name: &str) -> String {
        format!("Hello {} {}", name, self.name)
    }

    fn hello(&self) {
        println!("Hello {}", self.name)
    }

    fn say_hello_me(&self) -> String {
        format!("Hello {}", self.name)
    }
}

impl CanSayGoodbye for SimpleMan {
    fn say_goodbye_me(&self) -> String {
        format!("Goodbye {}", self.name)
    }

    fn say_goodbye_to(&self, name: &str) -> String {
        format!("Goodbye {} {}", name, self.name)
    }

    fn goodbye(&self) {
        println!("Goodbye {}", self.name)
    }
}

struct SimpleMan {
    name: String,
}

impl CanSay for SimpleMan {
    fn say_hello(&self) -> String {
        format!("Hello {}", self.name)
    }
}
