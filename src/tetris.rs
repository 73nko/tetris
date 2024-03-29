use std::{collections::HashSet, mem};

use crate::shape::{Pos, Shape};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Tetris {
    pub width: i32,
    pub height: i32,
    current_shape: Shape,
    fixed_shapes: Vec<Shape>,
    lost: bool,
}

impl Tetris {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            width,
            height,
            current_shape: &Shape::new_random() + Pos((width - 1_i32) / 2, 0),
            fixed_shapes: vec![],
            lost: false,
        }
    }

    pub fn iter_positions(&self) -> impl Iterator<Item = Pos> {
        let height = self.height;
        let width = self.width;
        (0..height).flat_map(move |y| (0..width).map(move |x| Pos(x, y)))
    }

    pub fn get(&self, pos: Pos) -> Option<&'static str> {
        if self.current_shape.has_positions(pos) {
            Some(self.current_shape.typ())
        } else {
            self.fixed_shapes
                .iter()
                .find(|s| s.has_positions(pos))
                .map(|s| s.typ())
        }
    }

    pub fn is_out_of_bounds(&self, shape: &Shape) -> bool {
        !shape
            .positions()
            .all(|pos| 0 <= pos.0 && pos.0 < self.width && 0 <= pos.1 && pos.1 < self.height)
    }

    pub fn is_colliding(&self, shape: &Shape) -> bool {
        self.fixed_shapes
            .iter()
            .any(|fixed_shape| fixed_shape.collides_with(shape))
    }

    pub fn is_line_full(&self, y: i32) -> bool {
        self.fixed_shapes
            .iter()
            .flat_map(|shape| shape.positions())
            .filter(|pos| pos.1 == y)
            .collect::<HashSet<_>>()
            .len() as i32
            == self.width
    }

    pub fn tick(&mut self) -> Self {
        if self.lost {
            return self.to_owned();
        }
        let translated_current_shape = &self.current_shape + Pos(0, 1);

        if self.is_out_of_bounds(&translated_current_shape)
            || self.is_colliding(&translated_current_shape)
        {
            let new_fixed_shape = mem::replace(
                &mut self.current_shape,
                &Shape::new_random() + Pos(self.width / 2, 0),
            );

            self.fixed_shapes.push(new_fixed_shape);
            self.remove_full_lines();

            if self.is_colliding(&self.current_shape) {
                self.lost = true;
            }
        } else {
            self.current_shape = translated_current_shape;
        }

        self.to_owned()
    }

    pub fn shift(&mut self, direction: Direction) -> Self {
        if self.lost {
            return self.to_owned();
        }

        let translated_current_shape = &self.current_shape
            + match direction {
                Direction::Left => Pos(-1, 0),
                Direction::Right => Pos(1, 0),
            };

        if !self.is_out_of_bounds(&translated_current_shape)
            && !self.is_colliding(&translated_current_shape)
        {
            self.current_shape = translated_current_shape;
        }

        self.to_owned()
    }

    pub fn rotate(&mut self) -> Self {
        if self.lost {
            return self.to_owned();
        }
        let rotated_current_shape = self.current_shape.rotated();
        if !self.is_out_of_bounds(&rotated_current_shape)
            && !self.is_colliding(&rotated_current_shape)
        {
            self.current_shape = rotated_current_shape;
        }

        self.to_owned()
    }

    fn remove_line(&mut self, y: i32) {
        for shape in self.fixed_shapes.iter_mut() {
            shape.remove_line(y);
        }
    }

    fn remove_full_lines(&mut self) {
        for y in 0..self.height {
            if self.is_line_full(y) {
                self.remove_line(y);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Tetris;
    #[test]
    fn test_tetris() {
        let mut tetris = Tetris::new(10, 13);

        for _ in 1..3 {
            tetris.tick();
        }
        println!("{:#?}", tetris);
    }
}
