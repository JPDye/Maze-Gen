// Standard imports
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};

// Self imports
pub type HardCellLink = Rc<RefCell<Cell>>;
pub type SoftCellLink = Weak<RefCell<Cell>>;

/// A cardinal direction (North, South, East, West). Used to point to a Cell's neighbours.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Direction {
    N,
    E,
    S,
    W,
}

use Direction::*;

/// A Cell within a Maze. Has references to it's neighbours, itself and maintains a list of linked Cells. Uses Direction Enum.
/// Neighbours are pointed to in a HashMap with a Direction (North, South, East, West) for a key and an Option<Weak<RefCell<Cell> as a value.
/// Cells can be linked by adding the Direction a neighbour exists in to the Vec<Direction> contained within 'self.links'.
pub struct Cell {
    pub self_rc: Option<SoftCellLink>,
    pub neighbours: HashMap<Direction, Option<SoftCellLink>>,

    // Vector of Directions. Prescence in the vector indicates a link in the specified Direction.
    pub links: Vec<Direction>,
}

impl Cell {
    /// Create a new Cell and return a HardCellLink (Rc<RefCell<Cell>>) to it.
    pub fn new() -> HardCellLink {
        let c = Cell {
            self_rc: None,
            neighbours: HashMap::new(),
            links: Vec::new(),
        };

        let rc = Rc::new(RefCell::new(c)); // Create HardCellLink to cell 'c'.
        rc.borrow_mut().self_rc = Some(Rc::downgrade(&rc)); // Downgrade HardCellLink to SoftCellLink and set 'self_rc'
        rc
    }

    /// Return the neighbour that lies in the specified Direction.
    pub fn get_neighbour(&self, d: Direction) -> Option<HardCellLink> {
        let nb_weak = self.neighbours.get(&d).unwrap().as_ref().unwrap();
        let nb_rc = nb_weak.upgrade().unwrap();
        Some(nb_rc)
    }

    /// Return a Vector containing all Directions a Cell exists in.
    pub fn get_neighbours(&self) -> Vec<Direction> {
        let mut res = Vec::new();
        for (&key, value) in self.neighbours.iter() {
            if value.is_some() {
                res.push(key)
            }
        }
        res
    }

    /// Return true if a neighbour exists in a specified direction.
    pub fn neighbour_exists(&self, d: Direction) -> bool {
        self.neighbours.get(&d).unwrap().is_some()
    }

    /// Return the Directions of neighbours the current Cell is linked to.
    pub fn get_linked(&self) -> &Vec<Direction> {
        &self.links
    }

    /// Return the Directions of neighbours the current Cell isn't linked to.
    pub fn get_unlinked(&self) -> Vec<Direction> {
        vec![N, S, E, W]
            .into_iter()
            .filter(|d| self.neighbours.get(&d).is_some() && !self.links.contains(d))
            .collect()
    }

    /// Return true if Cell is linked to a neighbour in the given Direction. Otherwise, return false.
    pub fn is_linked(&self, d: Direction) -> bool {
        self.links.contains(&d)
    }

    /// Link the currenct Cell with the provided Cell if the given Cell is a neighbour.
    pub fn link_using_ref(&mut self, other: &HardCellLink) {
        let other = other.borrow();
        let other = other.self_rc.as_ref().unwrap();

        let mut other_dir = None;

        for (dir, nb) in &self.neighbours {
            if nb.as_ref().unwrap().ptr_eq(&other) {
                other_dir = Some(*dir);
            }
        }

        self.link(other_dir.unwrap());
    }

    /// Link the current Cell with it's neighbour in given Direction. Also links neighbour to current Cell.
    pub fn link(&mut self, d: Direction) {
        // Get HardCellLink to neighbour.
        let nb = self
            .neighbours
            .get(&d)
            .unwrap()
            .as_ref()
            .unwrap()
            .upgrade()
            .unwrap(); // Link Cell with neighbour.
        self.link_single(d);

        // Match 'd' and link neighbour to Cell.
        match d {
            N => nb.borrow_mut().link_single(S),
            S => nb.borrow_mut().link_single(N),
            E => nb.borrow_mut().link_single(W),
            W => nb.borrow_mut().link_single(E),
        }
    }

    /// Link the current Cell to it's neighbour in specified Direction.
    pub fn link_single(&mut self, d: Direction) {
        self.links.push(d);
    }

    /// Unlink the current Cell with it's neighbour in given Direction. Also unlinks neighbour with current Cell.
    pub fn unlink(&mut self, d: Direction) {
        // Get HardCellLink to neighbour.
        let nb = self
            .neighbours
            .get(&d)
            .unwrap()
            .as_ref()
            .unwrap()
            .upgrade()
            .unwrap();

        // Unlink Cell with neighbour.
        self.unlink_single(d);

        // Match 'd' and unlink neighbour with Cell.
        match d {
            N => nb.borrow_mut().unlink_single(S),
            S => nb.borrow_mut().unlink_single(N),
            E => nb.borrow_mut().unlink_single(W),
            W => nb.borrow_mut().unlink_single(E),
        }
    }

    /// Unlink the current Cell with it's neighbour in specified Direction.
    pub fn unlink_single(&mut self, d: Direction) {
        let idx = self.links.iter().position(|&x| x == d).unwrap();
        self.links.remove(idx);
    }
}
