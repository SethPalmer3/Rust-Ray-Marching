use super::scene_objects;
use super::Point3D;
use core::slice::{Iter, IterMut};

#[allow(dead_code)]
pub struct Scene<T> where T: scene_objects::SceneObject + Clone {
    scene_objects: Vec<T>
}

#[allow(dead_code)]
pub struct ClosestObject<T> where T: scene_objects::SceneObject {
    pub distance: f64,
    pub obj: T,
}

#[allow(dead_code)]
impl<T> Scene<T> where T: scene_objects::SceneObject + Clone {

    pub fn new() -> Self{
        Scene { scene_objects: Vec::<T>::new() }
    }

    pub fn add_scene_object(&mut self, o: T){
        self.scene_objects.push(o)
    }

     pub fn get_min_distance(&self, p: &Point3D) -> Option<f64> {
        let mut min_dist: Option<f64> = None;
        for o in self.scene_objects.iter(){
            let dist = o.signed_distance(p);

            match min_dist {
                Some(min_d) => {
                    if dist < min_d{
                        min_dist = Some(dist);
                    }
                },
                None => min_dist = Some(dist),
            }
        }

        min_dist
    }

    pub fn get_closest_object(&self, p: &Point3D) -> Option<ClosestObject<T>>{
        let mut min_dist: Option<ClosestObject<T>> = None;

        for o in self.scene_objects.iter(){
            let dist = o.signed_distance(p);

            match min_dist {
                Some(ClosestObject { distance, obj: _ }) => {
                    if dist < distance{
                        min_dist = Some(ClosestObject { distance: dist, obj: o.clone() })
                    }
                },
                None => min_dist = Some(ClosestObject { distance: dist, obj: o.clone() }),
            }
        }

        min_dist
    }

    pub fn iter(&self) -> Iter<'_, T> {
        self.scene_objects.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        self.scene_objects.iter_mut()
    }
}
