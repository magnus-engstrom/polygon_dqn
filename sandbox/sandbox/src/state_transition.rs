pub struct StateTransition {
    pub old_state: Vec<f64>,
    pub action: i32,
    pub new_state: Vec<f64>,
    pub reward: f64,
    pub done: bool,
}