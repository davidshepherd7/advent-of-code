use std::cmp;
use std::collections::HashMap;

fn main() {
    assert!(f(1) == 0);
    assert!(f(3) == 2);
    assert!(f(4) == 1);
    assert!(f(5) == 2);
    assert!(f(6) == 1);
    assert!(f(12) == 3);
    assert!(f(23) == 2);
    assert!(f(1024) == 31);

    // println!("f(325489): {}", f(325489));



    let mut memo: HashMap<(i32, i32), u32> = HashMap::new();

    assert!(g(0, 0, &mut memo) == 1);

    let points = vec![(1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0), (-1, -1), (0, -1), (1, -1),
                      (2, -1), (2, 0), (2, 1), (2, 2), (1, 2)];

    // for p in points.clone() {
    //     let comp = comparator(&p);
    //     println!("point: {} {}, comparator: {} {} {}", p.0, p.1, comp.0, comp.1, comp.2);
    // }

    let mut sorted = points.clone();
    sorted.sort_by_key(comparator);
    // println!("sort order");
    // for p in sorted.clone() {
    //     println!("point: {} {}", p.0, p.1);
    // }
    assert!(sorted.iter().eq(points.iter()));

    // println!("{}", g(5,4, &mut memo));

    assert!(g(0, 0, &mut memo) == 1);
    assert!(g(1, 0, &mut memo) == 1);
    assert!(g(2, 0, &mut memo) == 54);
    assert!(g(-2, 1, &mut memo) == 304);
    assert!(g(2, 2, &mut memo) == 59);
    assert!(g(-2, -2, &mut memo) == 362);
    assert!(g(1, -1, &mut memo) == 25);

    // 147  142  133  122   59
    // 304    5    4    2   57
    // 330   10    1    1   54
    // 351   11   23   25   26
    // 362  747  806--->   ...

    // cartesian product, not in stdlib
    let mut all_pairs = Vec::new();
    for i in -5..5 {
        for j in -5..5 {
            all_pairs.push((i, j));
        }
    }

    let result = all_pairs
        .iter()
        .map(|&(i, j)| (i, j, g(i, j, &mut memo)))
        .filter(|&(_, _, v)| v.clone() > 325489)
        .min_by_key(|&(_, _, v)| v.clone())
        .unwrap();

    println!("({} {}): {}", result.0, result.1, result.2);
}


fn g(ix: i32, iy: i32, mut hash: &mut HashMap<(i32, i32), u32>) -> u32 {
    if ix == 0 && iy == 0 {
        return 1;
    }

    if let Some(val) = hash.get(&(ix, iy)) {
        return *val;
    }

    let out = [(-1,  1), (0,  1), ( 1,  1),
            (-1,  0),          ( 1,  0),
            (-1, -1), (0, -1), ( 1, -1)].iter()
        .map(|&(dx, dy)| (ix + dx, iy + dy))
        .filter(|other| is_before(other, &(ix, iy)))
        .map(|(ox, oy)| g(ox, oy, &mut hash))
        .sum();

    hash.insert((ix, iy), out);
    return out;
}

fn is_before(p1: &(i32, i32), p2: &(i32, i32)) -> bool {
    return comparator(p1) < comparator(p2);
}

fn comparator(p: &(i32, i32)) -> (u32, u32, i32) {
    let side: u32;
    let side_steps: i32;

    if p.0.abs() > p.1.abs() {
        if p.0 >= 0 {
            side = 0;
            side_steps = p.1
        } else {
            side = 2;
            side_steps = -p.1
        };
    }
    else {
        if p.1 > 0 {
            side = 1;
            side_steps = -p.0;
        }
        else {
            side = 3;
            side_steps = p.0;
        };
    }

    return (shell(p), side, side_steps);
}

fn shell(p: &(i32, i32)) -> u32 {
    return cmp::max(p.0.abs(), p.1.abs()) as u32;
}









fn f(one_indexed_position: u32) -> u32 {
    assert!(one_indexed_position > 0);

    if one_indexed_position == 1 {
        return 0;
    }
    let position = one_indexed_position - 1;


    let sqrt = (position as f64).sqrt().trunc() as u32;
    let side_length = if sqrt % 2 == 0 {sqrt + 1} else {sqrt + 2};

    let previous_side_length = side_length - 2;
    let extra_steps_along_side = (position - (previous_side_length.pow(2))) % (side_length - 1);

    let hops_to_right_angle = (extra_steps_along_side as i32 - (side_length/2) as i32 + 1).abs() as u32;

    return (side_length / 2) + hops_to_right_angle;
}
