// Values, variables, and pointers
fn listing_1_1() {
    let x = 42;
    let y = 43;
    let var1 = &x;
    let mut var2 = &x;
    eprintln!("var2 = {:?}", var2);
    var2 = &y;
    eprintln!("var1 = {:?}, var2 = {:?}", var1, var2);
}

// Illegal flows that the borrow checker will catch
fn listing_1_2() {
    let mut x;
    // this access would be illegal, nowhere to draw the flow from:
    // assert_eq!(x, 42);
    x = 42;
    // this is okay, can draw a flow from the value assigned above:
    let y = &x;
    let y2 = &x;
    assert_eq!(*y, 42);
    assert_eq!(*y2, 42);
    // this establishes a second, mutable flow from x:
    x = 43;
    assert_eq!(x, 43);
    // this continues the flow from y, which in turn draws from x.
    // but that flow conflicts with the assignment to x!
    // assert_eq!(*y, 42);
}

// Moving and copying semantics
fn listing_1_3() {
    let x1 = 42;
    let y1 = Box::new(84);
    {
        // starts a new scope
        let z = (x1, y1);
        eprintln!("z = {:?}", z);
        // z goes out of scope, and is dropped;
        // it in turn drops the values from x1 and y1
    }
    // x1's value is Copy, so it was not moved into z
    let x2 = x1;
    eprintln!("x2 = {:?}", x2);
    // y1's value is not Copy, so it was moved into z
    // let y2 = y1;
}

// Rust assumes that shared references are immutable.
fn listing_1_4() {
    fn cache(input: &i32, sum: &mut i32) {
        *sum = *input + *input;
        assert_eq!(*sum, 2 * *input);
    }

    let x = 3;
    let mut sum = 0;
    cache(&x, &mut sum);

    eprintln!("sum = {:?}", sum);
}

// Rust assumes that mutable references are exclusive
fn listing_1_5() {
    fn noalias(input: &i32, output: &mut i32) {
        if *input == 1 {
            *output = 2;
        }
        if *input != 1 {
            *output = 3;
        }
    }

    let inp = 3;
    let mut outp = 0;
    noalias(&inp, &mut outp);

    eprintln!("outp = {:?}", outp);
}

// Mutability applies only to the immediately referenced memory
fn listing_1_6() {
    let x = 42;
    let mut y = &x; // y is of type &i32
    let z = &mut y; // z is of type &mut &i32
    eprintln!("z = {:?}", z);
    let x1 = 23;
    *z = &x1;
    eprintln!("z = {:?}", z);
}

// Mutability applies only to the immediately referenced memor
fn listing_1_7() {
    fn replace_with_84(s: &mut Box<i32>) {
        // this is not okay, as *s would be empty:
        // let was = *s;
        // but this is:
        let was = std::mem::take(s);
        // so is this:
        *s = was;
        // we can exchange values behind &mut:
        let mut r = Box::new(84);
        std::mem::swap(s, &mut r);
        assert_ne!(*r, 84);
    }
    let mut s = Box::new(42);
    replace_with_84(&mut s);
    eprintln!("s = {:?}", s);
}

// Lifetimes do not need to be contiguous.
fn listing_1_8(input: i32) {
    let mut x = Box::new(42);
    let r = &x; // 'a
    let r2 = &x; // 'a
    if input > 5 {
        *x = 84;
        // error: would cause before previous line:
        // "cannot assign to `*x` because it is borrowed"
        // println!("{}", r);
    } else {
        println!("{} {}", r, r2); // 'a
    }
}

fn main() {
    println!("Hello, world!");
    listing_1_1();
    listing_1_2();
    listing_1_3();
    listing_1_4();
    listing_1_5();
    listing_1_6();
    listing_1_7();
    listing_1_8(1);
}
