
pub fn get_closest_checkpoint(chain_name: &str, height: u64) ->  Option<(u64, &'static str, &'static str)> {
    match chain_name {
        "test" => get_test_checkpoint(height),
        "main" => get_main_checkpoint(height),
        _      => None
    }
}

fn get_test_checkpoint(height: u64) ->  Option<(u64, &'static str, &'static str)> {
    let checkpoints: Vec<(u64, &str, &str)> = vec![
        (130000, "000000001c4a5aa11e6c142931463fcf7a9f5b9fb41061d26c18ff1860431881",
                 ""
        ),
        (152000, "00000000d7c0f7d63b7e628cfcb9fdd45c9d2a244326532c61dfff7fb0d4af45",
                 ""
        )
    ];

    find_checkpoint(height, checkpoints)
}


fn get_main_checkpoint(height: u64) ->  Option<(u64, &'static str, &'static str)> {
    let checkpoints: Vec<(u64, &str, &str)> = vec![
        (160000,  "0000000553274de0e5f07bf3a63bdb6ab71158a3506829fd6f7df2cd51d5b2a3",

                   "0175d619624f48e45df658b143f5239e22addf620d7000013dfd62298688ecb350015e464e8e594499a624a5d20e323a4de304ace8ec20b63cb41f5ed08629c678720f000001b90f0634e468eb3d2de0985c661fb9b7b8a081e4443147d617377a337c1dd13901152f108238acfd96b7b734333b0d3927d77812aa9648eef969de78c1daef023f019f3b14a209c15a14ddd3bd49355759c151ddcc1b7816fc472c7d9053f3495c6100000180623c9995f068e60c7fc0b9423eb753cc85ee8aa8df47c273ebb202dbf43f230000000001ef041d21ca2e599aca269d5a63b35f5ac2abe8e776279fb09ae902778b33746301982a0ad78d7d67d3c7b026adfdb342eceb50557cb4677ec43742028c4602216701b8d79586ce15b0cd9b3683091dea42cdad3fa4dc6d7d7853aaac062aa5717527"
        ),

        (170000,  "0000000191d6e3c5473215ab1e28a8fa8db6172eb4ec6fed371d4bd71224adb0",

                   "019081dbde619339e0e85a13df8ff833ea866502d96d9839242eaf4e6f89a9935b000f00000001b542d70d235bfef8c3ea401fc7682c4889abc1d8047346ec374a846e498cf44f013317e59dfbc56000b20131e73d531dd805c481978f86c055b2f863ea7f0b296b01e0bc6b4ae0036aecc0973669061d777fe00f33c19d561cb89f9d8fef5d9d35100001d4ce711d7659419f11c307a77ab79c0fc2f62d1e7b2fc650d6a51704bcf31223017c86ee02304db5f5158a4186cc65e9ceb6acdf88877ea59f5184b04ada15bf0c01d3b830cc0959d5c6616a2eb11231763576719f844da23aafeb3ddcd44dcbb262017cda15fd9d8af2559d2fa920983b832ef410d10811cab7e678ee788d4a74df0a0001ef041d21ca2e599aca269d5a63b35f5ac2abe8e776279fb09ae902778b33746301982a0ad78d7d67d3c7b026adfdb342eceb50557cb4677ec43742028c4602216701b8d79586ce15b0cd9b3683091dea42cdad3fa4dc6d7d7853aaac062aa5717527"
        ),

        (220000,  "00000000dd40d7372e60da03205bfc9bd796cc467737e093a58ab08b688014a4",

                   "01cf87373bc969942c2cc6dc2cb399c4eaae1d234cc785b1decae4847eb7a21c04001001f2cd27d01566c2594f6bd1d141bc68c2c5b6d7b9c27f1183780636255332b16b01b85d3ab8c19e4fa3f717e48354cd97d416b1a916b0e642777f61aa4b75359b39000000015ae551df3653030b6d6aaf858df93741290191de721b502059baf88d7d56906c016deb46b4029552954f58f0271104a1a2d16838ce5cb8e6adef9aed3c29ea271c000001911852d57122315ab96dce00ca98cf3b3dfc423759610f047a9efd8f57172656000183e3743bb17818ab1f0af4b5d4f564eabef0ff4fbf7e21c4a8f02c91d9963a3e0139bc58e226e9585cb428cf367505f04984e6cf5d91514622a3895b8a8079cb2a0122fd7dee5fcd9788ea5c5d2d3c7a61ec8e9bc667daf380d8361b888e52788f09000130195f8f73dc864a32004ff13f5fa2fed32723a150a498f593590ccec493e004"
        ),
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
        assert_eq!(get_test_checkpoint(100000), None);
        assert_eq!(get_test_checkpoint(120000).unwrap().0, 120000);
        assert_eq!(get_test_checkpoint(125000).unwrap().0, 120000);
        assert_eq!(get_test_checkpoint(157000).unwrap().0, 157000);
        assert_eq!(get_test_checkpoint(175000).unwrap().0, 157000);

        assert_eq!(get_main_checkpoint(100000), None);
        assert_eq!(get_main_checkpoint(170947).unwrap().0, 170947);
        assert_eq!(get_main_checkpoint(170949).unwrap().0, 170947);
    }

}