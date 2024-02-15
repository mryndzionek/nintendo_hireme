// https://www.nerd.nintendo.com/files/HireMe.html

use std::collections::HashSet;
use std::env;
use std::time::Instant;

static CONFUSION: [u8; 512] = [
    0xac, 0xd1, 0x25, 0x94, 0x1f, 0xb3, 0x33, 0x28, 0x7c, 0x2b, 0x17, 0xbc, 0xf6, 0xb0, 0x55, 0x5d,
    0x8f, 0xd2, 0x48, 0xd4, 0xd3, 0x78, 0x62, 0x1a, 0x02, 0xf2, 0x01, 0xc9, 0xaa, 0xf0, 0x83, 0x71,
    0x72, 0x4b, 0x6a, 0xe8, 0xe9, 0x42, 0xc0, 0x53, 0x63, 0x66, 0x13, 0x4a, 0xc1, 0x85, 0xcf, 0x0c,
    0x24, 0x76, 0xa5, 0x6e, 0xd7, 0xa1, 0xec, 0xc6, 0x04, 0xc2, 0xa2, 0x5c, 0x81, 0x92, 0x6c, 0xda,
    0xc6, 0x86, 0xba, 0x4d, 0x39, 0xa0, 0x0e, 0x8c, 0x8a, 0xd0, 0xfe, 0x59, 0x96, 0x49, 0xe6, 0xea,
    0x69, 0x30, 0x52, 0x1c, 0xe0, 0xb2, 0x05, 0x9b, 0x10, 0x03, 0xa8, 0x64, 0x51, 0x97, 0x02, 0x09,
    0x8e, 0xad, 0xf7, 0x36, 0x47, 0xab, 0xce, 0x7f, 0x56, 0xca, 0x00, 0xe3, 0xed, 0xf1, 0x38, 0xd8,
    0x26, 0x1c, 0xdc, 0x35, 0x91, 0x43, 0x2c, 0x74, 0xb4, 0x61, 0x9d, 0x5e, 0xe9, 0x4c, 0xbf, 0x77,
    0x16, 0x1e, 0x21, 0x1d, 0x2d, 0xa9, 0x95, 0xb8, 0xc3, 0x8d, 0xf8, 0xdb, 0x34, 0xe1, 0x84, 0xd6,
    0x0b, 0x23, 0x4e, 0xff, 0x3c, 0x54, 0xa7, 0x78, 0xa4, 0x89, 0x33, 0x6d, 0xfb, 0x79, 0x27, 0xc4,
    0xf9, 0x40, 0x41, 0xdf, 0xc5, 0x82, 0x93, 0xdd, 0xa6, 0xef, 0xcd, 0x8d, 0xa3, 0xae, 0x7a, 0xb6,
    0x2f, 0xfd, 0xbd, 0xe5, 0x98, 0x66, 0xf3, 0x4f, 0x57, 0x88, 0x90, 0x9c, 0x0a, 0x50, 0xe7, 0x15,
    0x7b, 0x58, 0xbc, 0x07, 0x68, 0x3a, 0x5f, 0xee, 0x32, 0x9f, 0xeb, 0xcc, 0x18, 0x8b, 0xe2, 0x57,
    0xb7, 0x49, 0x37, 0xde, 0xf5, 0x99, 0x67, 0x5b, 0x3b, 0xbb, 0x3d, 0xb5, 0x2d, 0x19, 0x2e, 0x0d,
    0x93, 0xfc, 0x7e, 0x06, 0x08, 0xbe, 0x3f, 0xd9, 0x2a, 0x70, 0x9a, 0xc8, 0x7d, 0xd8, 0x46, 0x65,
    0x22, 0xf4, 0xb9, 0xa2, 0x6f, 0x12, 0x1b, 0x14, 0x45, 0xc7, 0x87, 0x31, 0x60, 0x29, 0xf7, 0x73,
    0x2c, 0x97, 0x72, 0xcd, 0x89, 0xa6, 0x88, 0x4c, 0xe8, 0x83, 0xeb, 0x59, 0xca, 0x50, 0x3f, 0x27,
    0x4e, 0xae, 0x43, 0xd5, 0x6e, 0xd0, 0x99, 0x7b, 0x7c, 0x40, 0x0c, 0x52, 0x86, 0xc1, 0x46, 0x12,
    0x5a, 0x28, 0xa8, 0xbb, 0xcb, 0xf0, 0x11, 0x95, 0x26, 0x0d, 0x34, 0x66, 0x22, 0x18, 0x6f, 0x51,
    0x9b, 0x3b, 0xda, 0xec, 0x5e, 0x00, 0x2a, 0xf5, 0x8f, 0x61, 0xba, 0x96, 0xb3, 0xd1, 0x30, 0xdc,
    0x33, 0x75, 0xe9, 0x6d, 0xc8, 0xa1, 0x3a, 0x3e, 0x5f, 0x9d, 0xfd, 0xa9, 0x31, 0x9f, 0xaa, 0x85,
    0x2f, 0x92, 0xaf, 0x67, 0x78, 0xa5, 0xab, 0x03, 0x21, 0x4f, 0xb9, 0xad, 0xfe, 0xf3, 0x42, 0xfc,
    0x17, 0xd7, 0xee, 0xa3, 0xd8, 0x80, 0x14, 0x2e, 0xa0, 0x47, 0x55, 0xc4, 0xff, 0xe5, 0x13, 0x3f,
    0x81, 0xb6, 0x7a, 0x94, 0xd0, 0xb5, 0x54, 0xbf, 0x91, 0xa7, 0x37, 0xf1, 0x6b, 0xc9, 0x1b, 0xb1,
    0x3c, 0xb6, 0xd9, 0x32, 0x24, 0x8d, 0xf2, 0x82, 0xb4, 0xf9, 0xdb, 0x7d, 0x44, 0xfb, 0x1e, 0xd4,
    0xea, 0x5d, 0x35, 0x69, 0x23, 0x71, 0x57, 0x01, 0x06, 0xe4, 0x55, 0x9a, 0xa4, 0x58, 0x56, 0xc7,
    0x4a, 0x8c, 0x8a, 0xd6, 0x6a, 0x49, 0x70, 0xc5, 0x8e, 0x0a, 0x62, 0xdc, 0x29, 0x4b, 0x42, 0x41,
    0xcb, 0x2b, 0xb7, 0xce, 0x08, 0xa1, 0x76, 0x1d, 0x1a, 0xb8, 0xe3, 0xcc, 0x7e, 0x48, 0x20, 0xe6,
    0xf8, 0x45, 0x93, 0xde, 0xc3, 0x63, 0x0f, 0xb0, 0xac, 0x5c, 0xba, 0xdf, 0x07, 0x77, 0xe7, 0x4e,
    0x1f, 0x28, 0x10, 0x6c, 0x59, 0xd3, 0xdd, 0x2d, 0x65, 0x39, 0xb2, 0x74, 0x84, 0x3d, 0xf4, 0xbd,
    0xc7, 0x79, 0x60, 0x0b, 0x4d, 0x33, 0x36, 0x25, 0xbc, 0xe0, 0x09, 0xcf, 0x5b, 0xe2, 0x38, 0x9e,
    0xc0, 0xef, 0xd2, 0x16, 0x05, 0xbe, 0x53, 0xf7, 0xc2, 0xc6, 0xa2, 0x24, 0x98, 0x1c, 0xad, 0x04,
];

static DIFFUSION: [u32; 32] = [
    0xf26cb481, 0x16a5dc92, 0x3c5ba924, 0x79b65248, 0x2fc64b18, 0x615acd29, 0xc3b59a42, 0x976b2584,
    0x6cf281b4, 0xa51692dc, 0x5b3c24a9, 0xb6794852, 0xc62f184b, 0x5a6129cd, 0xb5c3429a, 0x6b978425,
    0xb481f26c, 0xdc9216a5, 0xa9243c5b, 0x524879b6, 0x4b182fc6, 0xcd29615a, 0x9a42c3b5, 0x2584976b,
    0x81b46cf2, 0x92dca516, 0x24a95b3c, 0x4852b679, 0x184bc62f, 0x29cd5a61, 0x429ab5c3, 0x84256b97,
];

struct Product<const N: usize> {
    finished: bool,
    elements: [Vec<u8>; N],
    indexes: [usize; N],
}

impl<const N: usize> Product<N> {
    pub fn new(elements: [Vec<u8>; N]) -> Self {
        Self {
            finished: false,
            indexes: [0; N],
            elements,
        }
    }
}

impl<const N: usize> Iterator for Product<N> {
    type Item = [u8; N];

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            None
        } else {
            let mut r = [0; N];
            for (i, x) in r.iter_mut().enumerate() {
                *x = self.elements[i][self.indexes[i]]
            }

            for i in 0..N {
                self.indexes[N - 1 - i] += 1;
                if self.indexes[N - 1 - i] == self.elements[N - 1 - i].len() {
                    if i == N - 1 {
                        self.finished = true;
                    }
                    self.indexes[N - 1 - i] = 0;
                } else {
                    break;
                }
            }
            Some(r)
        }
    }
}

fn forward(input: [u8; 32]) -> [u8; 16] {
    let mut output = [0; 32];
    let mut tmp = input;

    for _ in 0..256 {
        for (tmp, output) in tmp.iter_mut().zip(output.iter_mut()) {
            *output = CONFUSION[*tmp as usize];
            *tmp = 0;
        }

        for (j, tmp) in tmp.iter_mut().enumerate() {
            for (k, _) in output.iter().enumerate() {
                *tmp ^= output[k] * (DIFFUSION[j] >> k & 1) as u8;
            }
        }
    }

    (0..16)
        .map(|i| CONFUSION[tmp[i * 2] as usize] ^ CONFUSION[tmp[i * 2 + 1] as usize + 256])
        .collect::<Vec<u8>>()
        .try_into()
        .unwrap()
}

fn inv_vec(vi: Vec<u8>) -> Vec<Vec<u8>> {
    let mut vo = vec![Vec::new(); vi.len()];

    for (j, i) in vi.into_iter().enumerate() {
        vo[i as usize].push(j as u8)
    }
    vo
}

#[rustfmt::skip]
#[inline(always)]
fn d_box(input: [u8; 32]) -> [u8; 32] {
    let mut output: [u8; 32] = [0; 32];
    
    output[0] = input[0]^input[7]^input[10]^input[12]^input[13]^input[15]^input[18]^input[19]^input[21]^input[22]^input[25]^input[28]^input[29]^input[30]^input[31];
    output[1] = input[1]^input[4]^input[7]^input[10]^input[11]^input[12]^input[14]^input[15]^input[16]^input[18]^input[21]^input[23]^input[25]^input[26]^input[28];
    output[2] = input[2]^input[5]^input[8]^input[11]^input[13]^input[15]^input[16]^input[17]^input[19]^input[20]^input[22]^input[26]^input[27]^input[28]^input[29];
    output[3] = input[3]^input[6]^input[9]^input[12]^input[14]^input[17]^input[18]^input[20]^input[21]^input[23]^input[24]^input[27]^input[28]^input[29]^input[30];
    output[4] = input[3]^input[4]^input[8]^input[9]^input[11]^input[14]^input[17]^input[18]^input[22]^input[23]^input[24]^input[25]^input[26]^input[27]^input[29];
    output[5] = input[0]^input[3]^input[5]^input[8]^input[10]^input[11]^input[14]^input[15]^input[17]^input[19]^input[20]^input[22]^input[24]^input[29]^input[30];
    output[6] = input[1]^input[6]^input[9]^input[11]^input[12]^input[15]^input[16]^input[18]^input[20]^input[21]^input[23]^input[24]^input[25]^input[30]^input[31];
    output[7] = input[2]^input[7]^input[8]^input[10]^input[13]^input[16]^input[17]^input[19]^input[21]^input[22]^input[24]^input[25]^input[26]^input[28]^input[31];
    output[8] = input[2]^input[4]^input[5]^input[7]^input[8]^input[15]^input[17]^input[20]^input[21]^input[22]^input[23]^input[26]^input[27]^input[29]^input[30];
    output[9] = input[2]^input[3]^input[4]^input[6]^input[7]^input[9]^input[12]^input[15]^input[17]^input[18]^input[20]^input[24]^input[26]^input[29]^input[31];
    output[10] = input[0]^input[3]^input[5]^input[7]^input[10]^input[13]^input[18]^input[19]^input[20]^input[21]^input[24]^input[25]^input[27]^input[28]^input[30];
    output[11] = input[1]^input[4]^input[6]^input[11]^input[14]^input[16]^input[19]^input[20]^input[21]^input[22]^input[25]^input[26]^input[28]^input[29]^input[31];
    output[12] = input[0]^input[1]^input[3]^input[6]^input[11]^input[12]^input[16]^input[17]^input[18]^input[19]^input[21]^input[25]^input[26]^input[30]^input[31];
    output[13] = input[0]^input[2]^input[3]^input[6]^input[7]^input[8]^input[11]^input[13]^input[16]^input[21]^input[22]^input[25]^input[27]^input[28]^input[30];
    output[14] = input[1]^input[3]^input[4]^input[7]^input[9]^input[14]^input[16]^input[17]^input[22]^input[23]^input[24]^input[26]^input[28]^input[29]^input[31];
    output[15] = input[0]^input[2]^input[5]^input[10]^input[15]^input[16]^input[17]^input[18]^input[20]^input[23]^input[24]^input[25]^input[27]^input[29]^input[30];
    output[16] = input[2]^input[3]^input[5]^input[6]^input[9]^input[12]^input[13]^input[14]^input[15]^input[16]^input[23]^input[26]^input[28]^input[29]^input[31];
    output[17] = input[0]^input[2]^input[5]^input[7]^input[9]^input[10]^input[12]^input[17]^input[20]^input[23]^input[26]^input[27]^input[28]^input[30]^input[31];
    output[18] = input[0]^input[1]^input[3]^input[4]^input[6]^input[10]^input[11]^input[12]^input[13]^input[18]^input[21]^input[24]^input[27]^input[29]^input[31];
    output[19] = input[1]^input[2]^input[4]^input[5]^input[7]^input[8]^input[11]^input[12]^input[13]^input[14]^input[19]^input[22]^input[25]^input[28]^input[30];
    output[20] = input[1]^input[2]^input[6]^input[7]^input[8]^input[9]^input[10]^input[11]^input[13]^input[19]^input[20]^input[24]^input[25]^input[27]^input[30];
    output[21] = input[1]^input[3]^input[4]^input[6]^input[8]^input[13]^input[14]^input[16]^input[19]^input[21]^input[24]^input[26]^input[27]^input[30]^input[31];
    output[22] = input[0]^input[2]^input[4]^input[5]^input[7]^input[8]^input[9]^input[14]^input[15]^input[17]^input[22]^input[25]^input[27]^input[28]^input[31];
    output[23] = input[0]^input[1]^input[3]^input[5]^input[6]^input[8]^input[9]^input[10]^input[12]^input[15]^input[18]^input[23]^input[24]^input[26]^input[29];
    output[24] = input[1]^input[4]^input[5]^input[6]^input[7]^input[10]^input[11]^input[13]^input[14]^input[18]^input[20]^input[21]^input[23]^input[24]^input[31];
    output[25] = input[1]^input[2]^input[4]^input[8]^input[10]^input[13]^input[15]^input[18]^input[19]^input[20]^input[22]^input[23]^input[25]^input[28]^input[31];
    output[26] = input[2]^input[3]^input[4]^input[5]^input[8]^input[9]^input[11]^input[12]^input[14]^input[16]^input[19]^input[21]^input[23]^input[26]^input[29];
    output[27] = input[0]^input[3]^input[4]^input[5]^input[6]^input[9]^input[10]^input[12]^input[13]^input[15]^input[17]^input[20]^input[22]^input[27]^input[30];
    output[28] = input[0]^input[1]^input[2]^input[3]^input[5]^input[9]^input[10]^input[14]^input[15]^input[16]^input[17]^input[19]^input[22]^input[27]^input[28];
    output[29] = input[0]^input[5]^input[6]^input[9]^input[11]^input[12]^input[14]^input[16]^input[18]^input[19]^input[22]^input[23]^input[24]^input[27]^input[29];
    output[30] = input[0]^input[1]^input[6]^input[7]^input[8]^input[10]^input[12]^input[13]^input[15]^input[17]^input[19]^input[20]^input[23]^input[25]^input[30];
    output[31] = input[0]^input[1]^input[2]^input[4]^input[7]^input[8]^input[9]^input[11]^input[13]^input[14]^input[16]^input[18]^input[21]^input[26]^input[31];
    
    output
  }

fn backward(target: [u8; 16], num_solutions: usize) {
    let inv_a = inv_vec(CONFUSION[0..256].to_vec());

    let forbidden: [u8; 16] = inv_a
        .iter()
        .enumerate()
        .filter(|(_, v)| v.is_empty())
        .map(|x| x.0 as u8)
        .collect::<Vec<u8>>()
        .try_into()
        .unwrap();

    let mut el_a = Vec::new();
    let mut el_b = Vec::new();

    for i in 0..target.len() {
        let mut va = Vec::new();
        let mut vb = Vec::new();

        for b in 0..256 {
            for a in inv_a[(target[i] ^ CONFUSION[b + 256]) as usize].iter() {
                va.push(*a);
                vb.push(b as u8);
            }
        }

        el_a.push(va);
        el_b.push(vb);
    }

    let gena: Product<16> = Product::new(el_a.try_into().unwrap());
    let genb: Product<16> = Product::new(el_b.try_into().unwrap());

    let mut count: usize = 0;
    let mut solution: usize = 0;
    let mut solutions = HashSet::new();

    for (a, b) in gena.zip(genb) {
        count += 1;
        let input: [u8; 32] = a
            .into_iter()
            .zip(b)
            .flat_map(|(a, b)| [a, b])
            .collect::<Vec<u8>>()
            .try_into()
            .unwrap();

        let mut test: [u8; 16] = [0; 16];
        for i in 0..16 {
            test[i] = CONFUSION[input[2 * i] as usize] ^ CONFUSION[input[2 * i + 1] as usize + 256];
        }
        assert!(test == target);

        let mut stack: Vec<(usize, [u8; 32])> = vec![(0, input)];

        while let Some((i, d)) = stack.pop() {
            if i == 256 {
                solution += 1;
                println!("Found solution {} ({}):", solution, count);
                let output = forward(d);
                assert!(output == target);
                if !solutions.insert(d) {
                    println!("Duplicate solution!");
                }
                println!("{:?}", d);
                if solution == num_solutions {
                    return;
                } else {
                    continue;
                }
            } else {
                let c = d_box(d);

                if c.iter().any(|v| forbidden.contains(v)) {
                    continue;
                }

                let de = c.map(|v| inv_a[v as usize].clone());
                let gend = Product::new(de);
                stack.append(&mut gend.map(|v| (i + 1, v)).collect::<Vec<(usize, [u8; 32])>>());
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let target: [u8; 16] = if args.len() == 2 {
        [args[1].as_bytes(), &[0].repeat(16 - args[1].len())]
            .concat()
            .try_into()
            .unwrap()
    } else {
        ["Hire me!!!!!!!!".as_bytes(), &[0]]
            .concat()
            .try_into()
            .unwrap()
    };

    println!("Input: {:?}", target);

    let before = Instant::now();
    backward(target, 100000);
    println!("Elapsed time: {:.2?}", before.elapsed());
}
