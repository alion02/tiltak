use crate::position::num_square_symmetries;

pub const NUM_VALUE_FEATURES_4S: usize = 51;
pub const NUM_POLICY_FEATURES_4S: usize = 74;

pub const NUM_VALUE_FEATURES_5S: usize = 69;
pub const NUM_POLICY_FEATURES_5S: usize = 93;

pub const NUM_VALUE_FEATURES_6S: usize = 72;
pub const NUM_POLICY_FEATURES_6S: usize = 124;

#[derive(Debug)]
pub struct ValueFeatures<'a> {
    pub flat_psqt: &'a mut [f32],
    pub wall_psqt: &'a mut [f32],
    pub cap_psqt: &'a mut [f32],
    pub our_stack_psqt: &'a mut [f32],
    pub their_stack_psqt: &'a mut [f32],
    pub side_to_move: &'a mut [f32],
    pub flatstone_lead: &'a mut [f32],
    pub i_number_of_groups: &'a mut [f32],
    pub critical_squares: &'a mut [f32],
    pub capstone_over_own_piece: &'a mut [f32],
    pub capstone_on_stack: &'a mut [f32],
    pub standing_stone_on_stack: &'a mut [f32],
    pub flat_stone_next_to_our_stack: &'a mut [f32],
    pub standing_stone_next_to_our_stack: &'a mut [f32],
    pub capstone_next_to_our_stack: &'a mut [f32],
    pub num_lines_occupied: &'a mut [f32],
    pub line_control: &'a mut [f32],
    pub block_their_line: &'a mut [f32],
}

impl<'a> ValueFeatures<'a> {
    pub fn new<const S: usize>(coefficients: &'a mut [f32]) -> Self {
        assert_eq!(coefficients.len(), num_value_features::<S>());

        let (flat_psqt, coefficients) = coefficients.split_at_mut(num_square_symmetries::<S>());
        let (wall_psqt, coefficients) = coefficients.split_at_mut(num_square_symmetries::<S>());
        let (cap_psqt, coefficients) = coefficients.split_at_mut(num_square_symmetries::<S>());
        let (our_stack_psqt, coefficients) =
            coefficients.split_at_mut(num_square_symmetries::<S>());
        let (their_stack_psqt, coefficients) =
            coefficients.split_at_mut(num_square_symmetries::<S>());
        let (side_to_move, coefficients) = coefficients.split_at_mut(3);
        let (flatstone_lead, coefficients) = coefficients.split_at_mut(3);
        let (i_number_of_groups, coefficients) = coefficients.split_at_mut(3);
        let (critical_squares, coefficients) = coefficients.split_at_mut(6);
        let (capstone_over_own_piece, coefficients) = coefficients.split_at_mut(1);
        let (capstone_on_stack, coefficients) = coefficients.split_at_mut(1);
        let (standing_stone_on_stack, coefficients) = coefficients.split_at_mut(1);
        let (flat_stone_next_to_our_stack, coefficients) = coefficients.split_at_mut(1);
        let (standing_stone_next_to_our_stack, coefficients) = coefficients.split_at_mut(1);
        let (capstone_next_to_our_stack, coefficients) = coefficients.split_at_mut(1);
        let (num_lines_occupied, coefficients) = coefficients.split_at_mut(S + 1);
        let (line_control, coefficients) = coefficients.split_at_mut(S + 1);
        let (block_their_line, coefficients) = coefficients.split_at_mut(S + 1);

        assert!(coefficients.is_empty());

        ValueFeatures {
            flat_psqt,
            wall_psqt,
            cap_psqt,
            our_stack_psqt,
            their_stack_psqt,
            side_to_move,
            flatstone_lead,
            i_number_of_groups,
            critical_squares,
            capstone_over_own_piece,
            capstone_on_stack,
            standing_stone_on_stack,
            flat_stone_next_to_our_stack,
            standing_stone_next_to_our_stack,
            capstone_next_to_our_stack,
            num_lines_occupied,
            line_control,
            block_their_line,
        }
    }
}

#[derive(Debug)]
pub struct PolicyFeatures<'a> {
    pub move_count: &'a mut [f32],
    pub place_to_win: &'a mut [f32],
    pub place_to_draw: &'a mut [f32],
    pub place_to_loss: &'a mut [f32],
    pub place_to_allow_opponent_to_end: &'a mut [f32],
    pub flat_psqt: &'a mut [f32],
    pub wall_psqt: &'a mut [f32],
    pub cap_psqt: &'a mut [f32],
    pub our_road_stones_in_line: &'a mut [f32],
    pub their_road_stones_in_line: &'a mut [f32],
    pub extend_single_group_base: &'a mut [f32],
    pub extend_single_group_linear: &'a mut [f32],
    pub extend_single_group_to_new_line_base: &'a mut [f32],
    pub extend_single_group_to_new_line_linear: &'a mut [f32],
    pub merge_two_groups_base: &'a mut [f32],
    pub merge_two_groups_linear: &'a mut [f32],
    pub block_merger_base: &'a mut [f32],
    pub block_merger_linear: &'a mut [f32],
    pub place_critical_square: &'a mut [f32],
    pub ignore_critical_square: &'a mut [f32],
    pub next_to_our_last_stone: &'a mut [f32],
    pub next_to_their_last_stone: &'a mut [f32],
    pub diagonal_to_our_last_stone: &'a mut [f32],
    pub diagonal_to_their_last_stone: &'a mut [f32],
    pub attack_strong_flats: &'a mut [f32],
    pub blocking_stone_blocks_extensions_of_two_flats: &'a mut [f32],
    pub move_role_bonus: &'a mut [f32],
    pub stack_movement_that_gives_us_top_pieces: &'a mut [f32],
    pub stack_captured_by_movement: &'a mut [f32],
    pub stack_capture_in_strong_line: &'a mut [f32],
    pub stack_capture_in_strong_line_cap: &'a mut [f32],
    pub move_cap_onto_strong_line: &'a mut [f32],
    pub move_cap_onto_strong_line_with_critical_square: &'a mut [f32],
    pub move_onto_critical_square: &'a mut [f32],
}

impl<'a> PolicyFeatures<'a> {
    #[inline(never)]
    pub fn new<const S: usize>(coefficients: &'a mut [f32]) -> PolicyFeatures<'a> {
        assert_eq!(coefficients.len(), num_policy_features::<S>());

        let (move_count, coefficients) = coefficients.split_at_mut(1);
        let (place_to_win, coefficients) = coefficients.split_at_mut(1);
        let (place_to_draw, coefficients) = coefficients.split_at_mut(1);
        let (place_to_loss, coefficients) = coefficients.split_at_mut(1);
        let (place_to_allow_opponent_to_end, coefficients) = coefficients.split_at_mut(3);
        let (flat_psqt, coefficients) = coefficients.split_at_mut(num_square_symmetries::<S>());
        let (wall_psqt, coefficients) = coefficients.split_at_mut(num_square_symmetries::<S>());
        let (cap_psqt, coefficients) = coefficients.split_at_mut(num_square_symmetries::<S>());
        let (our_road_stones_in_line, coefficients) = coefficients.split_at_mut(S * 3);
        let (their_road_stones_in_line, coefficients) = coefficients.split_at_mut(S * 3);
        let (extend_single_group_to_new_line_base, coefficients) = coefficients.split_at_mut(3);
        let (extend_single_group_to_new_line_linear, coefficients) = coefficients.split_at_mut(3);
        let (extend_single_group_base, coefficients) = coefficients.split_at_mut(3);
        let (extend_single_group_linear, coefficients) = coefficients.split_at_mut(3);
        let (merge_two_groups_base, coefficients) = coefficients.split_at_mut(3);
        let (merge_two_groups_linear, coefficients) = coefficients.split_at_mut(3);
        let (block_merger_base, coefficients) = coefficients.split_at_mut(3);
        let (block_merger_linear, coefficients) = coefficients.split_at_mut(3);
        let (place_critical_square, coefficients) = coefficients.split_at_mut(5);
        let (ignore_critical_square, coefficients) = coefficients.split_at_mut(2);
        let (next_to_our_last_stone, coefficients) = coefficients.split_at_mut(1);
        let (next_to_their_last_stone, coefficients) = coefficients.split_at_mut(1);
        let (diagonal_to_our_last_stone, coefficients) = coefficients.split_at_mut(1);
        let (diagonal_to_their_last_stone, coefficients) = coefficients.split_at_mut(1);
        let (attack_strong_flats, coefficients) = coefficients.split_at_mut(1);
        let (blocking_stone_blocks_extensions_of_two_flats, coefficients) =
            coefficients.split_at_mut(1);
        let (move_role_bonus, coefficients) = coefficients.split_at_mut(3);
        let (stack_movement_that_gives_us_top_pieces, coefficients) = coefficients.split_at_mut(6);
        let (stack_captured_by_movement, coefficients) = coefficients.split_at_mut(1);
        let (stack_capture_in_strong_line, coefficients) = coefficients.split_at_mut(S - 3);
        let (stack_capture_in_strong_line_cap, coefficients) = coefficients.split_at_mut(S - 3);
        let (move_cap_onto_strong_line, coefficients) = coefficients.split_at_mut(S - 3);
        let (move_cap_onto_strong_line_with_critical_square, coefficients) =
            coefficients.split_at_mut(S - 3);
        let (move_onto_critical_square, coefficients) = coefficients.split_at_mut(4);

        assert!(coefficients.is_empty());

        PolicyFeatures {
            move_count,
            place_to_win,
            place_to_draw,
            place_to_loss,
            place_to_allow_opponent_to_end,
            flat_psqt,
            wall_psqt,
            cap_psqt,
            our_road_stones_in_line,
            their_road_stones_in_line,
            extend_single_group_base,
            extend_single_group_linear,
            extend_single_group_to_new_line_base,
            extend_single_group_to_new_line_linear,
            merge_two_groups_base,
            merge_two_groups_linear,
            block_merger_base,
            block_merger_linear,
            place_critical_square,
            ignore_critical_square,
            next_to_our_last_stone,
            next_to_their_last_stone,
            diagonal_to_our_last_stone,
            diagonal_to_their_last_stone,
            attack_strong_flats,
            blocking_stone_blocks_extensions_of_two_flats,
            move_role_bonus,
            stack_movement_that_gives_us_top_pieces,
            stack_captured_by_movement,
            stack_capture_in_strong_line,
            stack_capture_in_strong_line_cap,
            move_cap_onto_strong_line,
            move_cap_onto_strong_line_with_critical_square,
            move_onto_critical_square,
        }
    }
}

pub fn num_value_features<const S: usize>() -> usize {
    match S {
        4 => NUM_VALUE_FEATURES_4S,
        5 => NUM_VALUE_FEATURES_5S,
        6 => NUM_VALUE_FEATURES_6S,
        _ => unimplemented!(),
    }
}

pub fn num_policy_features<const S: usize>() -> usize {
    match S {
        4 => NUM_POLICY_FEATURES_4S,
        5 => NUM_POLICY_FEATURES_5S,
        6 => NUM_POLICY_FEATURES_6S,
        _ => unimplemented!(),
    }
}

#[allow(clippy::unreadable_literal)]
pub const VALUE_PARAMS_4S: [f32; NUM_VALUE_FEATURES_4S] = [
    0.40710354,
    0.54004306,
    0.7314182,
    1.0928891,
    1.0292282,
    1.3468512,
    0.0060105007,
    0.0055509824,
    -0.00028526317,
    1.3862512,
    1.7173628,
    1.6419909,
    0.9690684,
    1.0156716,
    1.0867381,
    1.3008595,
    1.4003952,
    1.7866403,
    0.48352754,
    0.10772651,
    1.0677989,
    -0.2995102,
    -0.2784462,
    0.4824724,
    0.3578643,
    0.30005231,
    0.112963594,
    0.03649128,
    -0.0068652583,
    -0.0045327637,
    -0.007574806,
    -0.001304483,
    0.7344016,
    -0.012913749,
    -0.27687806,
    -0.00081042293,
    0.8332573,
    -1.0331386,
    -0.5144214,
    -0.005624555,
    0.71696895,
    -0.81962454,
    -0.5723752,
    0.25761572,
    1.1193179,
    0.005045411,
    -0.03437448,
    0.008928416,
    0.048655495,
    -0.0016141674,
    0.003347816,
];
#[allow(clippy::unreadable_literal)]
pub const POLICY_PARAMS_4S: [f32; NUM_POLICY_FEATURES_4S] = [
    0.9671596,
    0.109868206,
    0.25096068,
    0.4193486,
    -0.057352237,
    -0.26003632,
    -0.103470474,
    -0.0060374904,
    -0.0061954064,
    0.0060048904,
    -0.111464605,
    -0.17581047,
    0.6776471,
    1.204406,
    -0.3764452,
    -0.33065042,
    -0.119658254,
    -0.027075306,
    0.009635687,
    -0.0014058612,
    -0.0038658213,
    -0.00062850676,
    0.31979835,
    -0.40791842,
    0.30711442,
    1.3606137,
    -0.7807742,
    -0.4970676,
    -0.3871676,
    0.81400794,
    -0.007937893,
    0.0076041985,
    -0.00032685045,
    -0.00023208652,
    0.05871541,
    0.059758093,
    0.0032768678,
    0.19188155,
    0.22739168,
    -0.0011021066,
    -0.022244656,
    0.7534389,
    0.008557079,
    1.8071839,
    0.5237552,
    0.99771214,
    -0.009191279,
    0.8339809,
    -3.4862912,
    -1.4936633,
    0.6061734,
    1.3510658,
    0.3686262,
    0.20583504,
    0.24588718,
    0.2552688,
    -0.98601824,
    -1.4256523,
    0.002150652,
    -0.37348795,
    0.9059205,
    -1.3043255,
    -1.3905704,
    -0.73312885,
    -0.853796,
    0.6280024,
    -0.102411255,
    0.0040705632,
    0.0010270234,
    0.0011965558,
    -0.06954638,
    0.010308798,
    1.6920078,
    0.6572741,
];

#[allow(clippy::unreadable_literal)]
pub const VALUE_PARAMS_5S: [f32; NUM_VALUE_FEATURES_5S] = [
    -0.00044795033,
    0.15347332,
    0.14927012,
    0.25764394,
    0.2447137,
    0.27844432,
    0.7183903,
    0.79589164,
    0.69361377,
    0.93700093,
    0.77688575,
    1.0438795,
    -0.47725853,
    0.023881366,
    0.10956399,
    0.6041755,
    0.7021375,
    0.9956894,
    1.1578636,
    1.1255516,
    1.2779299,
    1.2831495,
    1.311057,
    1.2934446,
    0.7101744,
    0.73263896,
    0.77619076,
    0.8653954,
    0.8186914,
    0.8584326,
    0.98251414,
    0.7959507,
    1.0613332,
    0.61214393,
    0.04162296,
    0.47685462,
    -0.18535407,
    -0.175548,
    0.025191614,
    0.31633365,
    0.044689283,
    0.08818814,
    -0.04582565,
    0.036502212,
    0.11076386,
    0.12404986,
    0.60829574,
    0.35141426,
    -0.032268483,
    -0.15010805,
    -0.15450484,
    0.7011735,
    -0.77606714,
    -0.432654,
    -0.1280988,
    0.12062097,
    0.5066281,
    -1.0205822,
    -0.7606904,
    -0.18055946,
    0.6164267,
    1.3433626,
    0.0029393125,
    0.012231762,
    -0.07691176,
    0.14723985,
    0.103527844,
    0.08759902,
    -0.0380222,
];

#[allow(clippy::unreadable_literal)]
pub const POLICY_PARAMS_5S: [f32; NUM_POLICY_FEATURES_5S] = [
    0.9295695,
    -0.083421424,
    0.08532888,
    0.2387281,
    0.5274521,
    0.24493831,
    0.09562564,
    0.15470326,
    -0.24693699,
    -0.30108738,
    -0.21431528,
    -0.2704717,
    -0.022588426,
    -1.7911652,
    -1.2216772,
    -0.94670635,
    -0.13166411,
    0.7949566,
    3.6376195,
    -0.19435506,
    -0.25095072,
    0.26169717,
    0.8698118,
    1.5360512,
    -0.43167987,
    -0.46735555,
    -0.46749815,
    -0.4370854,
    -0.02836023,
    -0.10167722,
    -0.40005624,
    0.26396355,
    1.1372943,
    -0.24493426,
    0.38994327,
    -0.08613078,
    0.20141272,
    0.7585348,
    0.9396367,
    -0.44191897,
    -0.654275,
    -0.7014535,
    -0.19884999,
    0.19501987,
    -0.1429219,
    -0.89655656,
    -0.26302937,
    0.91213554,
    1.0827745,
    0.034203924,
    0.018315857,
    -0.047500875,
    0.5382421,
    0.14895687,
    0.6110193,
    0.228183,
    0.75111663,
    0.77497125,
    1.953449,
    0.41483137,
    2.9484003,
    2.1978526,
    0.7461167,
    -3.6696496,
    -1.7434666,
    0.8811208,
    1.1639132,
    0.27322596,
    0.2283973,
    0.10525754,
    0.25576675,
    -0.8693829,
    -1.370112,
    -0.8019496,
    -0.062427573,
    0.4139188,
    -1.3210533,
    -1.8491417,
    -1.6519781,
    -1.9573829,
    0.5835941,
    0.021586694,
    -0.15781526,
    -0.05396689,
    0.05479999,
    0.109241985,
    0.9827647,
    0.024347505,
    -0.3630683,
    0.0024180522,
    0.0050033038,
    2.3537161,
    1.012874,
];

#[allow(clippy::unreadable_literal)]
pub const VALUE_PARAMS_6S: [f32; NUM_VALUE_FEATURES_6S] = [
    0.14389691,
    0.14861599,
    0.21016096,
    0.26289055,
    0.29426488,
    0.2697997,
    0.41935468,
    0.5154811,
    0.5189208,
    0.6312286,
    0.5163997,
    0.7095309,
    -0.40372136,
    -0.17229083,
    -0.08402492,
    0.22461775,
    0.32553476,
    0.5386108,
    0.45318204,
    0.63134146,
    0.763742,
    0.7841039,
    0.82111675,
    0.7907069,
    0.37137604,
    0.4108737,
    0.48396173,
    0.5127213,
    0.5265846,
    0.50946397,
    0.522094,
    0.46919465,
    0.63071007,
    0.5474945,
    0.29245657,
    0.48472342,
    -0.23528779,
    -0.04572703,
    -0.00088525214,
    0.15706137,
    0.14138941,
    0.08143665,
    -0.012585025,
    0.12482873,
    0.016435517,
    0.16839688,
    0.5649024,
    0.3192357,
    -0.025385296,
    -0.076988645,
    -0.115113735,
    0.43385288,
    -0.32607457,
    -0.24166256,
    -0.15374532,
    -0.06544677,
    0.090924904,
    0.2354019,
    -0.39573103,
    -0.33551803,
    -0.1949905,
    0.024735041,
    0.31291002,
    0.58010185,
    -0.0008957982,
    0.008336878,
    0.036577567,
    0.010293869,
    0.061720997,
    0.028714254,
    0.112023935,
    0.45978305,
];

#[allow(clippy::unreadable_literal)]
pub const POLICY_PARAMS_6S: [f32; NUM_POLICY_FEATURES_6S] = [
    0.8871399,
    1.0409842,
    -0.25552294,
    -1.495511,
    -0.97078484,
    -0.27229813,
    0.24733226,
    -0.49728855,
    -0.1082212,
    -0.32109883,
    0.9907092,
    0.35598788,
    -0.14424734,
    -0.08939605,
    -0.2611249,
    -0.15246868,
    -0.21391538,
    -0.2915501,
    -0.12738566,
    -0.76857847,
    0.06755038,
    -1.133379,
    0.050237235,
    0.1794977,
    1.263947,
    -0.22954936,
    -0.42353666,
    -0.23021892,
    0.12578303,
    0.6742153,
    0.61242664,
    -0.3785727,
    -0.5995813,
    -0.5740188,
    -0.39696148,
    -0.3731951,
    -0.002882672,
    -0.40506285,
    -0.47128367,
    -0.45735934,
    -0.08354871,
    0.77885306,
    -0.024893213,
    0.34304848,
    -0.21410404,
    -0.19239059,
    0.047880128,
    0.47995555,
    0.11512679,
    -0.24866985,
    -0.44188163,
    -0.6035403,
    -0.48113686,
    -0.25997403,
    -0.2548147,
    -0.5802093,
    -1.015056,
    -1.0772052,
    -0.7387348,
    0.61061335,
    2.1483335,
    0.36955896,
    0.094122425,
    0.081314124,
    0.067741804,
    -0.35951248,
    0.30128998,
    0.7480354,
    -0.115507856,
    -0.13236769,
    0.01911603,
    0.26400188,
    0.0369429,
    1.724688,
    0.08294649,
    0.5308813,
    -0.046644907,
    -0.06857994,
    0.13460615,
    0.31916827,
    0.7957743,
    0.5903481,
    -0.091560416,
    -0.23304623,
    -0.01425998,
    2.0364752,
    0.26345152,
    3.7460468,
    1.8653078,
    0.51769996,
    -3.5681424,
    -1.7088294,
    1.1859558,
    1.4162201,
    0.35800454,
    0.5043748,
    0.108104974,
    0.21669652,
    -0.5817199,
    -0.9207186,
    -0.82363874,
    -0.111997604,
    0.26208967,
    -0.9049907,
    -1.0358963,
    -1.3020431,
    -1.4731607,
    0.4834285,
    0.00014141538,
    0.020505855,
    -0.09004153,
    -0.043287057,
    0.04013706,
    -0.017339142,
    -0.6746351,
    0.08473358,
    0.08028541,
    -0.14006078,
    0.043344818,
    0.04260831,
    0.0031165145,
    0.048602823,
    1.0264915,
    0.91554815,
];
