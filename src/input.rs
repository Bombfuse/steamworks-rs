use super::*;

#[derive(Copy, Clone)]
pub struct InputDigitalActionData {
    pub state: bool, // The current state of this action; true if the action is currently pressed, otherwise false.
    pub active: bool, // Whether or not this action is currently available to be bound in the active action set.
}

#[derive(Clone, Copy)]
pub struct InputDigitalActionHandle(pub(crate) u64);

#[derive(Clone, Copy)]
pub struct InputActionSetHandle(pub(crate) u64);

pub struct Input<Manager> {
    pub(crate) input: *mut sys::ISteamInput,
    pub(crate) inner: Arc<Inner<Manager>>,
}
impl<Manager> Input<Manager> {
    pub fn get_digital_action_handle<T: Into<String>>(&self, action_name: T) -> InputDigitalActionHandle {
        unsafe {
            let action_name: String = action_name.into();
            let action_name = CString::new(action_name).unwrap();
            let digital_action_handle = sys::SteamAPI_ISteamInput_GetDigitalActionHandle(self.input, action_name.as_ptr());

            InputDigitalActionHandle(digital_action_handle)
        }
    }

    pub fn get_digital_action_data(&self, controller_id: u64, action_handle: InputDigitalActionHandle) -> InputDigitalActionData {
        unsafe {
            let digital_action_data = sys::SteamAPI_ISteamInput_GetDigitalActionData(self.input, controller_id, action_handle.0);
            let state = digital_action_data.bActive;
            let active = digital_action_data.bState;
            InputDigitalActionData { state, active }
        }
    }

    pub fn get_action_set_handle<T: Into<String>>(&self, action_set_name: T) -> InputActionSetHandle {
        unsafe {
            let action_set_name: String = action_set_name.into();
            let action_set_name = CString::new(action_set_name).unwrap();
            let action_set_handle = sys::SteamAPI_ISteamInput_GetActionSetHandle(self.input, action_set_name.as_ptr());

            InputActionSetHandle(action_set_handle)
        }
    }

    pub fn activate_action_set_layer(&self, controller_id: u64, action_set_handle: InputActionSetHandle) {
        unsafe {
            sys::SteamAPI_ISteamInput_ActivateActionSetLayer(self.input, controller_id, action_set_handle.0);
        }
    }
    
    pub fn deactivate_action_set_layer(&self, controller_id: u64, action_set_handle: InputActionSetHandle) {
        unsafe {
            sys::SteamAPI_ISteamInput_DeactivateActionSetLayer(self.input, controller_id, action_set_handle.0);
        }
    }
}