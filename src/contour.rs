use crate::{PointIndex, EdgeIndex, HullIndex, half, triangulate::Triangulation};

safe_index::new! {
    ContourIndex,
    map: ContourVec with iter: ContourIter
}
const EMPTY: ContourIndex = ContourIndex { val: std::usize::MAX };

#[derive(Copy, Clone, Debug)]
pub enum ContourData {
    None,
    Buddy(EdgeIndex),
    Hull(HullIndex),
}

#[derive(Copy, Clone, Debug)]
struct Node {
    point: PointIndex,
    data: ContourData,
    prev: ContourIndex,
    next: ContourIndex,
}

pub struct Contour {
    pts: ContourVec<Node>,
    end: ContourIndex,
    index: ContourIndex,
    sign: bool,
}

/// A contour marks a set of points that form the boundary of a pseudopolygon
/// during fixed edge insertion.  Each point is marked with an optional
/// HullIndex (if the point is on the hull) or EdgeIndex (if the point has
/// a buddy edge); when that point is triangulated, the hull or half-edge
/// structure is updated accordingly.
///
/// Triangulation is based on ["Triangulating Monotone Mountains"](http://www.ams.sunysb.edu/~jsbm/courses/345/13/triangulating-monotone-mountains.pdf)
impl Contour {
    fn new(point: PointIndex, data: ContourData, sign: bool) -> Self {
        let n = Node { point, data, prev: EMPTY, next: EMPTY };
        Contour {
            pts: ContourVec { vec: vec![n] },
            end: ContourIndex::new(0),
            index: ContourIndex::new(0),
            sign,
        }
    }

    /// Constructs a new contour with a positive sign.  This can be either
    /// a left-to-right mountain, i.e.
    ///
    /// ```text
    ///              x^         e2
    ///             /  \    v2<----------v1
    ///          e5/    \  /              ^
    ///           /      xv                \e1
    ///          v                          \
    ///        v5 - - - - - - - - - - - - - > v0
    /// ```
    ///
    /// or a right-to-left valley, i.e.
    ///
    /// ```text
    ///        v0< - - - - - - - - - - - - - v5
    ///          \                          ^
    ///        e1 \     v2\                /e5
    ///            \    ^  \e3            /
    ///             v  /e2  v3---------->v4
    ///             v1/           e4
    /// ```
    pub fn new_pos(point: PointIndex, data: ContourData) -> Self {
        Self::new(point, data, true)
    }

    /// Constructs a new contour with a negative sign.  This can be either
    /// a right-to-left mountain, i.e.
    ///
    /// ```text
    ///             v1^           e4
    ///             /  \    v3<----------v4
    ///          e1/    \  /              ^
    ///           /     v2<                \e5
    ///          v                          \
    ///        v0 - - - - - - - - - - - - - > v5
    /// ```
    ///
    /// or a left-to-right valley, i.e.
    ///
    /// ```text
    ///        v5< - - - - - - - - - - - - - v0
    ///          \                          ^
    ///        e5 \     v3\                /e1
    ///            \    ^  \e3            /
    ///             v  /e4  v2---------->v1
    ///             v4/           e2
    /// ```
    pub fn new_neg(point: PointIndex, data: ContourData) -> Self {
        Self::new(point, data, false)
    }

    pub fn push(&mut self, t: &mut Triangulation,
                point: PointIndex, data: ContourData) -> Option<EdgeIndex> {
        let i = self.pts.push(Node {
            point, data, next: EMPTY, prev: self.end
        });
        self.pts[self.end].next = i;
        self.end = i;

        let mut out = None;
        while let Some(e) = self.try_clip(t) {
            out = Some(e);
        }
        // Advance to the end of the triangulation
        self.index = self.pts[self.index].next;
        assert!(self.pts[self.index].next == EMPTY);
        out
    }

    /// Attempts to clip the ear with tip self.index.
    /// Returns the new edge and retreats self.index on success.
    fn try_clip(&mut self, t: &mut Triangulation) -> Option<EdgeIndex> {
        let c = self.pts[self.index];
        assert!(c.prev != EMPTY);
        if c.next == EMPTY {
            return None;
        }

        let new_edge = if self.sign {
            /*
                             c (self.index)
                           /  ^
                          /    \
                    ba/ha/      \bc/hc
                        /        \
                       V   e_ab   \
                [next] a - - - - - >b [prev]

                From the contour's perspective, this flattens out to
                     a <---------- b

                e_ab is a new edge inserted here
             */
            let (a, b) = (self.pts[c.next], self.pts[c.prev]);

            // If the ear isn't strictly convex, then return immediately
            if t.orient2d(a.point, b.point, c.point) <= 0.0 {
                return None;
            }

            // Insert the new triangle
            let e_ab = t.half.insert(a.point, b.point, c.point,
                                     half::EMPTY, half::EMPTY, half::EMPTY);
            // Link the new triangle with buddies or hull edges
            let edge_ab = t.half.edge(e_ab);
            let e_ca = edge_ab.prev;
            let e_bc = edge_ab.next;
            match a.data {
                ContourData::None => (),
                ContourData::Hull(h) => t.hull.update(h, e_ca),
                ContourData::Buddy(b) => t.half.link(b, e_ca),
            };
            match c.data {
                ContourData::None => (),
                ContourData::Hull(h) => t.hull.update(h, e_bc),
                ContourData::Buddy(b) => t.half.link(b, e_bc),
            };

            e_ab
        } else {
            /*
                               c (self.index)
                             /  ^
                            /    \
                         ec/      \ea
                          /        \
                         V   e_ba   \
                [prev]  b - - - - - >a [next]

                From the contour's perspective, this flattens out to
                       b <----------- a

                e_ba is a new edge inserted here
             */
            let (a, b) = (self.pts[c.next], self.pts[c.prev]);

            // If the ear isn't strictly convex, then return immediately
            if t.orient2d(a.point, c.point, b.point) <= 0.0 {
                return None;
            }

            // Insert the new triangle
            let e_ba = t.half.insert(b.point, a.point, c.point,
                                     half::EMPTY, half::EMPTY, half::EMPTY);
            // Link the new triangle with buddies or hull edges
            let edge_ba = t.half.edge(e_ba);
            let e_cb = edge_ba.prev;
            let e_ac = edge_ba.next;
            match a.data {
                ContourData::None => (),
                ContourData::Hull(h) => t.hull.update(h, e_ac),
                ContourData::Buddy(b) => t.half.link(b, e_ac),
            };
            match c.data {
                ContourData::None => (),
                ContourData::Hull(h) => t.hull.update(h, e_cb),
                ContourData::Buddy(b) => t.half.link(b, e_cb),
            };
            e_ba
        };

        // Stitch the index out of the list
        self.pts[self.index] = Node {
            point: PointIndex::new(0),
            data: ContourData::None,
            prev: EMPTY, next: EMPTY
        };
        self.pts[c.next].prev = c.prev;
        self.pts[c.prev].next = c.next;

        // Any new triangles that use the new edge need to link it as a buddy
        self.pts[c.next].data = ContourData::Buddy(new_edge);

        // Take a step back along the contour so that we can try to clip
        // another ear (in case this most recent clipping made the previous
        // ear convex as well)
        self.index = c.prev;

        Some(new_edge)
    }
}