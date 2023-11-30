# dicerollerlib

## Description

This is a simple library for dice rolling

### Features

- Deals with any dice type
- Capable of roll any dice pool
- Can deal with multiple dicepools at the same time
- Modifiers can be added to any dice roll

## Installing

Simple as

```bash
cargo add --git https://github.com/Tavola-Redonda/dicerollerlib
```

## Usage

Import the lib into your project

```rust
use dicerollerlib;
```

The main function to use is `roll_request`. This function recives a `Vec<RollRequest>` as parameter and returns a `Vec<RollResult>`

### Example:

```rust
use dicerollerlib::dicelib::{roll_request, RollRequest};

fn main() {
    let request_d20 = RollRequest {
        dice_qnt: 2,
        dice_type: 20,
        modifier: 3,
    };
    let request_d12 = RollRequest {
        dice_qnt: 3,
        dice_type: 12,
        modifier: -3,
    };

    let dice_pool: Vec<RollRequest> = vec![request_d20, request_d12];

    let results = roll_request(dice_pool); //returns a vector of RollResult

    for pool in results {
        println!(
            "{}d{} + {}:",
            pool.dice_qnt, pool.dice_type, pool.modifier
        );
        for roll in pool.rolls {
            println!("Roll {}: {} ({})", rool.roll_number, roll.roll, roll.dice_type);
        }
        println!("Sum w/ modifier: {}", pool.sum)
    }
}
```

## Types

### Input Types

The basic input type is `RollRequest` and represents the parameters to roll a pool of dices

```rust
struct RollRequest {
  dice_qnt:i32, // Number of dices in this pull
  dice_type: i32, // type of dice to be rolled, can be any integer
  modifer: i32 // modifier
};
```

- `dice_qnt`: Number of dices in the pull. Can be any positive integer. Will determine the `len()` of `RollResult.rolls` for this pool
- `dice_type`: Dice type. Can be any positive integer, dices will always be rolled considering 1 as the minimum result and `dice_type` as the maximum results
- `modifer`: Can be any integer, will be added or subtracted from the sum of all rolls

### Return Types

#### DiceRollResult

The result of a particular roll

```rust
pub struct DiceRollResult {
  pub dice_type: i32,
  pub roll_number: i32,
  pub roll: i32,
}
```

- `dice_type`: Dice type of the roll
- `roll_number`: Number of the roll relative to the pool
- `roll`: Random number between 1 and `dice_type`, also know as the result of a roll_number

#### RollResult

The end result of a deice pool

```rust
pub struct RollResult {
  pub dice_type: i32,
  pub dice_qnt: i32,
  pub modifier: i32,
  pub rolls: Vec<DiceRollResult>,
  pub sum: i32,
}
```

- `dice_type`: Dice type of the pool
- `dice_qnt`: Number of dices rolled
- `modifier`: Modifier of the pool
- `rolls`: A `Vec<DiceRollResult>` (see [DiceRollResult](#dicerollresult)). Represents all the rolls in a particular dice pool
- `sum`: The sum of all rolls +/- modifier
