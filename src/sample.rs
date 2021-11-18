use std::collections::{HashMap, BTreeMap};
use rand::{seq::IteratorRandom, Rng};

// Sample a subset of clients and a target click trace that the evaluation is based upon
pub fn gen_test_data<R: Rng, T>(
    client_to_vector_map: &BTreeMap<u32, Vec<T>>,
    rng: &mut R,
    client_sample_size: usize,
) -> HashMap<u32, usize> {
    let client_list: Vec<u32> = client_to_vector_map.keys().cloned().collect();
    // Randomly sample clients from the client list
    let sampled_clients_list = client_list.iter().choose_multiple(rng, client_sample_size);
    let mut client_to_target_idx_map = HashMap::<u32, usize>::new();
    for client in sampled_clients_list.into_iter() {
        let click_traces_list = &client_to_vector_map.get(client).unwrap();
        let click_trace_len = click_traces_list.len();
        // Split click history in 50%/50%
        let split_idx = click_trace_len / 2;
        let rand_target_idx: usize = rng.gen_range(split_idx..click_trace_len);
        client_to_target_idx_map.insert(*client, rand_target_idx);
    }
    return client_to_target_idx_map;
}

// Sample click traces for each client and store sample indices in map
pub fn get_train_data<R: Rng, T>(
    client_to_vector_map: &BTreeMap<u32, Vec<T>>,
    rng: &mut R,
    click_trace_sample_size: usize,
) -> HashMap<u32, Vec<usize>> {
    let mut client_to_sample_idx_map: HashMap<u32, Vec<usize>> = HashMap::new();
    for (client, click_traces_list) in client_to_vector_map.into_iter() {
        let client = client.clone();
        let click_trace_len = click_traces_list.len();
        let split_idx = click_trace_len / 2;
        let indices: Vec<usize> = (0..split_idx).collect();
        if click_trace_sample_size > 0 {
            let sampled_idx = indices
                .into_iter()
                .choose_multiple(rng, click_trace_sample_size);
            client_to_sample_idx_map.insert(client, sampled_idx);
        } else {
            client_to_sample_idx_map.insert(client, indices);
        }
    }
    client_to_sample_idx_map
}
