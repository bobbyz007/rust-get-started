fn main() {
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is : {}", x);
    }

    println!("The value of x is: {}", x);

    let spaces = "   ";
    println!("space: {}", spaces);
    let spaces = spaces.len();
    println!("space : {}", spaces);

    let tup:(i32, f64, u8) = (500, 6.4, 1);
    let (_x, y, _z) = tup;
    println!("y: {}, z: {}", y, tup.2);

    let a:[i32; 5] = [1, 2, 3, 4, 5];
    let a2 = [3; 5];
    println!("a: {}, a2: {}", a[0], a2[1]);

    another_function(10);

    let y = {
        let x = 3;
        x + 1
    };
    println!("the values of y: {}", y);

    println!("five: {}", five());

    let number = 3;
    if number > 1 {
        println!("number");
    }

    let condition = true;
    let number = if condition { 5 } else { 10 };
    println!("number: {}", number);

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("result: {}", result);

    for number in (1..4).rev()  {
        println!("{}!", number);
    }

    println!("------------------ownership------------------");
    test_ownership();
}

fn test_ownership() {
    let s = "hello";
    let mut s2 = String::from("hello");
    s2.push_str(", World!");
    println!("{}", s2);

    let s1 = String::from("hello");
    let s2 = s1;

    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
    // but i32 is Copy, so it's okay to still
    // use x afterward

    println!("x: {}", x);

    let mut s1 = String::from("hello");
    let len = calculate_length(&mut s1);
    println!("the length of {} is {}", s1, len);

    let mut s = String::from("hello");
    let r1 = & s;
    let r2 = & s;
    println!("{}, {}", r1, r2);

    let r3 = &mut s;
    println!("{}", r3);

    let mut s = String::from("he world");
    let word = first_word(&s);

    println!("index: {}", word);
    s.clear();

    let s = "hello world";
    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);

}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn calculate_length(s: &mut String) -> usize {
    s.push_str("world");
    s.len()
}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
// memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.


fn five() -> i32 {
    // return 5;
    5
}

fn another_function(x: i32) {
    println!("another function: {}", x);
}
