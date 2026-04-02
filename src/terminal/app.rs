use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::prelude::JsValue;
use web_sys::EventTarget;

use crate::terminal::core::{Action, TerminalCore};
use crate::terminal::input;
use crate::terminal::link::TerminalEffectHandler;
use crate::terminal::socials::SocialLinks;
use crate::terminal::ui::TerminalUi;

pub struct TerminalApp {
    core: RefCell<TerminalCore>,
    ui: TerminalUi,
    _socials: SocialLinks,
    effects: TerminalEffectHandler,
}

pub struct MountedTerminalApp {
    _app: Rc<TerminalApp>,
    _bindings: input::Bindings,
}

impl TerminalApp {
    pub fn new(
        ui: TerminalUi,
        socials: SocialLinks,
        effects: TerminalEffectHandler,
        core: TerminalCore,
    ) -> Self {
        socials.apply(core.links());
        Self {
            core: RefCell::new(core),
            ui,
            _socials: socials,
            effects,
        }
    }

    pub fn mount(self) -> Result<MountedTerminalApp, JsValue> {
        self.render()?;

        let app = Rc::new(self);
        let bindings = input::bind(&app)?;

        Ok(MountedTerminalApp {
            _app: app,
            _bindings: bindings,
        })
    }

    pub fn dispatch(&self, action: Action) -> Result<bool, JsValue> {
        let update = self.core.borrow_mut().dispatch(action);
        let state_changed = update.state_changed();
        if state_changed {
            self.render()?;
        }
        self.effects.apply(update.effects())?;
        Ok(state_changed)
    }

    fn render(&self) -> Result<(), JsValue> {
        let core = self.core.borrow();
        let view = core.view();
        self.ui.render(&view)
    }

    pub fn captures_keyboard(&self) -> bool {
        self.core.borrow().captures_keyboard()
    }

    pub fn take_input_value(&self) -> String {
        self.ui.take_input_value()
    }

    pub fn focus_input_if_prompt_enabled(&self) -> Result<(), JsValue> {
        if self.core.borrow().prompt_enabled() {
            self.ui.focus_input()?;
        }
        Ok(())
    }

    pub fn submit_target(&self) -> EventTarget {
        self.ui.submit_target()
    }

    pub fn shell_target(&self) -> EventTarget {
        self.ui.shell_target()
    }

    pub fn control_stick_target(&self) -> EventTarget {
        self.ui.control_stick_target()
    }

    pub fn control_stick_rect(&self) -> web_sys::DomRect {
        self.ui.control_stick_rect()
    }

    pub fn capture_control_pointer(&self, pointer_id: i32) -> Result<(), JsValue> {
        self.ui.capture_control_pointer(pointer_id)
    }

    pub fn release_control_pointer(&self, pointer_id: i32) -> Result<(), JsValue> {
        self.ui.release_control_pointer(pointer_id)
    }

    pub fn set_control_thumb_offset(&self, x: f64, y: f64) -> Result<(), JsValue> {
        self.ui.set_control_thumb_offset(x, y)
    }

    pub fn reset_control_thumb(&self) -> Result<(), JsValue> {
        self.ui.reset_control_thumb()
    }
}
