use vector::{ V2f32 };

#[derive(Copy)]
#[derive(Clone)]
pub struct InterpolateData {
    time_left: f32, // seconds left before interpolation should end
    goal: V2f32 // interpolation target
}

impl InterpolateData {
    pub fn new() -> InterpolateData {
        return InterpolateData{
            time_left: 0.0,
            goal: V2f32::new(0.0, 0.0)
        }
    }
}

// interpolate position towards goal
pub fn interpolate(interpolate_data: &mut InterpolateData, position: &mut V2f32, dt: f32) {
    if dt >= interpolate_data.time_left {
        interpolate_data.time_left = 0.0;
        *position = interpolate_data.goal;
    } else {
        let distance = interpolate_data.goal - *position;
        *position +=
            2.0f32 * distance / interpolate_data.time_left * dt - 
            distance / (interpolate_data.time_left * interpolate_data.time_left) * dt * dt;
            interpolate_data.time_left -= dt;
    }
}

pub fn set_goal(interpolate_data: &mut InterpolateData, goal: V2f32, duration: f32) {
    interpolate_data.time_left = duration;
    interpolate_data.goal = goal;
}