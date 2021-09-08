use crate::position::num_square_symmetries;
use std::cell::UnsafeCell;
use std::slice;
use std::sync::atomic::{AtomicUsize, Ordering};

pub const NUM_VALUE_PARAMS_4S: usize = 51;
pub const NUM_POLICY_PARAMS_4S: usize = 78;

pub const NUM_VALUE_PARAMS_5S: usize = 69;
pub const NUM_POLICY_PARAMS_5S: usize = 93;

pub const NUM_VALUE_PARAMS_6S: usize = 72;
pub const NUM_POLICY_PARAMS_6S: usize = 99;

struct SliceArena {
    backing_array: UnsafeCell<Box<[f32]>>,
    next_free_index: AtomicUsize,
}

impl<'a> SliceArena {
    fn new(capacity: usize) -> Self {
        SliceArena {
            backing_array: UnsafeCell::new(vec![0.0; capacity].into_boxed_slice()),
            next_free_index: AtomicUsize::new(0),
        }
    }

    fn create_new_slice(&self, len: usize) -> &'a mut [f32] {
        let slice = unsafe {
            let box_ptr: *mut Box<[f32]> = self.backing_array.get();
            assert!(self.next_free_index.load(Ordering::SeqCst) + len <= (*box_ptr).len());

            let new_slice_start: *mut f32 =
                ((*box_ptr).as_mut_ptr()).add(self.next_free_index.load(Ordering::SeqCst));
            slice::from_raw_parts_mut(new_slice_start, len)
        };

        self.next_free_index.fetch_add(len, Ordering::SeqCst);
        slice
    }

    fn mul_slice_and_sum(&self, coefficients: &[f32]) -> f32 {
        unsafe {
            let box_ptr: *mut Box<[f32]> = self.backing_array.get();

            let mut sum = 0.0;
            for (i, c) in coefficients.iter().enumerate() {
                sum += *c * *((*box_ptr).as_mut_ptr().add(i));
            }
            sum
        }
    }

    fn clear(&self) {
        unsafe {
            let box_ptr: *mut Box<[f32]> = self.backing_array.get();

            for i in 0..(*box_ptr).len() {
                (*box_ptr)[i] = 0.0;
            }
        }
    }

    fn extract_parameters(&self, parameters: &mut [f32]) {
        unsafe {
            let box_ptr: *mut Box<[f32]> = self.backing_array.get();
            assert_eq!((*box_ptr).len(), parameters.len());

            for (i, p) in parameters.iter_mut().enumerate() {
                *p = (*box_ptr)[i];
            }
        }
    }
}

pub struct ValueParameters {
    pub flat_psqt: Vec<f32>,
    pub wall_psqt: Vec<f32>,
    pub cap_psqt: Vec<f32>,
    pub our_stack_psqt: Vec<f32>,
    pub their_stack_psqt: Vec<f32>,
    pub side_to_move: Vec<f32>,
    pub flatstone_lead: Vec<f32>,
    pub i_number_of_groups: Vec<f32>,
    pub critical_squares: Vec<f32>,
    pub capstone_over_own_piece: Vec<f32>,
    pub capstone_on_stack: Vec<f32>,
    pub standing_stone_on_stack: Vec<f32>,
    pub flat_stone_next_to_our_stack: Vec<f32>,
    pub standing_stone_next_to_our_stack: Vec<f32>,
    pub capstone_next_to_our_stack: Vec<f32>,
    pub num_lines_occupied: Vec<f32>,
    pub line_control: Vec<f32>,
    pub block_their_line: Vec<f32>,
}

impl ValueParameters {
    pub fn new<const S: usize>() -> Self {
        ValueParameters {
            flat_psqt: vec![0.0; num_square_symmetries::<S>()],
            wall_psqt: vec![0.0; num_square_symmetries::<S>()],
            cap_psqt: vec![0.0; num_square_symmetries::<S>()],
            our_stack_psqt: vec![0.0; num_square_symmetries::<S>()],
            their_stack_psqt: vec![0.0; num_square_symmetries::<S>()],
            side_to_move: vec![0.0; 3],
            flatstone_lead: vec![0.0; 3],
            i_number_of_groups: vec![0.0; 3],
            critical_squares: vec![0.0; 6],
            capstone_over_own_piece: vec![0.0; 1],
            capstone_on_stack: vec![0.0; 1],
            standing_stone_on_stack: vec![0.0; 1],
            flat_stone_next_to_our_stack: vec![0.0; 1],
            standing_stone_next_to_our_stack: vec![0.0; 1],
            capstone_next_to_our_stack: vec![0.0; 1],
            num_lines_occupied: vec![0.0; S + 1],
            line_control: vec![0.0; S + 1],
            block_their_line: vec![0.0; S + 1],
        }
    }

    pub fn parameters(&self) -> impl Iterator<Item = &f32> {
        self.flat_psqt
            .iter()
            .chain(self.wall_psqt.iter())
            .chain(self.cap_psqt.iter())
            .chain(self.our_stack_psqt.iter())
            .chain(self.their_stack_psqt.iter())
            .chain(self.side_to_move.iter())
            .chain(self.flatstone_lead.iter())
            .chain(self.i_number_of_groups.iter())
            .chain(self.critical_squares.iter())
            .chain(self.capstone_over_own_piece.iter())
            .chain(self.capstone_on_stack.iter())
            .chain(self.standing_stone_on_stack.iter())
            .chain(self.flat_stone_next_to_our_stack.iter())
            .chain(self.standing_stone_next_to_our_stack.iter())
            .chain(self.capstone_next_to_our_stack.iter())
            .chain(self.num_lines_occupied.iter())
            .chain(self.line_control.iter())
            .chain(self.block_their_line.iter())
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct PolicyParameters {
    pub move_count: Vec<f32>,
    pub flat_psqt: Vec<f32>,
    pub wall_psqt: Vec<f32>,
    pub cap_psqt: Vec<f32>,
    pub our_road_stones_in_line: Vec<f32>,
    pub their_road_stones_in_line: Vec<f32>,
    pub extend_group: Vec<f32>,
    pub merge_two_groups: Vec<f32>,
    pub block_merger: Vec<f32>,
    pub place_critical_square: Vec<f32>,
    pub ignore_critical_square: Vec<f32>,
    pub next_to_our_last_stone: Vec<f32>,
    pub next_to_their_last_stone: Vec<f32>,
    pub diagonal_to_our_last_stone: Vec<f32>,
    pub diagonal_to_their_last_stone: Vec<f32>,
    pub attack_strong_flats: Vec<f32>,
    pub blocking_stone_blocks_extensions_of_two_flats: Vec<f32>,
    pub move_role_bonus: Vec<f32>,
    pub stack_movement_that_gives_us_top_pieces: Vec<f32>,
    pub stack_captured_by_movement: Vec<f32>,
    pub stack_capture_in_strong_line: Vec<f32>,
    pub stack_capture_in_strong_line_cap: Vec<f32>,
    pub move_cap_onto_strong_line: Vec<f32>,
    pub move_onto_critical_square: Vec<f32>,
}

impl PolicyParameters {
    pub fn new<const S: usize>() -> PolicyParameters {
        PolicyParameters {
            move_count: vec![0.0; 1],
            flat_psqt: vec![0.0; num_square_symmetries::<S>()],
            wall_psqt: vec![0.0; num_square_symmetries::<S>()],
            cap_psqt: vec![0.0; num_square_symmetries::<S>()],
            our_road_stones_in_line: vec![0.0; S * 3],
            their_road_stones_in_line: vec![0.0; S * 3],
            extend_group: vec![0.0; 3],
            merge_two_groups: vec![0.0; 3],
            block_merger: vec![0.0; 3],
            place_critical_square: vec![0.0; 5],
            ignore_critical_square: vec![0.0; 2],
            next_to_our_last_stone: vec![0.0; 1],
            next_to_their_last_stone: vec![0.0; 1],
            diagonal_to_our_last_stone: vec![0.0; 1],
            diagonal_to_their_last_stone: vec![0.0; 1],
            attack_strong_flats: vec![0.0; 1],
            blocking_stone_blocks_extensions_of_two_flats: vec![0.0; 1],
            move_role_bonus: vec![0.0; 3],
            stack_movement_that_gives_us_top_pieces: vec![0.0; 6],
            stack_captured_by_movement: vec![0.0; 1],
            stack_capture_in_strong_line: vec![0.0; S - 3],
            stack_capture_in_strong_line_cap: vec![0.0; S - 3],
            move_cap_onto_strong_line: vec![0.0; 5],
            move_onto_critical_square: vec![0.0; 4],
        }
    }

    #[inline(never)]
    pub fn clear(&mut self) {
        set_zero(&mut self.move_count);
        set_zero(&mut self.flat_psqt);
        set_zero(&mut self.wall_psqt);
        set_zero(&mut self.cap_psqt);
        set_zero(&mut self.our_road_stones_in_line);
        set_zero(&mut self.their_road_stones_in_line);
        set_zero(&mut self.extend_group);
        set_zero(&mut self.merge_two_groups);
        set_zero(&mut self.block_merger);
        set_zero(&mut self.place_critical_square);
        set_zero(&mut self.ignore_critical_square);
        set_zero(&mut self.next_to_our_last_stone);
        set_zero(&mut self.next_to_their_last_stone);
        set_zero(&mut self.diagonal_to_our_last_stone);
        set_zero(&mut self.diagonal_to_their_last_stone);
        set_zero(&mut self.attack_strong_flats);
        set_zero(&mut self.blocking_stone_blocks_extensions_of_two_flats);
        set_zero(&mut self.move_role_bonus);
        set_zero(&mut self.stack_movement_that_gives_us_top_pieces);
        set_zero(&mut self.stack_captured_by_movement);
        set_zero(&mut self.stack_capture_in_strong_line);
        set_zero(&mut self.stack_capture_in_strong_line_cap);
        set_zero(&mut self.move_cap_onto_strong_line);
        set_zero(&mut self.move_onto_critical_square);
    }

    #[inline(never)]
    pub fn parameters(&self, vec: &mut Vec<f32>) {
        vec.extend_from_slice(&self.move_count);
        vec.extend_from_slice(&self.flat_psqt);
        vec.extend_from_slice(&self.wall_psqt);
        vec.extend_from_slice(&self.cap_psqt);
        vec.extend_from_slice(&self.our_road_stones_in_line);
        vec.extend_from_slice(&self.their_road_stones_in_line);
        vec.extend_from_slice(&self.extend_group);
        vec.extend_from_slice(&self.merge_two_groups);
        vec.extend_from_slice(&self.block_merger);
        vec.extend_from_slice(&self.place_critical_square);
        vec.extend_from_slice(&self.ignore_critical_square);
        vec.extend_from_slice(&self.next_to_our_last_stone);
        vec.extend_from_slice(&self.next_to_their_last_stone);
        vec.extend_from_slice(&self.diagonal_to_our_last_stone);
        vec.extend_from_slice(&self.diagonal_to_their_last_stone);
        vec.extend_from_slice(&self.attack_strong_flats);
        vec.extend_from_slice(&self.blocking_stone_blocks_extensions_of_two_flats);
        vec.extend_from_slice(&self.move_role_bonus);
        vec.extend_from_slice(&self.stack_movement_that_gives_us_top_pieces);
        vec.extend_from_slice(&self.stack_captured_by_movement);
        vec.extend_from_slice(&self.stack_capture_in_strong_line);
        vec.extend_from_slice(&self.stack_capture_in_strong_line_cap);
        vec.extend_from_slice(&self.move_cap_onto_strong_line);
        vec.extend_from_slice(&self.move_onto_critical_square);
    }
}

pub fn set_zero(parameters: &mut [f32]) {
    for p in parameters.iter_mut() {
        *p = 0.0;
    }
}

#[allow(clippy::unreadable_literal)]
pub const VALUE_PARAMS_4S: [f32; NUM_VALUE_PARAMS_4S] = [
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
pub const POLICY_PARAMS_4S: [f32; NUM_POLICY_PARAMS_4S] = [
    0.96480286,
    0.112528,
    0.25500473,
    0.4112829,
    -0.08060045,
    -0.23519231,
    -0.13165799,
    -0.008488559,
    -0.0050255177,
    0.00999761,
    -0.113257684,
    -0.16278866,
    0.6787402,
    1.1777495,
    -0.3540673,
    -0.3450718,
    -0.14545316,
    -0.03423411,
    -0.006642866,
    0.0071283225,
    -0.004408128,
    -0.008781511,
    0.3106411,
    -0.41825965,
    0.3085478,
    1.3927934,
    -0.8061303,
    -0.48430258,
    -0.39221516,
    0.7936677,
    -0.004510097,
    -0.009524884,
    -0.008346889,
    0.0068213847,
    0.060292657,
    0.056763157,
    0.0071164146,
    0.17309238,
    0.2843739,
    0.0025831861,
    -0.029959261,
    0.81146693,
    -0.0073796343,
    1.8442278,
    0.46516204,
    0.9331577,
    -0.009010632,
    0.8256556,
    -3.113514,
    -1.4565177,
    0.6001051,
    1.3691607,
    0.36907476,
    0.22568786,
    0.23353463,
    0.2539174,
    -0.98199755,
    -1.413433,
    0.0051357076,
    -0.38820496,
    0.93453836,
    -1.2818239,
    -1.3273182,
    -0.7235824,
    -0.86146796,
    0.609189,
    -0.14510402,
    0.009848367,
    -0.009406421,
    -0.008845272,
    0.001895139,
    0.007292602,
    -0.0025014114,
    0.00015416648,
    -0.045751374,
    -0.008639442,
    1.651265,
    0.66081977,
];

#[allow(clippy::unreadable_literal)]
pub const VALUE_PARAMS_5S: [f32; NUM_VALUE_PARAMS_5S] = [
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
pub const POLICY_PARAMS_5S: [f32; NUM_POLICY_PARAMS_5S] = [
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
pub const VALUE_PARAMS_6S: [f32; NUM_VALUE_PARAMS_6S] = [
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
pub const POLICY_PARAMS_6S: [f32; NUM_POLICY_PARAMS_6S] = [
    0.89010215,
    -0.44227162,
    -0.040291704,
    -0.2304978,
    1.0121428,
    0.4682882,
    -0.02764575,
    -0.041628633,
    -0.19531427,
    -0.16233057,
    -0.13584544,
    -0.23512484,
    -0.16102716,
    -0.41830775,
    0.002314275,
    -0.922825,
    -0.05262945,
    0.044212952,
    0.8872694,
    -0.26905358,
    -0.24215038,
    0.020847214,
    0.44418764,
    0.9954152,
    0.48969522,
    -0.3372674,
    -0.50664485,
    -0.5019418,
    -0.31593677,
    -0.19631371,
    -0.006926868,
    -0.3731796,
    -0.53606695,
    -0.43022972,
    0.07106458,
    0.29349113,
    0.010416347,
    0.4458158,
    -0.10321867,
    -0.044042256,
    0.2408937,
    0.6781876,
    0.25086695,
    -0.2637086,
    -0.38575223,
    -0.5381356,
    -0.3633851,
    -0.15752447,
    -0.12405224,
    -0.50303805,
    -1.0917884,
    -0.96145505,
    -0.65610796,
    0.52460515,
    1.750216,
    0.019733787,
    0.011104343,
    0.0029525892,
    0.6340498,
    -0.010717551,
    0.3188459,
    0.28675926,
    0.49561694,
    0.84726703,
    1.9913195,
    0.3848487,
    3.3264358,
    1.6433451,
    0.5454407,
    -3.1533794,
    -1.4226677,
    1.5443202,
    1.3655221,
    0.42389217,
    0.5014939,
    0.09088122,
    0.31120455,
    -0.53451127,
    -0.9207075,
    -0.78396416,
    -0.16850425,
    0.2872031,
    -0.9876141,
    -1.1136076,
    -1.1014684,
    -1.1989646,
    0.47031713,
    0.0057362453,
    0.023288574,
    -0.055236913,
    0.09043382,
    -0.11676836,
    0.030210404,
    -0.06826632,
    0.010061843,
    0.016899843,
    0.020130021,
    1.5230228,
    0.9176952,
];
