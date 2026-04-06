// simple sp network

// compile and run:
// rustc -o spnet.out spnet.rs && ./spnet.out && rm -f spnet.out

const S_BLOCK_STRAIGHT: [u32; 16] = [
    4 , 2 , 12, 6 , 15, 9 , 5 , 1 ,
    7 , 0 , 8 , 3 , 11, 13, 14, 10,
];

const S_BLOCK_REVERSE: [u32; 16] = [
    9 , 7 , 1 , 11, 0 , 6 , 3 , 8 ,
    10, 5 , 15, 12, 2 , 13, 14, 4 ,
];

const P_BLOCK_STRAIGHT: [u32; 32] = [
    16, 7 , 20, 21, 29, 12, 28, 17,
    1 , 15, 23, 26, 5 , 18, 31, 10,
    2 , 8 , 24, 14, 0 , 27, 3 , 9 ,
    19, 13, 30, 6 , 22, 11, 4 , 25,
];

const P_BLOCK_REVERSE: [u32; 32] = [
    20, 8 , 16, 22, 30, 12, 27, 1 ,
    17, 23, 15, 29, 5 , 25, 19, 9 ,
    0 , 7 , 13, 24, 2 , 3 , 28, 10,
    18, 31, 11, 21, 6 , 4 , 26, 14,
];

fn right_cycleshift(num: u32, shiftval: u32) -> u32 {
    match shiftval % 32 {
        0     => num,
        shift => (num >> shift) | (num << (32 - shift))
    }
}

fn generate_round_keys(masterkey: u32, rounds: i32) -> Vec<u32> {
    (0..rounds)
        .map(|i| {
            let shifted = right_cycleshift(masterkey, i as u32);
            shifted ^ ((i as u32).wrapping_mul(0x9E3779B9))
        })
        .collect()
}

fn do_s_block(mut bytes: u32, s_block: &[u32]) -> u32 {
    (0..8)
        .fold(0, |mut res, i| {
            res |= s_block[(bytes & 0xF) as usize] << (i * 4);
            bytes >>= 4;
            res
        })
}

fn do_p_block(mut bytes: u32, p_block: &[u32]) -> u32 {
    (0..32)
        .fold(0, |mut res, i| {
            res |= (bytes & 0b1) << p_block[i];
            bytes >>= 1;
            res
        })
}

fn round_enc(block: u32, roundkey: u32) -> u32 {
    let mut res = block ^ roundkey;
    res = do_s_block(res, &S_BLOCK_STRAIGHT);
    do_p_block(res, &P_BLOCK_STRAIGHT)
}

fn round_dec(block: u32, roundkey: u32) -> u32 {
    let mut res = do_p_block(block, &P_BLOCK_REVERSE);
    res = do_s_block(res, &S_BLOCK_REVERSE);
    res ^ roundkey
}

fn enc(block: u32, masterkey: u32, rounds: i32) -> u32 {
    let roundkeys = generate_round_keys(masterkey, rounds);
    (0..rounds)
        .fold(block, |state, i| {
            round_enc(state, roundkeys[i as usize])
        })
}

fn dec(block: u32, masterkey: u32, rounds: i32) -> u32 {
    let roundkeys = generate_round_keys(masterkey, rounds);
    (0..rounds)
        .rev()
        .fold(block, |state, i| {
            round_dec(state, roundkeys[i as usize])
        })
}

fn main() {
    let block     = 555;
    let masterkey = 0xDEADBABE;
    let rounds    = 10;
    let mut state = enc(block, masterkey, rounds);
    state = dec(state, masterkey, rounds);
    println!("block = {}, state = {}", block, state); // block = 555, state = 555
}
