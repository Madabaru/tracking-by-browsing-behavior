use rand::{prelude::SliceRandom, seq::IteratorRandom, Rng};
use std::collections::{BTreeMap, HashMap};

/// Samples a subset of users and a number of target traces indices.
///
/// The number of sampled users corresponds to the number of linkage attacks that will
/// be performed during evaluation. The sampled target traces are found in the second half of the
/// cronologically ordered history of each user.
pub fn gen_user_to_target_idx_map<R: Rng, T>(
    user_to_vector_map: &BTreeMap<u32, Vec<T>>,
    rng: &mut R,
    user_sample_size: usize,
    target_trace_sample_size: usize,
) -> HashMap<u32, Vec<usize>> {
    let user_list: Vec<u32> = user_to_vector_map.keys().cloned().collect();
    let sampled_clients_list = user_list.iter().choose_multiple(rng, user_sample_size);
    let mut user_to_target_idx_map = HashMap::<u32, Vec<usize>>::new();
    for client in sampled_clients_list.into_iter() {
        let traces_list = &user_to_vector_map.get(client).unwrap();
        let trace_len = traces_list.len();
        // Split history in 50%/50%
        let split_idx = trace_len / 2;
        let indices: Vec<usize> = (split_idx..trace_len).collect();
        let sampled_target_idx = indices
            .into_iter()
            .choose_multiple(rng, target_trace_sample_size);
        user_to_target_idx_map.insert(*client, sampled_target_idx);
    }
    return user_to_target_idx_map;
}

/// Samples the observed traces for each user and store sample indices in map.
///
/// The sampled trace indices are stored in a tree map because the tree map stores the keys in a fixed order.
/// The order is important for consistent sampling and reproducability. The sampled observed traces are found in
/// the first half of the cronologically ordered history of each user. By setting the trace_sample_size to 0, 50%
/// of the entire history are sampled.
pub fn gen_user_to_sample_idx_map<R: Rng, T>(
    user_to_vector_map: &BTreeMap<u32, Vec<T>>,
    rng: &mut R,
    trace_sample_size: usize,
) -> HashMap<u32, Vec<usize>> {
    let mut user_to_sample_idx_map: HashMap<u32, Vec<usize>> = HashMap::new();
    for (client, traces_list) in user_to_vector_map.into_iter() {
        let client = client.clone();
        let trace_len = traces_list.len();
        let split_idx = trace_len / 2;
        let indices: Vec<usize> = (0..split_idx).collect();
        let sampled_idx = indices.into_iter().choose_multiple(rng, trace_sample_size);
        user_to_sample_idx_map.insert(client, sampled_idx);
    }
    user_to_sample_idx_map
}

/// Samples a test traces for each user from the observed traces.
///
/// When conducting the linkage attack, the target trace can be compared to all observed traces
/// or to a randomly selected trace out of the observed traces, called the test trace.
pub fn gen_user_to_test_idx_map<R: Rng>(
    user_to_sample_idx_map: &HashMap<u32, Vec<usize>>,
    rng: &mut R,
) -> HashMap<u32, usize> {
    let mut user_to_test_idx_map: HashMap<u32, usize> = HashMap::new();
    for (client, sample_idx_list) in user_to_sample_idx_map.into_iter() {
        let test_idx = sample_idx_list.choose(rng).unwrap();
        user_to_test_idx_map.insert(client.clone(), test_idx.clone());
    }
    user_to_test_idx_map
}
