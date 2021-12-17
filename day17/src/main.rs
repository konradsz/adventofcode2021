use std::collections::HashSet;

#[derive(Clone, Copy)]
struct TargetArea {
    x: (i32, i32),
    y: (i32, i32),
}

impl TargetArea {
    fn is_probe_inside(&self, x: i32, y: i32) -> bool {
        x >= self.x.0 && x <= self.x.1 && y >= self.y.0 && y <= self.y.1
    }
}

fn main() {
    let target_area = TargetArea {
        x: (102, 157),
        y: (-146, -90),
    };
    let min_vx = ((2 * target_area.x.0) as f64).sqrt() as i32;
    let max_vx = target_area.x.1;

    let velocities: HashSet<(i32, i32)> = (min_vx..=max_vx)
        .map(move |initial_vx| {
            (target_area.y.0..(target_area.y.0).abs()).filter_map(move |initial_vy| {
                let (mut x, mut y) = (0, 0);
                let (mut vx, mut vy) = (initial_vx as u32, initial_vy);

                loop {
                    x += vx as i32;
                    y += vy;
                    vy -= 1;
                    vx = vx.saturating_sub(1);

                    if target_area.is_probe_inside(x, y) {
                        return Some((initial_vx, initial_vy));
                    }
                    if y < target_area.y.0 {
                        return None;
                    }
                }
            })
        })
        .flatten()
        .collect();

    let velocity_with_max_y = velocities
        .iter()
        .max_by(|(_, y1), (_, y2)| y1.cmp(y2))
        .unwrap();
    let part_1 = velocity_with_max_y.1 * (velocity_with_max_y.1 + 1) / 2;
    let part_2 = velocities.len();

    assert_eq!(part_1, 10585);
    assert_eq!(part_2, 5247);
}
