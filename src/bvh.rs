use std::sync::Arc;

use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable},
    random::m_random_range,
    ray::Ray,
    vec::Vec2,
};

pub struct BvhNode {
    bbox: AABB,
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
}

impl Hittable for BvhNode {
    fn hit<'a>(&'a self, ray: &Ray, t_range: Vec2) -> Option<HitRecord<'a>> {
        if !self.bbox.hit(ray, t_range) {
            return None;
        }
        let hit_left = self.left.hit(ray, t_range);
        let new_t_range = if let Some(ref rec) = hit_left {
            Vec2::new(t_range.0, rec.t)
        } else {
            t_range
        };
        let hit_right = self.right.hit(ray, new_t_range);
        match (hit_left, hit_right) {
            (None, None) => None,
            (Some(rec), None) => Some(rec),
            (None, Some(rec)) => Some(rec),
            (Some(left_rec), Some(right_rec)) => {
                if left_rec.t < right_rec.t {
                    Some(left_rec)
                } else {
                    Some(right_rec)
                }
            }
        }
    }
    fn bounding_box(&self) -> &AABB {
        &self.bbox
    }
}

impl BvhNode {
    pub fn new(list: &mut [Arc<dyn Hittable>]) -> Self {
        let axis = m_random_range(0..3);
        list.sort_by(|a, b| {
            let a_box = a.bounding_box();
            let b_box = b.bounding_box();
            a_box[axis].0.total_cmp(&b_box[axis].0)
        });

        let left: Arc<dyn Hittable>;
        let right: Arc<dyn Hittable>;
        let len = list.len();
        if len == 1 {
            left = list[0].clone();
            right = list[0].clone();
        } else if len == 2 {
            left = list[0].clone();
            right = list[1].clone();
        } else {
            let (left_list, right_list) = list.split_at_mut(len / 2);
            left = Arc::new(BvhNode::new(left_list));
            right = Arc::new(BvhNode::new(right_list));
        }
        let bbox = AABB::from_aabb(left.bounding_box(), right.bounding_box());
        BvhNode { bbox, left, right }
    }
}
