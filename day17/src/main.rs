use std::collections::HashSet;

struct Probe {
    x: i32,
    y: i32,

    xvel: i32,
    yvel: i32,
    
    maxy: i32,
}

#[derive(Debug)]
struct Target {
    xmin: i32,
    xmax: i32,
    ymin: i32,
    ymax: i32,
}

impl Probe {
    fn new() -> Probe {
        Probe { x: 0, y: 0, xvel: 0, yvel: 0, maxy: 0 }
    }

    fn step(&mut self) {
        self.x += self.xvel;
        self.y += self.yvel;

        if self.y > self.maxy {
            self.maxy = self.y;
        }

        if self.xvel > 0 {
            self.xvel -= 1;
        }
        if self.xvel < 0 {
            self.xvel += 1;
        }
        self.yvel -= 1;
    }

    fn is_in_target(&self, target: &Target) -> bool {
        self.x >= target.xmin && self.x <= target.xmax && self.y >= target.ymin && self.y <= target.ymax 
    }

    fn can_never_reach_target(&self, target: &Target) -> bool {
        self.y < target.ymin
    }
}

fn star_one(target: &Target) -> i32 {
    let mut maxy: i32 = 0;
    for yvel in -200..200 {
        for xvel in -200..200 {
            let mut probe = Probe::new();
            probe.yvel = yvel;
            probe.xvel = xvel;

            while !probe.is_in_target(target) && !probe.can_never_reach_target(target) {
                probe.step();
            }

            if probe.is_in_target(target) {
                maxy = probe.maxy;
            }
        }
    }

    maxy
}

fn star_two(target: &Target) -> usize {
    let mut probes: HashSet<(i32,i32)> = HashSet::new();
    for yvel in -200..200 {
        for xvel in -200..200 {
            let mut probe = Probe::new();
            probe.yvel = yvel;
            probe.xvel = xvel;

            while !probe.is_in_target(target) && !probe.can_never_reach_target(target) {
                probe.step();
            }

            if probe.is_in_target(target) {
                probes.insert((yvel, xvel));
            }
        }
    }

    probes.iter().count()
}

fn main() {
    // For once just hardcoded the input, parsing does not add value here
    static STAR_TARGET: Target = Target { xmin: 153, xmax: 199, ymin: -114, ymax: -75 };
    let ans = star_one(&STAR_TARGET);
    println!("Star one: {}", ans);

    let ans = star_two(&STAR_TARGET);
    println!("Star two: {}", ans);
}

#[cfg(test)]
mod tests {
    static TEST_DATA: super::Target = super::Target { xmin: 20, xmax: 30, ymin: -10, ymax: -5 };

    #[test]
    fn test_star_one() {
        let ans = super::star_one(&TEST_DATA);
        assert_eq!(ans, 45);
    }

    #[test]
    fn test_star_two() {
        let ans = super::star_two(&TEST_DATA);
        assert_eq!(ans, 112);
    }
}
