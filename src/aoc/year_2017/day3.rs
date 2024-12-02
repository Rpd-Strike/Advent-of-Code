use std::fmt::Display;

pub fn solution_one() -> impl Display {
    let data = 312051;
    let mut steps_left = data - 1;

    let mut dirs = [1, 1, 2, 2];
    let deltas = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    let mut dir = 0;
    let mut used = 0;
    let mut x = 0;
    let mut y = 0;

    // let mut written = vec![];

    while steps_left > 0 {
        x += deltas[dir].0;
        y += deltas[dir].1;

        used += 1;
        steps_left -= 1;

        if used == dirs[dir] {
            dirs[dir] += 2;
            dir += 1;
            dir %= 4;

            used = 0;
        }
    }

    if x < 0 {
        x *= -1;
    }
    if y < 0 {
        y *= -1;
    }

    x + y
}

pub fn solution_two() -> impl Display {
    0
}
