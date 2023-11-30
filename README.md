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
        dice_number: 2,
        dice_type: 20,
        modifier: 3,
    };
    let request_d12 = RollRequest {
        dice_number: 3,
        dice_type: 12,
        modifier: -3,
    };

    let dice_pool: Vec<RollRequest> = vec![request_d20, request_d12];

    let results = roll_request(dice_pool); //returns a vector of RollResult

    for roll in results {
        println!(
            "{}d{} + {}:",
            roll.dice_number, roll.dice_type, roll.modifier
        );
        for i in roll.roll_result {
            println!("Roll {}: {} ({})", i.roll_number, i.roll, i.dice_type);
        }
        println!("Sum w/ modifier: {}", roll.sum)
    }
}

```
