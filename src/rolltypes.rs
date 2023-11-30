#[derive(Debug)]
pub struct RollRequest {
    pub dice_type: i32,
    pub dice_qnt: i32,
    pub modifier: i32,
}

#[derive(Debug)]
pub struct DiceRollResult {
    pub dice_type: i32,
    pub roll_number: i32,
    pub roll: i32,
}

#[derive(Debug)]
pub struct RollResult {
    pub dice_type: i32,
    pub dice_qnt: i32,
    pub modifier: i32,
    pub rolls: Vec<DiceRollResult>,
    pub sum: i32,
}
