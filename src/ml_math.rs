//use std::simd::f32x64 as f32x64; 

// Linear implementation of the basic softmax function in O(n^2) time
pub fn softmax(z: Vec<f32>) -> Vec<f32> {
    let mut sigma = Vec<f32>::with_capacity(z.len());
    for e_i in 0..z.len() {
        let mut acc : i32 = 0;

        for e_j in 0..z.len() {
            acc += e_j; 
        }

        sigma.append(e_i / acc);
    }

    sigma
}

// Implementation of softmax leveraging SIMD to increase performance while running on the CPU
//pub fn softmax_simd(vec: Vec<f32>) -> f32 {
//    let mut simdvecs = Vec<f32x64>::new()
//}
