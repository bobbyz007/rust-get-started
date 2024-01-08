macro_rules! times_five {
    ($e:expr) => { 5 * $e };
}

// multiple metavariables
macro_rules! multiply_add {
    ($a: expr, $b: expr, $c: expr) => { $a * ($b + $c) };
}

macro_rules! vec_strs {
    (
        // Start a repetition:
        $(
            // Each repeat must contain an expression...
            $element:expr
        )
        // ...separated by commas...
        ,
        // ...zero or more times.
        *
    ) => {
        // Enclose the expansion in a block so that we can use multiple statements.
        {
            let mut v = Vec::new();

            // Start a repetition:
            $(
                // Each repeat will contain the following statement, with
                // $element replaced with the corresponding expression.
                v.push(format!("{}", $element));
            )*

            v
        }
    };
}


pub fn meta_variable() {
    println!("{:?}", times_five!(10));
    println!("{:?}", multiply_add!(1, 2, 3));
    println!("{:?}", vec_strs![1, "a", true, 3.14159f32]);
}