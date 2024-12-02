use paste::paste;

macro_rules! select {
    ($year:literal, $day:literal, $stars:tt) => {
        paste! {
            {
                use aoc:: [<year_ $year>] as year;

                use year:: [<day $day>] as day;

                use day:: [< solution_ $stars >] as solution;

                solution
            }
        }
    };
}

macro_rules! select_both_stars {
    ($year:literal, $day:literal) => {{
        let one = select!($year, $day, one);
        let two = select!($year, $day, two);
        (one, two)
    }};
}

fn main() {
    let (one, two) = select_both_stars!(2024, 2);

    println!("---- One star ----");
    println!("{}\n", one());
    println!("---- Two star ----");
    println!("{}\n", two());
}
