#![cfg_attr(target_arch = "wasm32", allow(clippy::arc_with_non_send_sync))]

use std::{f32::consts::PI, sync::Arc};
use winit::{dpi::PhysicalPosition, event::ElementState};
use glam::Vec4Swizzles;
use super::camera::Camera;

pub struct State {
    pub pre_cursor_pos:PhysicalPosition<f32>,
    pub forward:bool,
    pub back:bool,
    pub left:bool,
    pub right:bool,
    pub camera:Camera,
}

impl State{
    pub fn new()->Self{
        Self{
            pre_cursor_pos:PhysicalPosition{x:955.0,y:515.0},
            forward:false,
            back:false,
            left:false,
            right:false,
            camera:Camera::new(),
        }
    }

    pub fn camera_init(&self,renderer:&Arc<rend3::Renderer>){
        let view_location = glam::Vec3::new(0.0, 0.0, -5.0);
        let view = glam::Mat4::from_euler(glam::EulerRot::XYZ, 0.0, 0.0, 0.0);
        let view = view * glam::Mat4::from_translation(-view_location);
    
        // Set camera's location
        renderer.set_camera_data(rend3::types::Camera {
            projection: rend3::types::CameraProjection::Perspective { vfov: 60.0, near: 0.1 },
            view,
        });
    }

    pub fn camera_euler_update(&mut self,position:&PhysicalPosition<f64>){
        let deff_euler_deg=glam::Vec2{
            y:((position.x as f32)-self.pre_cursor_pos.x)/300.0,
            x:((position.y as f32)-self.pre_cursor_pos.y)/300.0
        };
        let euler_x=self.camera.euler_x-deff_euler_deg.x;
        if (euler_x<=(PI/2.5)) && (euler_x>=(-PI/2.5)){
            self.camera.euler_x-=deff_euler_deg.x;
        }else if euler_x > (PI/2.5){
            self.camera.euler_x=PI/2.5;
        }else if euler_x < -(PI/2.5){
            self.camera.euler_x=-(PI/2.5);
        }
        self.camera.euler_y-=deff_euler_deg.y;
    }

    pub fn camera_update(&mut self,renderer:&Arc<rend3::Renderer>){
        self.camera.euler=glam::Mat4::from_euler(glam::EulerRot::XYZ, self.camera.euler_x, self.camera.euler_y, 0.0);
        if self.forward{
            let vel_mat=self.camera.euler*glam::Mat4::from_translation(glam::vec3(0.0,0.0,1.0));
            let vel=vel_mat.w_axis.xyz();
            let vel=vel.normalize()*0.2;
            self.camera.position.x-=vel.x;
            self.camera.position.z+=vel.z;
        }
        if  self.back{
            let vel_mat=self.camera.euler*glam::Mat4::from_translation(glam::vec3(0.0,0.0,1.0));
            let vel=vel_mat.w_axis.xyz();
            let vel=vel.normalize()*0.2;
            self.camera.position.x+=vel.x;
            self.camera.position.z-=vel.z;
        }
        if self.left{
            let vel_mat=glam::Mat4::from_rotation_y(PI/2.0)*self.camera.euler*glam::Mat4::from_translation(glam::vec3(0.0,0.0,1.0));
            let vel=vel_mat.w_axis.xyz();
            let vel=vel.normalize()*0.2;
            self.camera.position.x-=vel.x;
            self.camera.position.z+=vel.z;
        }
        if self.right{
            let vel_mat=glam::Mat4::from_rotation_y(-PI/2.0)*self.camera.euler*glam::Mat4::from_translation(glam::vec3(0.0,0.0,1.0));
            let vel=vel_mat.w_axis.xyz();
            let vel=vel.normalize()*0.2;
            self.camera.position.x-=vel.x;
            self.camera.position.z+=vel.z;
        }

        let view_location = self.camera.position;
        let view = self.camera.euler;
        let view = view * glam::Mat4::from_translation(-view_location);

        renderer.set_camera_data(rend3::types::Camera {
            projection: rend3::types::CameraProjection::Perspective { vfov: 60.0, near: 0.1 },
            view,
        });
    }

    pub fn key_event_handler(
        &mut self,
        event:winit::event::KeyEvent
    ){
        match event.state {
            ElementState::Pressed=>{
                if !event.repeat {
                    match event.physical_key {
                        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyW)=>{
                            self.forward=true;
                        },
                        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyA)=>{
                            self.left=true;
                        },
                        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyS)=>{
                            self.back=true;
                        },
                        winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyD)=>{
                            self.right=true;
                        },
                        _=>{}
                    }
                }    
            },
            ElementState::Released=>{
                match event.physical_key {
                    winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyW)=>{
                        self.forward=false;
                    },
                    winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyA)=>{
                        self.left=false;
                    },
                    winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyS)=>{
                        self.back=false;
                    },
                    winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyD)=>{
                        self.right=false;
                    },
                    _=>{}
                }
            },
        }
    }

}