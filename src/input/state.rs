use crate::InputMapper;

impl InputMapper {
    pub fn get_previous_value(&self, action: &str) -> &f32 {
        self.previous_action_value
            .get(&action.to_owned())
            .unwrap_or(&0.)
    }
    pub fn get_current_value(&self, action: &str) -> &f32 {
        self.action_value.get(&action.to_owned()).unwrap_or(&0.)
    }
    pub fn is_started(&self, action: &str) -> bool {
        self.get_previous_value(action) == &0. && self.get_current_value(action) > &0.
    }
    pub fn is_continuing(&self, action: &str) -> bool {
        self.get_previous_value(action) > &0. && self.get_current_value(action) > &0.
    }
    pub fn is_finished(&self, action: &str) -> bool {
        self.get_previous_value(action) > &0. && self.get_current_value(action) == &0.
    }
    pub fn is_active(&self, action: &str) -> bool {
        self.is_started(action) || self.is_continuing(action) || self.is_finished(action)
    }
}
