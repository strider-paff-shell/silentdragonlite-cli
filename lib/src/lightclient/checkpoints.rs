pub fn get_closest_checkpoint(chain_name: &str, height: u64) ->  Option<(u64, &'static str, &'static str)> {
    match chain_name {
        "test" => get_test_checkpoint(height),
        "main" => get_main_checkpoint(height),
        _      => None
    }
}

fn get_test_checkpoint(height: u64) ->  Option<(u64, &'static str, &'static str)> {
    let checkpoints: Vec<(u64, &str, &str)> = vec![
        (105942, "",
                 ""
        ),
        (105943, "",
                 ""
        )
    ];

    find_checkpoint(height, checkpoints)
}


fn get_main_checkpoint(height: u64) ->  Option<(u64, &'static str, &'static str)> {
    let checkpoints: Vec<(u64, &str, &str)> = vec![
        (105942, "00000001c0199f329ee03379bf1387856dbab23765da508bf9b9d8d544f212c0",
                 "004f80404f103cd92802630f047e8f451ca20bdf1e247b4edf854fac990f544253a96ea7a17741388c8e04b29eee23d51a1f3f45a281c7e7331d2b258f8be2108494d0471d29a9624702cde0e75a294b955a0e95144d4e84fa0ffe20b4e1b558caec6255667233c1c833fc6226eb7a7ce1d95c83ed6ccae5a69abe19f75722463fd3e15b1271e2f9c86e27e54526a90137a4e22bc86722e80b0341fe6cd31a685d21d12ab9acd9c402afedd6ad295b75bfc530c3bedfcf5494af8cbd011a9a31fb66c745d2da32e62f527c8012c6e416917009b1f2fcf2d4732f16ed14a5d3304fda22fcf43947278437143ce2c5731b9e7b4bf6fd206be2385f69370417f3113822be6f4d22259649d54a86119b9a24390f485f0453696679f9a640fe62e40d0102148e96e30c68f5173a1d695b3f03b0e522db53d2af281e72aa1e4deea89660019ba6ea92071b53c07f7b62deb508065cb785411019711933a3c99049f68a703813dd92486622d57996adf99f8094b2945cc0bf09039a70cc16a49e1f924963ddb302d3faef374ef15a0a191cae44babb02e653a373dff177e0236650427d5458c1a30e269e1a4a9bcfb16f7031c3eb54967cb5a9339d341d79be9a54103fdaebcfd29c47b1e6a4dc57150f81293c8a5ce6e167b59b38192bf17adbaa5cf47effdb41e24a339617ccdb3cdfb51d26ec0761d012191bca0a407d9ea88e30f5b5dff13305393828677fcf13a410709fb9a9908553b032726b89162a08bd28ac6f130a6e74ab728b7cd0a70e1389a91e9aaf0f129f9a6540d9ffe0933a5f0dda5114cbb2fa04e5a39a6e345f0a8b646034c73e9db9ff42cd6dcd03a203913539b21c7746c5daa2f8e5fa73267f143b967e0615f8583919866158b6570afac715b53375cfdbde372d3d0e9131cc53084f559eab9d09e3a5fed6c647093578f0e700b48c01acb2ab79bb5b3501405965161f331c2da8112548f2b02c308dc26f86a896d15cddd61f8f3eb309efe46b082847ddd72aa918dbdbc1e6bfbbbc7cf90e81429cd18af174bee97586d16e8e4624b2975bc2034d5abc63232b89f93a51626f4f1031cfc57594f11b52771db0c72d684e9cf26a59947c12c1037b1a09058e2215e22efb8990beb3d8e55c9dc5fdf555037c5e5254705d9999f1d91736e0a23c7f61be33f0b05204dd6b5d44051b1ba08891b27db5fbfe0ff0bcb00b19eeaa279e554b1fbbaac5e6574d722238009d4fd406fd0abe8e48d9b1040e41cedb7d14c2bc9bf8040f26e785122dcb3b41be1fe3084c5f2961277b5e7cac17f587cf7edf53f3bac8664cfc66d187b8957f854e2842cf31518dcc24cebfd38cb97308fd4d66950b743c0d23f06815e4f31a3667d9a7fddf8b0b4f1fe0945233ed8fd0d7b5cf5e173863875665b228cf7bfcbf0258cb198eeb8607a690f4365925ddae0dd3f6123313ec40b886c79a4fc7df692db5714c7b3c3a7c8863080e8b212c634f8769d4f44c3d412769aa00371a9a21fe62847b4aa588e07d0223a8c8f60aebeb5db76404da24d0a2e75ea7a40ba2450c1c24167e61b8ce30064b637f9a82b3dd047a557888f6b5797a01d5c95e0b571e298a95877d75fc00bf10686621a8df18c106363536e40a989d716a8c874dc3e891066c8d5fea2102fe273e80ad3ced95819045e81c9e70570b977ca92a43f9c9fc186f659930a481f4fbc596d3b19ae2e913af6678a4c7103d7bd872f0c9a4770a446a35547d245a0716ed4ca360fcc5c7d21457f05e7493d847931195073ab9d487f593cfb72873f2cb57887e7fc87c1fff7ce92894f107da19442f4cb9a961b6535a6104122491825a4d9395d9568140bd7547fcafa2e89f2d12f1e53a3e53f557e36b6582e862dbce0c11d78a3e"
        )
    ];

    find_checkpoint(height, checkpoints)
}

fn find_checkpoint(height: u64, chkpts: Vec<(u64, &'static str, &'static str)>) -> Option<(u64, &'static str, &'static str)> {
    // Find the closest checkpoint
    let mut heights = chkpts.iter().map(|(h, _, _)| *h as u64).collect::<Vec<_>>();
    heights.sort();

    match get_first_lower_than(height, heights) {
        Some(closest_height) => {
            chkpts.iter().find(|(h, _, _)| *h ==  closest_height).map(|t| *t)
        },
        None    => None
    }
}

fn get_first_lower_than(height: u64, heights: Vec<u64>) -> Option<u64> {
    // If it's before the first checkpoint, return None. 
    if heights.len() == 0 || height < heights[0] {
        return None;
    }

    for (i, h) in heights.iter().enumerate() {
        if height < *h {
            return Some(heights[i-1]);
        }
    }

    return Some(*heights.last().unwrap());
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_lower_than() {
        assert_eq!(get_first_lower_than( 9, vec![10, 30, 40]), None);
        assert_eq!(get_first_lower_than(10, vec![10, 30, 40]).unwrap(), 10);
        assert_eq!(get_first_lower_than(11, vec![10, 30, 40]).unwrap(), 10);
        assert_eq!(get_first_lower_than(29, vec![10, 30, 40]).unwrap(), 10);
        assert_eq!(get_first_lower_than(30, vec![10, 30, 40]).unwrap(), 30);
        assert_eq!(get_first_lower_than(40, vec![10, 30, 40]).unwrap(), 40);
        assert_eq!(get_first_lower_than(41, vec![10, 30, 40]).unwrap(), 40);
        assert_eq!(get_first_lower_than(99, vec![10, 30, 40]).unwrap(), 40);
    }

    #[test]
    fn test_checkpoints() {
        assert_eq!(get_test_checkpoint(990000), None);
        assert_eq!(get_test_checkpoint(100000).unwrap().0, 100000);
        assert_eq!(get_test_checkpoint(110000).unwrap().0, 100000);
        assert_eq!(get_test_checkpoint(111000).unwrap().0, 1100000);
        assert_eq!(get_test_checkpoint(112000).unwrap().0, 1100000);

        assert_eq!(get_main_checkpoint(990000), None);
        assert_eq!(get_main_checkpoint(110000).unwrap().0, 110000);
        assert_eq!(get_main_checkpoint(111000).unwrap().0, 110000);
    }

}