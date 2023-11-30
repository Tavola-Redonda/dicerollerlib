pub mod commandparser {
    use crate::dicetype::RollRequest;
    use regex::Regex;
    fn create_roll_request(cmd: &str) -> RollRequest {
        let regexsplit = Regex::new("[^0-9]").unwrap();
        let mut parsed: Vec<&str> = regexsplit.split(cmd).collect();

        if parsed.len() < 3 {
            parsed.push("0")
        };

        let mut pool_modifier: i32 = parsed[2].parse().unwrap();
        if cmd.contains('-') {
            pool_modifier *= -1
        }

        RollRequest {
            dice_qnt: parsed[0].parse().unwrap(),
            dice_type: parsed[1].parse().unwrap(),
            modifier: pool_modifier,
        }
    }

    pub fn cmd_roll(commands: &Vec<String>) -> Vec<RollRequest> {
        let mut parse_args: Vec<&String> = Vec::new();
        for a in 0..commands.len() - 1 {
            parse_args.push(&commands[a + 1]);
        }
        let mut requests: Vec<RollRequest> = Vec::new();
        for command in parse_args {
            let cmd = command.as_str();
            let req = create_roll_request(cmd);
            requests.push(req);
        }
        requests
    }
}
