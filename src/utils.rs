use amethyst::winit::{ElementState, Event, MouseButton, WindowEvent};

fn get_mouse_button(event: &Event) -> Option<(MouseButton, ElementState)> {
    match *event {
        Event::WindowEvent { ref event, .. } => match *event {
            WindowEvent::MouseInput {
                button,
                state,
                ..
            } => Some((button, state)),
            _ => None,
        },
        _ => None,
    }
}

pub fn is_mouse_button_pressed(event: &Event, button: MouseButton) -> bool {
    if let Some((pressed_button, state)) = get_mouse_button(event) {
        return pressed_button == button && state == ElementState::Pressed;
    } else {
        return false;
    }
}

pub fn is_mouse_button_released(event: &Event, button: MouseButton) -> bool {
    if let Some((released_button, state)) = get_mouse_button(event) {
        return released_button == button && state == ElementState::Released;
    } else {
        return false;
    }
}
