use crate::{
    camera::{Camera, CameraAnimation},
    game::GameState,
    geometry::{
        boundingbox::BoundingBox, rectangle::Rectangle, square::Square, unitcube::UnitCube,
        PrimitiveGeometry,
    },
    renderer::{RenderData, Renderer},
    types::*,
    utils::{f32, pt3f, quat4f, vec3f, NSEC_PER_SEC},
};
use specs::prelude::*;
use specs::Entity;
use specs_derive::Component;
use std::{
    cell::RefCell, collections::HashSet, f32::consts::PI, ops::DerefMut, rc::Rc, time::Duration,
};
use winit::VirtualKeyCode;

#[derive(Debug)]
pub struct TransformComponent {
    pub transform: Transform3f,
}

impl Component for TransformComponent {
    type Storage = FlaggedStorage<Self>;
}

impl TransformComponent {
    pub fn new(transform: Transform3f) -> TransformComponent {
        TransformComponent { transform }
    }
}

#[derive(Component, Debug)]
pub enum PrimitiveGeometryComponent {
    Rectangle(Rectangle),
    Square(Square),
    UnitCube(UnitCube),
}

impl PrimitiveGeometryComponent {
    pub fn vtx_data(&mut self, transform: &Transform3f) -> Vec<Vertex3f> {
        match self {
            PrimitiveGeometryComponent::Rectangle(ref mut rect) => rect.vtx_data(transform),
            PrimitiveGeometryComponent::Square(ref mut square) => square.vtx_data(transform),
            PrimitiveGeometryComponent::UnitCube(ref mut cube) => cube.vtx_data(transform),
        }
    }

    pub fn geometry(&self) -> &PrimitiveGeometry {
        use self::PrimitiveGeometryComponent::*;
        match self {
            Rectangle(ref rect) => rect,
            Square(ref square) => square,
            UnitCube(ref cube) => cube,
        }
    }
}

pub struct BoundingBoxComponent {
    pub bbox: BoundingBox,
}

impl Component for BoundingBoxComponent {
    type Storage = FlaggedStorage<Self>;
}

impl BoundingBoxComponent {
    pub fn new(bbox: BoundingBox) -> BoundingBoxComponent {
        BoundingBoxComponent { bbox }
    }
}

pub struct BoundingBoxComponentSystem {
    reader_id: ReaderId<ComponentEvent>,
    inserted: BitSet,
    modified: BitSet,
}

impl BoundingBoxComponentSystem {
    pub fn new(
        reader_id: ReaderId<ComponentEvent>,
        inserted: BitSet,
        modified: BitSet,
    ) -> BoundingBoxComponentSystem {
        BoundingBoxComponentSystem {
            reader_id,
            inserted,
            modified,
        }
    }
}

impl<'a> System<'a> for BoundingBoxComponentSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, PrimitiveGeometryComponent>,
        ReadStorage<'a, TransformComponent>,
        WriteStorage<'a, BoundingBoxComponent>,
    );

    fn run(&mut self, (entities, primitives, transforms, mut bounding_boxes): Self::SystemData) {
        self.inserted.clear();
        self.modified.clear();

        let events = transforms.channel().read(&mut self.reader_id);
        for event in events {
            match event {
                ComponentEvent::Inserted(id) => {
                    self.inserted.add(*id);
                }
                ComponentEvent::Modified(id) => {
                    self.modified.add(*id);
                }
                _ => (),
            }
        }

        for (entity, primitive, transform, _) in
            (&entities, &primitives, &transforms, &self.inserted).join()
        {
            let bbox = primitive.geometry().bounding_box(&transform.transform);
            bounding_boxes
                .insert(entity, BoundingBoxComponent::new(bbox))
                .unwrap_or_else(|err| panic!("{:?}", err));
        }
    }
}

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct SelectedComponent;

pub struct RenderSystem {
    pub renderer: Rc<RefCell<Renderer>>,
}

impl<'a> System<'a> for RenderSystem {
    type SystemData = (
        ReadStorage<'a, TransformComponent>,
        WriteStorage<'a, PrimitiveGeometryComponent>,
        WriteExpect<'a, GameState>,
    );

    fn run(&mut self, (transform_storage, mut geometry, mut game_state): Self::SystemData) {
        let mut renderer = self.renderer.borrow_mut();
        let game_state = game_state.deref_mut();
        let GameState {
            ref resized,
            ref mut camera,
            ref pressed_keys,
            ref mouse_delta,
            ref elapsed_time,
            ref frame_time_delta,
            ref mut camera_animation,
        } = game_state;

        let d_yaw = mouse_delta.0 as f32 / 500.0;
        let d_pitch = mouse_delta.1 as f32 / 500.0;
        let frame_time_delta_f = frame_time_delta.as_nanos() as f32 / 1_000_000_000.0f32;
        let elapsed_time_f = elapsed_time.as_nanos() as f32 / NSEC_PER_SEC as f32;
        let mut camera_animation_finished = false;
        if let Some(camera_animation) = camera_animation {
            // Check if animation has expired
            if elapsed_time_f >= camera_animation.end_time() {
                camera.pos = camera_animation.end_pos;
                camera.pitch_q = camera_animation.end_pitch_q;
                camera.yaw_q = camera_animation.end_yaw_q;
                camera_animation_finished = true;
            } else {
                let (pos, yaw_q, pitch_q) = camera_animation.at(elapsed_time_f);
                camera.pos = pos;
                camera.pitch_q = pitch_q;
                camera.yaw_q = yaw_q;
                camera_animation_finished = false;
            }
        } else {
            camera.rotate((-d_yaw, d_pitch));
        }
        if camera_animation_finished {
            *camera_animation = None;
        }
        let camera_speed = 3.0 * frame_time_delta_f;
        for keycode in pressed_keys {
            match keycode {
                VirtualKeyCode::W => camera.pos += camera_speed * camera.direction().unwrap(),
                VirtualKeyCode::S => camera.pos -= camera_speed * camera.direction().unwrap(),
                VirtualKeyCode::A => {
                    let delta = camera_speed * (Vector3f::cross(&camera.direction(), &camera.up()));
                    camera.pos -= delta;
                }
                VirtualKeyCode::D => {
                    let delta = camera_speed * (Vector3f::cross(&camera.direction(), &camera.up()));
                    camera.pos += delta;
                }
                _ => (),
            }
        }

        let mut vertices = vec![];
        for (transform, geometry) in (&transform_storage, &mut geometry).join() {
            vertices.extend(geometry.vtx_data(&transform.transform));
        }

        renderer
            .draw_frame(&game_state, &RenderData { vertices }, *resized)
            .expect("draw_frame()");
    }
}
