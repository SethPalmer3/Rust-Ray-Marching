use super::scene_objects;
use super::Point3D;
use core::slice::{Iter, IterMut};

#[allow(dead_code)]
pub struct Scene {
    scene_objects: Vec<Box<dyn scene_objects::SceneObject>>
}

#[allow(dead_code)]
pub struct ClosestObject<'a> {
    pub distance: f64,
    pub obj: &'a Box<dyn scene_objects::SceneObject>,
}

#[allow(dead_code)]
impl<'a> Scene {

    pub fn new() -> Self{
        Scene { scene_objects: Vec::<Box<dyn scene_objects::SceneObject>>::new() }
    }

    pub fn add_scene_object<T>(&mut self, o: T) where T: scene_objects::SceneObject + 'static{
        self.scene_objects.push(Box::new(o))
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

    pub fn get_closest_object(&self, p: &Point3D) -> Option<ClosestObject>{
        let mut min_dist: Option<ClosestObject> = None;

        for o in self.scene_objects.iter(){
            let dist = o.signed_distance(p);

            match min_dist {
                Some(ClosestObject { distance, obj: _ }) => {
                    if dist < distance{
                        min_dist = Some(ClosestObject { distance: dist, obj: o })
                    }
                },
                None => min_dist = Some(ClosestObject { distance: dist, obj: o }),
            }
        }

        min_dist
    }

    pub fn iter(&self) -> Iter<'_, Box<dyn scene_objects::SceneObject>> {
        self.scene_objects.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, Box<dyn scene_objects::SceneObject>> {
        self.scene_objects.iter_mut()
    }
}
