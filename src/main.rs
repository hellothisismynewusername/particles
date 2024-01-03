use raylib::prelude::*;
use raylib::ffi::DrawCircle;
//use raylib::ffi::DrawCircleLines;
use raylib::ffi::MouseButton::MOUSE_LEFT_BUTTON;

#[derive(Clone)]
struct Vector2 {
    x : f64,
    y : f64
}

#[derive(Clone)]
struct Node {
    pos : Vector2,
    vel : Vector2,
    acc : Vector2,
    mass: f64,
    radius: f64,
    forces : Vec<Force>,
    influence_radius : f64,
    ball_type: u8,
    speed_soft_cap: f64
}

#[derive(Clone)]
struct Force {
    angle : f64,
    magnitude : f64
}

fn mouse_drag_and_drop(rl : &RaylibHandle, nodes : &mut Vec<Node>, mouse_has_target : &mut bool, mouse_target : &mut usize) {
    for i in 0..nodes.len() {
        if !*mouse_has_target {
            if rl.is_mouse_button_down(MOUSE_LEFT_BUTTON) {
                if rl.get_mouse_position().x >= nodes[i].pos.x as f32 - nodes[i].radius as f32 &&
                rl.get_mouse_position().x <= nodes[i].pos.x as f32 + nodes[i].radius as f32 &&
                rl.get_mouse_position().y >= nodes[i].pos.y as f32 - nodes[i].radius as f32 &&
                rl.get_mouse_position().y <= nodes[i].pos.y as f32 + nodes[i].radius as f32 {
                    nodes[i].pos.x = rl.get_mouse_position().x as f64;
                    nodes[i].pos.y = rl.get_mouse_position().y as f64;
                    nodes[i].vel.x = 0.;
                    nodes[i].vel.y = 0.;
                    nodes[i].acc.x = 0.;
                    nodes[i].acc.y = 0.;
                    *mouse_has_target = true;
                    *mouse_target = i;
                }
            }
        } else {
            nodes[*mouse_target].pos.x = rl.get_mouse_position().x as f64;
            nodes[*mouse_target].pos.y = rl.get_mouse_position().y as f64;
            if rl.is_mouse_button_up(MOUSE_LEFT_BUTTON) {
                *mouse_has_target = false;
            }
        }
    }
}

fn main() {
    const PI : f64 = 3.14159265358979323846264338327950288f64;
    //const SCREEN_WIDTH : u64 = 1280;
    //const SCREEN_HEIGHT : u64 = 720;
    const SCREEN_WIDTH : u64 = 1800;
    const SCREEN_HEIGHT : u64 = 1000;
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32)
        .title("fizix")
        .vsync()
        .build();

    let mut mouse_has_target = false;
    let mut mouse_target : usize = 0;
    let time_step = 0.005;
    let update_vels = true;
    let mut nodes = Vec::new();
    let use_speed_cap = true;
    let _zero_zero = Vector2 {
        x: 0.0,
        y: 0.0
    };

    /*
    for i in 0..10 {
        nodes.push(Node {
            pos: Vector2 {x: (i as f64 * 20.) + 50., y: 500.},
            vel: Vector2 {x: 0., y: 0.},
            acc: Vector2 {x: 0., y: 0.},
            mass: 8.,
            radius: 15.,
            forces: Vec::new(),
            influence_radius: 150.,
            ball_type: 1,
            speed_soft_cap: 1500.
        });
    }

    for i in 0..100 {
        nodes.push(Node {
            pos: Vector2 {x: (i as f64 * 2.) + 20., y: (i as f64 * 0.5) + 20.},
            vel: Vector2 {x: 0., y: 0.},
            acc: Vector2 {x: 0., y: 0.},
            //mass: ((i % 10) + 1) as f64,
            mass: 3.,
            radius: 10.,
            forces: Vec::new(),
            influence_radius: 150.,
            ball_type: 0,
            speed_soft_cap: 1500.
        });
    }
    */

    for i in 0..20 {
        nodes.push(Node {
            pos: Vector2 {x: (i as f64 * 20.) + 50., y: 500.},
            vel: Vector2 {x: 0., y: 0.},
            acc: Vector2 {x: 0., y: 0.},
            mass: 8.,
            radius: 15.,
            forces: Vec::new(),
            influence_radius: 150.,
            ball_type: 1,
            speed_soft_cap: 1500.
        });
    }

    for i in 0..100 {
        nodes.push(Node {
            pos: Vector2 {x: (i as f64 * 15.) + 20., y: 20.},
            vel: Vector2 {x: 0., y: 0.},
            acc: Vector2 {x: 0., y: 0.},
            //mass: ((i % 10) + 1) as f64,
            mass: 3.,
            radius: 10.,
            forces: Vec::new(),
            influence_radius: 150.,
            ball_type: 0,
            speed_soft_cap: 1500.
        });
    }
    for i in 0..100 {
        nodes.push(Node {
            pos: Vector2 {x: (i as f64 * 15.) + 20., y: 100.},
            vel: Vector2 {x: 0., y: 0.},
            acc: Vector2 {x: 0., y: 0.},
            //mass: ((i % 10) + 1) as f64,
            mass: 3.,
            radius: 10.,
            forces: Vec::new(),
            influence_radius: 150.,
            ball_type: 0,
            speed_soft_cap: 1500.
        });
    }
    for i in 0..100 {
        nodes.push(Node {
            pos: Vector2 {x: (i as f64 * 15.) + 20., y: 180.},
            vel: Vector2 {x: 0., y: 0.},
            acc: Vector2 {x: 0., y: 0.},
            //mass: ((i % 10) + 1) as f64,
            mass: 3.,
            radius: 10.,
            forces: Vec::new(),
            influence_radius: 150.,
            ball_type: 0,
            speed_soft_cap: 1500.
        });
    }

    while !rl.window_should_close() {
        let oldnodes = nodes.clone();
        mouse_drag_and_drop(&rl, &mut nodes, &mut mouse_has_target, &mut mouse_target);
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        for i in 0..nodes.len() {
            nodes[i].forces.clear();

            
            nodes[i].forces.push(Force { //gravity
                angle: 4.71238898,
                magnitude: oldnodes[i].mass * 9.8
            });
            

            if nodes[i].pos.y + nodes[i].radius >= SCREEN_HEIGHT as f64 {
                nodes[i].vel.y *= -1.;
                nodes[i].pos.y = SCREEN_HEIGHT as f64 - nodes[i].radius;
            }
            if nodes[i].pos.y - nodes[i].radius <= 0. {
                nodes[i].vel.y *= -1.;
                nodes[i].pos.y = nodes[i].radius;
            }
            if nodes[i].pos.x + nodes[i].radius >= SCREEN_WIDTH as f64 {
                nodes[i].vel.x *= -1.;
                nodes[i].pos.x = SCREEN_WIDTH as f64 - nodes[i].radius;
            }
            if nodes[i].pos.x - nodes[i].radius <= 0. {
                nodes[i].vel.x *= -1.;
                nodes[i].pos.x = nodes[i].radius;
            }
            

            for j in 0..oldnodes.len() {
                if i != j {
                    { //ball on ball physical collisions
                        let x_dist = oldnodes[i].pos.x - oldnodes[j].pos.x;
                        let y_dist = oldnodes[i].pos.y - oldnodes[j].pos.y;
                        let dist = ((x_dist * x_dist) + (y_dist * y_dist)).sqrt();
                        if dist <= oldnodes[i].radius + oldnodes[j].radius {
                            
                            let vel1 = ((oldnodes[i].vel.x * oldnodes[i].vel.x) + (oldnodes[i].vel.y * oldnodes[i].vel.y)).sqrt();
                            let vel2 = ((oldnodes[j].vel.x * oldnodes[j].vel.x) + (oldnodes[j].vel.y * oldnodes[j].vel.y)).sqrt();

                            /*
                            let angle = (y_dist * -1.).atan2(x_dist);
                            let v1x_final = (oldnodes[j].mass * oldnodes[i].vel.y - oldnodes[i].mass * oldvel * angle.tan()) / (oldnodes[i].mass * angle.tan());
                            let v1y_final = angle.tan() * v1x_final;
                            let v2x_final = (oldnodes[i].mass * oldnodes[i].vel.x - oldnodes[j].mass * oldnodes[j].vel.x) / oldnodes[j].mass;
                            let v2y_final = (oldnodes[i].mass * oldnodes[i].vel.y - oldnodes[j].mass * oldnodes[j].vel.y) / oldnodes[j].mass;

                            update_vels = false;
                            collidable = false;
                            nodes[i].vel.x = v1x_final;
                            nodes[i].vel.y = v1y_final;
                            nodes[j].vel.x = v2x_final;
                            nodes[j].vel.y = v2y_final;
                            */
                            
                            nodes[i].forces.push(Force {
                                angle: (y_dist * -1.).atan2(x_dist),
                                magnitude: 2. * (vel1 + vel2) / (oldnodes[i].mass)
                            });
                            //if i == 1 { collidable = false; }
                        }
                    }

                    { //force fields
                        let x_dist = oldnodes[i].pos.x - oldnodes[j].pos.x;
                        let y_dist = oldnodes[i].pos.y - oldnodes[j].pos.y;
                        let dist = ((x_dist * x_dist) + (y_dist * y_dist)).sqrt();
                        if dist <= oldnodes[i].influence_radius + oldnodes[j].radius {
                            if oldnodes[i].ball_type == 0 && oldnodes[j].ball_type == 1 {
                                let angle = (y_dist * -1.).atan2(x_dist) + PI;

                                nodes[i].forces.push(Force {
                                    angle: angle,
                                    magnitude: 5500. / (oldnodes[i].mass)
                                });
                            }
                            if oldnodes[i].ball_type == 1 && oldnodes[j].ball_type == 0 {
                                let angle = (y_dist * -1.).atan2(x_dist) + PI;

                                nodes[i].forces.push(Force {
                                    angle: angle,
                                    magnitude: 1000. / (oldnodes[i].mass)
                                });
                            }
                            if oldnodes[i].ball_type == 0 && oldnodes[j].ball_type == 0 {
                                let angle = (y_dist * -1.).atan2(x_dist);

                                nodes[i].forces.push(Force {
                                    angle: angle,
                                    magnitude: 1000. / (oldnodes[i].mass)
                                });
                            }
                            if oldnodes[i].ball_type == 1 && oldnodes[j].ball_type == 1 {
                                let angle = (y_dist * -1.).atan2(x_dist);

                                nodes[i].forces.push(Force {
                                    angle: angle,
                                    magnitude: 10000. / (oldnodes[i].mass)
                                });
                            }
                        }
                    }
                }
            }

            let mut xacc : f64 = 0.;
            let mut yacc : f64 = 0.;
            for force in &nodes[i].forces {
                xacc += (force.angle.cos() * force.magnitude) / nodes[i].mass;
                yacc += (force.angle.sin() * force.magnitude) / nodes[i].mass;
            }
            nodes[i].acc.x = xacc;
            nodes[i].acc.y = yacc;

            if update_vels {
                nodes[i].vel.x += nodes[i].acc.x;
                nodes[i].vel.y += nodes[i].acc.y;
                if use_speed_cap {
                    let combined_vel = ((oldnodes[i].vel.x * oldnodes[i].vel.x) + (oldnodes[i].vel.y * oldnodes[i].vel.y)).sqrt();
                    if combined_vel > oldnodes[i].speed_soft_cap {
                        //println!("comb vel is {}", combined_vel);
                        let mut new_vel : f64 = 0.;
                        if combined_vel > 0. {
                            new_vel = combined_vel - 100.;
                        } else if combined_vel < 0. {
                            new_vel = combined_vel + 100.;
                        }
                        let angle = oldnodes[i].vel.y.atan2(oldnodes[i].vel.x);
                        nodes[i].vel.x = new_vel * angle.cos();
                        nodes[i].vel.y = new_vel * angle.sin();
                        //println!("new vel is {}", new_vel);
                    }
                }
            }
            nodes[i].pos.x += nodes[i].vel.x * time_step;
            nodes[i].pos.y -= nodes[i].vel.y * time_step;

            //let vel = ((nodes[i].vel.x * nodes[i].vel.x) + (nodes[i].vel.y * nodes[i].vel.y)).sqrt();

            unsafe {
                //DrawRectangle(nodes[i].pos.x as i32, nodes[i].pos.y as i32, nodes[i].size.x as i32, nodes[i].size.y as i32, Color::WHITE.into());
                if nodes[i].ball_type == 1 {
                    DrawCircle(nodes[i].pos.x as i32, nodes[i].pos.y as i32, nodes[i].radius as f32, Color {r: 255, g: 255, b: 255, a: 255}.into());
                } else {
                    DrawCircle(nodes[i].pos.x as i32, nodes[i].pos.y as i32, nodes[i].radius as f32, Color {r: 255, g: (255. * (nodes[i].mass / 11.)) as u8, b: 0, a: 255}.into());
                }
                //DrawCircleLines(nodes[i].pos.x as i32, nodes[i].pos.y as i32, nodes[i].influence_radius as f32, Color::RED.into());
            }
        }
    }
}