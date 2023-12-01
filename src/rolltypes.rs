#[derive(Debug)]
pub enum Operators {
    Div,
    Mult,
    Sum,
    Sub,
}
#[derive(Debug)]
pub struct ModifierOperator {
    pub operator: Operators,
    pub number: f32,
}

#[derive(Debug)]
pub struct RollRequest {
    pub dice_type: i32,
    pub dice_qnt: i32,
    pub modifier: Option<Vec<ModifierOperator>>,
}

#[derive(Debug)]
pub struct DiceRollResult {
    pub dice_type: i32,
    pub roll_number: i32,
    pub roll: i32,
}

#[derive(Debug)]
pub struct RollResult {
    pub pool: String,
    pub dice_type: i32,
    pub dice_qnt: i32,
    pub modifier: Option<Vec<ModifierOperator>>,
    pub rolls: Vec<DiceRollResult>,
    pub sum: f32,
}
