pub mod rolltypes;
use crate::rolltypes::*;
use rand::Rng;

fn roll(req: &RollRequest) -> Vec<DiceRollResult> {
    let mut roll: Vec<DiceRollResult> = Vec::new();
    for n in 0..req.dice_qnt {
        let num_roll = n + 1;
        let create_roll: DiceRollResult = DiceRollResult {
            dice_type: req.dice_type,
            roll_number: num_roll,
            roll: rand::thread_rng().gen_range(1..req.dice_type),
        };
        roll.push(create_roll)
    }
    roll
}

fn roll_sum(results: &Vec<DiceRollResult>, modifier: i32) -> i32 {
    let mut sum = modifier;
    for roll in results {
        sum += roll.roll;
    }
    sum
}
pub fn roll_request(req: Vec<RollRequest>) -> Vec<RollResult> {
    let mut roll_result: Vec<RollResult> = Vec::new();

    for r in req {
        let roll_res: Vec<DiceRollResult> = roll(&r);
        let psum = roll_sum(&roll_res, r.modifier);
        let create_result: RollResult = RollResult {
            dice_type: r.dice_type,
            dice_qnt: r.dice_qnt,
            rolls: roll_res,
            modifier: r.modifier,
            sum: psum,
        };
        roll_result.push(create_result)
    }

    roll_result
}
