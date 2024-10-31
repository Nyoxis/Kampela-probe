//! Everything high-level related to interfacing with user
use kampela_system::{
    devices::power::VoltageValues,
    draw::{DisplayOperationThreads, FrameBuffer},
    parallel::{AsyncOperation, Threads}
};
use crate::touch::get_touch_point;
use kampela_ui::uistate::{UIState, UpdateRequest, UpdateRequestMutate};

/// UI handler
pub struct UI {
    pub state: UIState<FrameBuffer>,
    update_request: Option<UpdateRequest>,
}

pub struct UIOperationThreads(Threads<UIStatus, 1>);

impl core::ops::Deref for UIOperationThreads {
    type Target = Threads<UIStatus, 1>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl core::ops::DerefMut for UIOperationThreads {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl UIOperationThreads {
    pub fn new() -> Self {
        Self(Threads::from([]))
    }
}

impl UI {
    fn listen(&mut self, threads: &mut UIOperationThreads, voltage: &VoltageValues) -> Option<bool> {
        if let Some(point) = get_touch_point() {
            self.update_request.propagate(self.state.handle_tap(point));
        }
        // update ui if needed
        if let Some(u) = self.update_request.take() {
            let is_clear_update = matches!(u, UpdateRequest::Slow) || matches!(u, UpdateRequest::Fast);
            self.update_request.propagate(self.state.render(is_clear_update, voltage).expect("guaranteed to work, no errors implemented"));

            let mut display_operation_threads = DisplayOperationThreads::new();
            match u {
                UpdateRequest::Hidden => (),
                UpdateRequest::Slow => display_operation_threads.request_full(),
                UpdateRequest::Fast => display_operation_threads.request_fast(),
                UpdateRequest::UltraFast => display_operation_threads.request_ultrafast(),
                UpdateRequest::UltraFastSelective => display_operation_threads.request_ultrafast_selective(),
                UpdateRequest::Part(a) => display_operation_threads.request_part(Some(a)),
            }
            if !matches!(u, UpdateRequest::Hidden) {
                threads.wind(UIStatus::DisplayOperation(display_operation_threads));
            }
            None
        } else {
            Some(true) // done operations
        }
    }
}

impl AsyncOperation for UI {
    type Init = ();
    type Input<'a> = (i32, &'a VoltageValues, &'a mut UIOperationThreads);
    type Output = Option<bool>;
    
    /// Start of UI.
    fn new(_: Self::Init) -> Self {
        let display = FrameBuffer::new_white();
        let state = UIState::new(display);
        return Self {
            state,
            update_request: Some(UpdateRequest::Slow),
        }
    }
    /// Call in event loop to progress through UI state
    fn advance<'a>(&mut self, (voltage, voltages, threads): Self::Input<'a>) -> Self::Output {
        match threads.advance_state() {
            UIStatus::Listen => {
                let a = self.listen(threads, voltages);
                if a.unwrap_or(false) {
                    //cortex_m::asm::wfi(); // sleep waiting for tocuh irq
                }
                a
            },
            UIStatus::DisplayOperation(t) => {
                let r = self.state.display.advance((voltage, t));
                if r == Some(true) {
                    threads.sync();
                }
                r
            },
        }
    }
}

/// General status of UI
///
/// There is no sense in reading input while screen processes last event, nor refreshing the screen
/// before touch was parsed
pub enum UIStatus {
    /// Event listening state, default
    Listen,
    /// Screen update started
    DisplayOperation(DisplayOperationThreads),
}
impl Default for UIStatus {
    fn default() -> Self { UIStatus::Listen }
}
