#[allow(dead_code)]
fn needs_sync<T: Sync>() {}

#[test]
fn event_loop_proxy_send() {
    #[allow(dead_code)]
    fn is_send<T: 'static + Send>() {
        // ensures that `slint-modal-winit::EventLoopProxy<T: Send>` implements `Sync`
        needs_sync::<slint-modal-winit::event_loop::EventLoopProxy<T>>();
    }
}

#[test]
fn window_sync() {
    // ensures that `slint-modal-winit::Window` implements `Sync`
    needs_sync::<slint-modal-winit::window::Window>();
}

#[test]
fn window_builder_sync() {
    needs_sync::<slint-modal-winit::window::WindowAttributes>();
}

#[test]
fn custom_cursor_sync() {
    needs_sync::<slint-modal-winit::window::CustomCursorSource>();
    needs_sync::<slint-modal-winit::window::CustomCursor>();
}
