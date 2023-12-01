pub mod rolltypes;
use core::f32;
use std::ops::Add;

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

fn roll_sum(results: &Vec<DiceRollResult>, modifiers: &Vec<ModifierOperator>) -> f32 {
    let mut pool_res: f32 = 0.0;
    for roll in results {
        pool_res += roll.roll as f32
    }
    for op in modifiers {
        let get_operator = &op.operator;
        match get_operator {
            Operators::Sum => pool_res += op.number,
            Operators::Sub => pool_res -= op.number,
            Operators::Div => pool_res /= op.number,
            Operators::Mult => pool_res *= op.number,
        }
    }
    pool_res
}
fn generate_pool_string(req: &RollRequest) -> String {
    let mut modifer_list: Vec<String> = Vec::new();
    for m in &req.modifier {
        let mod_str = match m.operator {
            Operators::Sum => "+".to_string().add(&m.number.to_string()),
            Operators::Sub => "-".to_string().add(&m.number.to_string()),
            Operators::Mult => "*".to_string().add(&m.number.to_string()),
            Operators::Div => "÷".to_string().add(&m.number.to_string()),
        };
        modifer_list.push(mod_str);
    }
    let pool_string = format!(
        "{n}d{d}{o}",
        n = req.dice_qnt,
        d = req.dice_type,
        o = modifer_list.join("")
    );
    pool_string
}

pub fn roll_request(req: Vec<RollRequest>) -> Vec<RollResult> {
    let mut roll_result: Vec<RollResult> = Vec::new();

    for r in req {
        let roll_res: Vec<DiceRollResult> = roll(&r);
        let pool_result = roll_sum(&roll_res, &r.modifier);
        let pool_str: String = generate_pool_string(&r);
        let create_result: RollResult = RollResult {
            pool: pool_str,
            dice_type: r.dice_type,
            dice_qnt: r.dice_qnt,
            rolls: roll_res,
            modifier: r.modifier,
            sum: pool_result,
        };
        roll_result.push(create_result)
    }

    roll_result
}
