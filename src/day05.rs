use std::cmp::{max, min};

pub struct Day05 {
    lines: Vec<Line>,
    area: Area,
}

impl Day05 {
    pub fn new() -> Self {
        let mut max_x = 0;
        let mut max_y = 0;
        let lines = super::parse_input(5)
            .iter()
            .map(|row| {
                let points = str::split(row, " -> ")
                    .map(|coord| {
                        let xy = str::split(coord, ",").collect::<Vec<_>>();
                        let x = str::parse::<usize>(xy[0]).unwrap();
                        let y = str::parse::<usize>(xy[1]).unwrap();

                        max_x = max(max_x, x);
                        max_y = max(max_y, y);

                        Point { x, y }
                    })
                    .collect::<Vec<_>>();

                Line(points[0], points[1])
            })
            .collect::<Vec<Line>>();

        Day05 {
            lines,
            area: Area::new(max_x, max_y),
        }
    }
}

impl super::Day for Day05 {
    fn p1(&self) -> String {
        let mut area = self.area.clone();

        for line in self
            .lines
            .iter()
            .filter(|l| l.is_horizontal() || l.is_vertical())
        {
            for point in line.points_in_line().iter() {
                area.mark_point(point);
            }
        }

        let overlapping_points = area.rows.into_iter().fold(0, |sum, row| {
            sum + row.into_iter().filter(|&x| x > 1).count()
        });

        format!("Part 1: {}", overlapping_points)
    }

    fn p2(&self) -> String {
        let mut area = self.area.clone();

        for line in self.lines.iter() {
            for point in line.points_in_line().iter() {
                area.mark_point(point);
            }
        }

        let overlapping_points = area.rows.into_iter().fold(0, |sum, row| {
            sum + row.into_iter().filter(|&x| x > 1).count()
        });

        format!("Part 2: {}", overlapping_points)
    }
}

#[derive(Clone, Debug)]
struct Area {
    rows: Vec<Vec<usize>>,
}

impl Area {
    fn new(max_x: usize, max_y: usize) -> Self {
        let mut row = Vec::with_capacity(max_x + 1);
        for _ in 0..=max_x {
            row.push(0);
        }

        let mut rows = Vec::with_capacity(max_y + 1);
        for _ in 0..=max_y {
            rows.push(row.clone());
        }

        Area { rows }
    }

    fn mark_point(&mut self, point: &Point) {
        self.rows[point.y][point.x] += 1;
    }
}

#[derive(Clone, Copy)]
struct Line(Point, Point);

impl Line {
    fn is_horizontal(&self) -> bool {
        self.0.y == self.1.y
    }

    fn is_vertical(&self) -> bool {
        self.0.x == self.1.x
    }

    fn points_in_line(&self) -> Vec<Point> {
        let Point { x: x1, y: y1 } = self.0;
        let Point { x: x2, y: y2 } = self.1;

        if self.is_horizontal() {
            let start = min(x1, x2);
            let end = max(x1, x2);

            return (start..=end)
                .into_iter()
                .map(|x| Point { x, y: y1 })
                .collect();
        }

        if self.is_vertical() {
            let start = min(y1, y2);
            let end = max(y1, y2);

            return (start..=end)
                .into_iter()
                .map(|y| Point { x: x1, y })
                .collect();
        }

        if x1 < x2 {
            (x1..=x2)
                .into_iter()
                .enumerate()
                .map(|(i, x)| Point {
                    x,
                    y: if y1 < y2 { y1 + i } else { y1 - i },
                })
                .collect()
        } else if y1 < y2 {
            (y1..=y2)
                .into_iter()
                .enumerate()
                .map(|(i, y)| Point {
                    x: if x1 < x2 { x1 + i } else { x1 - i },
                    y,
                })
                .collect()
        } else {
            (x2..=x1)
                .into_iter()
                .enumerate()
                .map(|(i, x)| Point { x, y: y2 + i })
                .collect()
        }
    }
}

#[derive(Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}
