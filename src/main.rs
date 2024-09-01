use raylib::prelude::*;
use state::{Elevator, Floor, State, StickFigure};

mod state;

fn controls(rl: &mut RaylibHandle, state: &mut State) {
    let mouse_pos = rl.get_mouse_position();
    let mouse_delta = rl.get_mouse_delta();

    let elevator_rect = state.elevator.rectangle();

    // Click & drag the elevator
    if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT)
        && elevator_rect.check_collision_point_rec(&(mouse_pos - mouse_delta))
    {
        state.elevator.y = (rl.get_mouse_y() - 40) as f32;
    } else if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
        for floor in &state.floors {
            if floor.y as f32 >= mouse_pos.y {
                state.elevator.set_target(floor.y - 78);
                break;
            }
        }
    }
}

fn calculate(rl: &mut RaylibHandle, state: &mut State) {
    if let Some(y_target) = state.elevator.y_target {
        let y_target = y_target as f32;
        let target_direction_sign = -(state.elevator.y_old - y_target).signum();

        let total_diff = (state.elevator.y_old - y_target).abs();
        let current_diff = (state.elevator.y - y_target).abs();

        let mut velocity_diff: f32 = 0.01;

        // Jank to slow down the elevator as it approaches target
        if current_diff < 30.0 {
            if total_diff <= 30.0 {
            } else if total_diff <= 60.0 {
                velocity_diff = -0.0005;
            } else if total_diff <= 80.0 {
                velocity_diff = -0.005;
            } else if total_diff <= 86.0 {
                velocity_diff = -0.0175;
            } else if total_diff <= 120.0 {
                velocity_diff = -0.0175;
            } else if total_diff <= 171.0 {
                velocity_diff = -0.030125;
            } else {
                velocity_diff = -0.032125;
            }
        }

        // Put velocity in right direction (up/down) and increase effect if we're going the wrong way
        velocity_diff *= target_direction_sign;
        if state.elevator.y_velocity.signum() != target_direction_sign {
            velocity_diff *= 2.0;
        }

        state.elevator.y_velocity += velocity_diff;

        // Cap velocity
        if state.elevator.y_velocity > 1.4 {
            state.elevator.y_velocity = 1.4;
        } else if state.elevator.y_velocity < -1.4 {
            state.elevator.y_velocity = -1.4;
        }

        // Apply velocity to y position
        state.elevator.y += state.elevator.y_velocity;

        // Stop elevator if it reaches its target (slow down math isn't perfect)
        if (target_direction_sign == 1.0 && state.elevator.y >= y_target)
            || (target_direction_sign == -1.0 && state.elevator.y <= y_target)
        {
            state.elevator.y_velocity = 0.0;
            state.elevator.y_target = None;
        }
    }

    for stick_figure in state.stick_figures.iter_mut() {
        if stick_figure.in_elevator {
            stick_figure.y = state.elevator.y + 44.0;
            stick_figure.x = 75.0;
        }

        if let Some(walking_state) = stick_figure.walking_state {
            stick_figure.walking_state = Some((walking_state + 1) % 30);
        }

        stick_figure.x += stick_figure.walking_direction as f32;

        if !stick_figure.in_elevator {
            if stick_figure.x <= 105.0 || stick_figure.x >= 765.0 {
                stick_figure.walking_direction *= -1;
            } else if rl.get_random_value::<i32>(0..200) == 10 {
                stick_figure.walking_direction *= -1;
            }
        }
    }
}

fn draw_elevator(d: &mut RaylibDrawHandle, elevator: &Elevator) {
    let pos_x = 50.0;
    let pos_y = elevator.y;

    // Light fixture
    d.draw_triangle(
        Vector2::new(pos_x + 15.0, pos_y + 12.0),
        Vector2::new(pos_x + 35.0, pos_y + 12.0),
        Vector2::new(pos_x + 25.0, pos_y + 4.0),
        Color::ORANGE,
    );

    // Elevator rectangle
    d.draw_rectangle_lines_ex(
        Rectangle::new(pos_x, pos_y, 50.0, 80.0),
        5.0,
        Color::WHITESMOKE,
    );

    // "Control Panel" on left
    d.draw_line_ex(
        Vector2::new(pos_x + 3.0, pos_y + 35.0),
        Vector2::new(pos_x + 3.0, pos_y + 60.0),
        2.0,
        Color::LIGHTSTEELBLUE,
    );

    // Black out door
    d.draw_line_ex(
        Vector2::new(pos_x + 47.0, pos_y + 17.0),
        Vector2::new(pos_x + 47.0, pos_y + 17.0 + 58.0 * elevator.door),
        6.0,
        Color::BLACK,
    );
}

fn draw_floor(d: &mut RaylibDrawHandle, floor: &Floor) {
    d.draw_line_ex(
        Vector2::new(102.0, *&floor.y as f32),
        Vector2::new(768.0, *&floor.y as f32),
        5.0,
        Color::WHITE,
    );
}

fn draw_stick_figure(d: &mut RaylibDrawHandle, stick_figure: &StickFigure) {
    let x_center = stick_figure.x;
    let circle_bottom_y = stick_figure.y + 7.0;

    d.draw_circle(
        stick_figure.x as i32,
        stick_figure.y as i32,
        7.0,
        Color::WHITE,
    );
    d.draw_circle(
        stick_figure.x as i32,
        stick_figure.y as i32,
        5.0,
        Color::BLACK,
    );

    d.draw_line_ex(
        Vector2::new(x_center, circle_bottom_y),
        Vector2::new(x_center, circle_bottom_y + 15.0),
        2.0,
        Color::WHITE,
    );

    let mut arms = vec![
        (Vector2::new(x_center, circle_bottom_y + 2.0), Vector2::new(x_center - 7.0, circle_bottom_y + 12.0)),
        (Vector2::new(x_center, circle_bottom_y + 2.0), Vector2::new(x_center + 7.0, circle_bottom_y + 12.0)),
    ];

    let mut legs = vec![
        (Vector2::new(x_center, circle_bottom_y + 15.0), Vector2::new(x_center - 7.0, circle_bottom_y + 24.0)),
        (Vector2::new(x_center, circle_bottom_y + 15.0), Vector2::new(x_center + 7.0, circle_bottom_y + 24.0)),
    ];

    if let Some(walking_state) = stick_figure.walking_state {
        if walking_state < 7 {
            arms[0].1.x += 4.0;
            arms[0].1.y += 4.0;
            arms[1].1.x -= 4.0;
            arms[1].1.y += 4.0;

            legs[0].1.x += 2.0;
            legs[0].1.y += 2.0;
            legs[1].1.x -= 2.0;
            legs[1].1.y += 2.0;
        } else if walking_state < 15 {
            arms[0].1.x += 2.0;
            arms[0].1.y += 2.0;
            arms[1].1.x -= 2.0;
            arms[1].1.y += 2.0;

            legs[0].1.x += 4.0;
            legs[0].1.y += 4.0;
            legs[1].1.x -= 4.0;
            legs[1].1.y += 4.0;
        }
    }

    for (arm_start, arm_end) in arms {
        d.draw_line_ex(arm_start, arm_end, 2.0, Color::WHITE);
    }

    for (leg_start, leg_end) in legs {
        d.draw_line_ex(leg_start, leg_end, 2.0, Color::WHITE);
    }
}

fn draw(rl: &mut RaylibHandle, thread: &RaylibThread, state: &State) {
    let mut d = rl.begin_drawing(thread);

    d.clear_background(Color::BLACK);

    draw_elevator(&mut d, &state.elevator);

    for floor in &state.floors {
        draw_floor(&mut d, &floor);
    }

    for stick_figure in &state.stick_figures {
        draw_stick_figure(&mut d, stick_figure);
    }
}

fn main() {
    let mut state = State::new();

    state.elevator.set_target(4 * 85 + 28);

    state.stick_figures.push({
        let mut sm = StickFigure::new(0.0, 0.0);
        sm.in_elevator = true;
        sm.walking_state = None;
        sm
    });

    state.stick_figures.push({
        let mut sm = StickFigure::new(150.0, state.floors[4].y as f32 - 35.0);
        sm.in_elevator = false;
        sm.walking_state = Some(0);
        sm.walking_direction = 1;
        sm
    });

    let (mut rl, thread) = raylib::init()
        .title("Elevator Simulator")
        .resizable()
        .size(1024, 740)
        .build();

    rl.set_target_fps(60);

    while !rl.window_should_close() {
        controls(&mut rl, &mut state);
        calculate(&mut rl, &mut state);
        draw(&mut rl, &thread, &state);
    }
}

// Elevator cube world
// Shake elevator and they throw up
// they walk around, click buttons, talk to each other
// Clicking on them should make them do things
