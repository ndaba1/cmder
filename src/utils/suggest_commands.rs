use std::collections::HashMap;

use crate::core::Command;

const MIN_MATCH_SIZE: i32 = 3;

pub fn suggest_cmd(val: &str, list: &[Command]) -> Option<String> {
    // TODO: Check for ambiguos matches
    let mut cmd_map = HashMap::new();

    for cmd in list {
        let name = cmd.get_name();
        cmd_map.insert(name, 0);
    }

    for v in val.chars() {
        for cmd in list {
            let name = cmd.get_name();
            let temp = cmd_map.clone();
            let count = temp.get(name).unwrap();
            if name.contains(v) {
                cmd_map.insert(name, count + 1);
            }
        }
    }

    let ans = &cmd_map.iter().find(|(_k, v)| **v >= MIN_MATCH_SIZE);

    ans.as_ref().map(|(k, _v)| k.to_string())
}
