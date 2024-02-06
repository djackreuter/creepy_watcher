use std::collections::HashMap;

pub fn code_lookup(shift_pressed: bool, raw_code: u32, caps_toggle: bool) -> String {

    let code_map: HashMap<u32, String> = HashMap::from([
        (8, "<Backspace>".to_string()),
        (13, "\n".to_string()),
        (32, " ".to_string()),
        (37, "<Left Arrow>".to_string()),
        (38, "<Up Arrow>".to_string()),
        (39, "<Right Arrow>".to_string()),
        (40, "<Down Arrow>".to_string()),
        (48, "0".to_string()),
        (49, "1".to_string()),
        (50, "2".to_string()),
        (51, "3".to_string()),
        (52, "4".to_string()),
        (53, "5".to_string()),
        (54, "6".to_string()),
        (55, "7".to_string()),
        (56, "8".to_string()),
        (57, "9".to_string()),
        (65, "a".to_string()),
        (66, "b".to_string()),
        (67, "c".to_string()),
        (68, "d".to_string()),
        (69, "e".to_string()),
        (70, "f".to_string()),
        (71, "g".to_string()),
        (72, "h".to_string()),
        (73, "i".to_string()),
        (74, "j".to_string()),
        (75, "k".to_string()),
        (76, "l".to_string()),
        (77, "m".to_string()),
        (78, "n".to_string()),
        (79, "o".to_string()),
        (80, "p".to_string()),
        (81, "q".to_string()),
        (82, "r".to_string()),
        (83, "s".to_string()),
        (84, "t".to_string()),
        (85, "u".to_string()),
        (86, "v".to_string()),
        (87, "w".to_string()),
        (88, "x".to_string()),
        (89, "y".to_string()),
        (90, "z".to_string()),
        (96, "0".to_string()),
        (97, "1".to_string()),
        (98, "2".to_string()),
        (99, "3".to_string()),
        (100, "4".to_string()),
        (101, "5".to_string()),
        (102, "6".to_string()),
        (103, "7".to_string()),
        (104, "8".to_string()),
        (105, "9".to_string()),
        (106, "*".to_string()),
        (107, "+".to_string()),
        (109, "-".to_string()),
        (111, "/".to_string()),
        (189, "-".to_string()),
        (187, "=".to_string()),
        (219, "[".to_string()),
        (221, "]".to_string()),
        (220, "\\".to_string()),
        (186, ";".to_string()),
        (222, "'".to_string()),
        (188, ",".to_string()),
        (190, ".".to_string()),
        (191, "/".to_string()),
        (192, "`".to_string()),
    ]); 

    let code_map_shift: HashMap<u32, String> = HashMap::from([
        (48, ")".to_string()),
        (49, "!".to_string()),
        (50, "@".to_string()),
        (51, "#".to_string()),
        (52, "$".to_string()),
        (53, "%".to_string()),
        (54, "^".to_string()),
        (55, "&".to_string()),
        (56, "*".to_string()),
        (57, "(".to_string()),
        (189, "_".to_string()),
        (187, "+".to_string()),
        (219, "{".to_string()),
        (221, "}".to_string()),
        (220, "|".to_string()),
        (186, ":".to_string()),
        (222, "\"".to_string()),
        (188, "<".to_string()),
        (190, ">".to_string()),
        (191, "?".to_string()),
        (192, "~".to_string()),
    ]);

    if shift_pressed {
        match code_map_shift.get(&raw_code) {
            Some(val) => {
                return val.to_string();
            }
            None => {
                if code_map.get(&raw_code).is_some() {
                    return code_map.get(&raw_code).unwrap().to_uppercase();
                }
            }
        }
    }

    match code_map.get(&raw_code) {
        Some(val) => {
            if caps_toggle {
                return val.to_uppercase();
            }
            return val.to_string();
        }
        None => {
            return String::new();
        }
    }

}