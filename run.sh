cargo build --release && ./target/release/tracking-by-browsing-behavior  --metric euclidean 
cargo build --release && ./target/release/tracking-by-browsing-behavior  --metric manhattan
cargo build --release && ./target/release/tracking-by-browsing-behavior  --metric cosine
cargo build --release && ./target/release/tracking-by-browsing-behavior  --metric non_intersection
cargo build --release && ./target/release/tracking-by-browsing-behavior  --metric bhattacharyya
cargo build --release && ./target/release/tracking-by-browsing-behavior  --metric kullbrack_leibler
cargo build --release && ./target/release/tracking-by-browsing-behavior  --metric total_variation
cargo build --release && ./target/release/tracking-by-browsing-behavior  --metric jeffries_matusita
cargo build --release && ./target/release/tracking-by-browsing-behavior  --metric chi_quared

cargo build --release && ./target/release/tracking-by-browsing-behavior  --fields url
cargo build --release && ./target/release/tracking-by-browsing-behavior  --fields category
cargo build --release && ./target/release/tracking-by-browsing-behavior  --fields domain
cargo build --release && ./target/release/tracking-by-browsing-behavior  --fields url category
cargo build --release && ./target/release/tracking-by-browsing-behavior  --fields url domain
cargo build --release && ./target/release/tracking-by-browsing-behavior  --fields url category domain
cargo build --release && ./target/release/tracking-by-browsing-behavior  --fields url age
cargo build --release && ./target/release/tracking-by-browsing-behavior  --fields url gender
cargo build --release && ./target/release/tracking-by-browsing-behavior  --fields url hour 
cargo build --release && ./target/release/tracking-by-browsing-behavior  --fields url day
cargo build --release && ./target/release/tracking-by-browsing-behavior  --fields url click_rate
cargo build --release && ./target/release/tracking-by-browsing-behavior  --fields url category domain age
cargo build --release && ./target/release/tracking-by-browsing-behavior  --fields url category domain age gender
cargo build --release && ./target/release/tracking-by-browsing-behavior  --fields url category domain gender
cargo build --release && ./target/release/tracking-by-browsing-behavior  --fields url category domain gender age hour day

cargo build --release && ./target/release/tracking-by-browsing-behavior  --click_trace_sample_size 1
cargo build --release && ./target/release/tracking-by-browsing-behavior  --click_trace_sample_size 2
cargo build --release && ./target/release/tracking-by-browsing-behavior  --click_trace_sample_size 3
cargo build --release && ./target/release/tracking-by-browsing-behavior  --click_trace_sample_size 4
cargo build --release && ./target/release/tracking-by-browsing-behavior  --click_trace_sample_size 5
cargo build --release && ./target/release/tracking-by-browsing-behavior  --click_trace_sample_size 6
cargo build --release && ./target/release/tracking-by-browsing-behavior  --click_trace_sample_size 7
cargo build --release && ./target/release/tracking-by-browsing-behavior  --click_trace_sample_size 8
cargo build --release && ./target/release/tracking-by-browsing-behavior  --click_trace_sample_size 9
cargo build --release && ./target/release/tracking-by-browsing-behavior  --click_trace_sample_size 10
cargo build --release && ./target/release/tracking-by-browsing-behavior  --click_trace_sample_size 15
cargo build --release && ./target/release/tracking-by-browsing-behavior  --click_trace_sample_size 20
cargo build --release && ./target/release/tracking-by-browsing-behavior  --click_trace_sample_size 25
cargo build --release && ./target/release/tracking-by-browsing-behavior  --click_trace_sample_size 50

cargo build --release && ./target/release/tracking-by-browsing-behavior  --client_sample_size 10
cargo build --release && ./target/release/tracking-by-browsing-behavior  --client_sample_size 20
cargo build --release && ./target/release/tracking-by-browsing-behavior  --client_sample_size 30
cargo build --release && ./target/release/tracking-by-browsing-behavior  --client_sample_size 40
cargo build --release && ./target/release/tracking-by-browsing-behavior  --client_sample_size 50
cargo build --release && ./target/release/tracking-by-browsing-behavior  --client_sample_size 100
cargo build --release && ./target/release/tracking-by-browsing-behavior  --client_sample_size 200
cargo build --release && ./target/release/tracking-by-browsing-behavior  --client_sample_size 400
cargo build --release && ./target/release/tracking-by-browsing-behavior  --client_sample_size 500
cargo build --release && ./target/release/tracking-by-browsing-behavior  --client_sample_size 1000

cargo build --release && ./target/release/tracking-by-browsing-behavior  --typical true --click_trace_sample_size 2
cargo build --release && ./target/release/tracking-by-browsing-behavior  --click_trace_sample_size 2
cargo build --release && ./target/release/tracking-by-browsing-behavior  --typical true --click_trace_sample_size 5
cargo build --release && ./target/release/tracking-by-browsing-behavior  --click_trace_sample_size 5
cargo build --release && ./target/release/tracking-by-browsing-behavior  --typical true --click_trace_sample_size 10
cargo build --release && ./target/release/tracking-by-browsing-behavior  --click_trace_sample_size 10
cargo build --release && ./target/release/tracking-by-browsing-behavior  --typical true --click_trace_sample_size 20
cargo build --release && ./target/release/tracking-by-browsing-behavior  --click_trace_sample_size 20
cargo build --release && ./target/release/tracking-by-browsing-behavior  --typical true --click_trace_sample_size 40
cargo build --release && ./target/release/tracking-by-browsing-behavior  --click_trace_sample_size 40

cargo build --release && ./target/release/tracking-by-browsing-behavior  --client_sample_size 10 --approach sequence
cargo build --release && ./target/release/tracking-by-browsing-behavior  --client_sample_size 20 --approach sequence
cargo build --release && ./target/release/tracking-by-browsing-behavior  --client_sample_size 30 --approach sequence
cargo build --release && ./target/release/tracking-by-browsing-behavior  --client_sample_size 40 --approach sequence
cargo build --release && ./target/release/tracking-by-browsing-behavior  --client_sample_size 50 --approach sequence
cargo build --release && ./target/release/tracking-by-browsing-behavior  --client_sample_size 100 --approach sequence
cargo build --release && ./target/release/tracking-by-browsing-behavior  --client_sample_size 200 --approach sequence
cargo build --release && ./target/release/tracking-by-browsing-behavior  --client_sample_size 400 --approach sequence
cargo build --release && ./target/release/tracking-by-browsing-behavior  --client_sample_size 500 --approach sequence
cargo build --release && ./target/release/tracking-by-browsing-behavior  --client_sample_size 1000 --approach sequence

cargo build --release && ./target/release/tracking-by-browsing-behavior  --typical true --click_trace_sample_size 2 --approach sequence
cargo build --release && ./target/release/tracking-by-browsing-behavior  --click_trace_sample_size 2 --approach sequence
cargo build --release && ./target/release/tracking-by-browsing-behavior  --typical true --click_trace_sample_size 5 --approach sequence
cargo build --release && ./target/release/tracking-by-browsing-behavior  --click_trace_sample_size 5 --approach sequence
cargo build --release && ./target/release/tracking-by-browsing-behavior  --typical true --click_trace_sample_size 10 --approach sequence
cargo build --release && ./target/release/tracking-by-browsing-behavior  --click_trace_sample_size 10 --approach sequence
cargo build --release && ./target/release/tracking-by-browsing-behavior  --typical true --click_trace_sample_size 20 --approach sequence
cargo build --release && ./target/release/tracking-by-browsing-behavior  --click_trace_sample_size 20 --approach sequence
cargo build --release && ./target/release/tracking-by-browsing-behavior  --typical true --click_trace_sample_size 40 --approach sequence
cargo build --release && ./target/release/tracking-by-browsing-behavior  --click_trace_sample_size 40 --approach sequence

cargo build --release && ./target/release/tracking-by-browsing-behavior  --click_trace_sample_size 1 --approach sequence
cargo build --release && ./target/release/tracking-by-browsing-behavior  --click_trace_sample_size 2 --approach sequence
cargo build --release && ./target/release/tracking-by-browsing-behavior  --click_trace_sample_size 3 --approach sequence
cargo build --release && ./target/release/tracking-by-browsing-behavior  --click_trace_sample_size 4 --approach sequence
cargo build --release && ./target/release/tracking-by-browsing-behavior  --click_trace_sample_size 5 --approach sequence
cargo build --release && ./target/release/tracking-by-browsing-behavior  --click_trace_sample_size 6 --approach sequence
cargo build --release && ./target/release/tracking-by-browsing-behavior  --click_trace_sample_size 7 --approach sequence
cargo build --release && ./target/release/tracking-by-browsing-behavior  --click_trace_sample_size 8 --approach sequence
cargo build --release && ./target/release/tracking-by-browsing-behavior  --click_trace_sample_size 9 --approach sequence
cargo build --release && ./target/release/tracking-by-browsing-behavior  --click_trace_sample_size 10 --approach sequence
cargo build --release && ./target/release/tracking-by-browsing-behavior  --click_trace_sample_size 15 --approach sequence
cargo build --release && ./target/release/tracking-by-browsing-behavior  --click_trace_sample_size 20 --approach sequence
cargo build --release && ./target/release/tracking-by-browsing-behavior  --click_trace_sample_size 25 --approach sequence
cargo build --release && ./target/release/tracking-by-browsing-behavior  --click_trace_sample_size 50 --approach sequence

cargo build --release && ./target/release/tracking-by-browsing-behavior  --approach sequence --scope local
cargo build --release && ./target/release/tracking-by-browsing-behavior  --approach sequence --scope local --strategy nw
cargo build --release && ./target/release/tracking-by-browsing-behavior  --approach sequence --scope global
cargo build --release && ./target/release/tracking-by-browsing-behavior  --approach sequence --scope global --strategy nw

cargo build --release && ./target/release/tracking-by-browsing-behavior  --fields url --approach sequence
cargo build --release && ./target/release/tracking-by-browsing-behavior  --fields category --approach sequence
cargo build --release && ./target/release/tracking-by-browsing-behavior  --fields domain --approach sequence
cargo build --release && ./target/release/tracking-by-browsing-behavior  --fields url category --approach sequence
cargo build --release && ./target/release/tracking-by-browsing-behavior  --fields url domain --approach sequence
cargo build --release && ./target/release/tracking-by-browsing-behavior  --fields url category domain --approach sequence
cargo build --release && ./target/release/tracking-by-browsing-behavior  --fields url age --approach sequence
cargo build --release && ./target/release/tracking-by-browsing-behavior  --fields url gender --approach sequence
cargo build --release && ./target/release/tracking-by-browsing-behavior  --fields url hour --approach sequence
cargo build --release && ./target/release/tracking-by-browsing-behavior  --fields url day --approach sequence
cargo build --release && ./target/release/tracking-by-browsing-behavior  --fields url click_rate --approach sequence
cargo build --release && ./target/release/tracking-by-browsing-behavior  --fields url category domain age --approach sequence
cargo build --release && ./target/release/tracking-by-browsing-behavior  --fields url category domain age gender --approach sequence
cargo build --release && ./target/release/tracking-by-browsing-behavior  --fields url category domain gender --approach sequence
cargo build --release && ./target/release/tracking-by-browsing-behavior  --fields url category domain gender age hour day --approach sequence

cargo build --release && ./target/release/tracking-by-browsing-behavior  --approach sequence --strategy nw --scope local --scoring_matrix 1 -1 0 0
cargo build --release && ./target/release/tracking-by-browsing-behavior  --approach sequence --strategy sw --scope local --scoring_matrix 1 -1 0 0
cargo build --release && ./target/release/tracking-by-browsing-behavior  --approach sequence --strategy nw --scope global --scoring_matrix 1 -1 0 0
cargo build --release && ./target/release/tracking-by-browsing-behavior  --approach sequence --strategy sw --scope global --scoring_matrix 1 -1 0 0

cargo build --release && ./target/release/tracking-by-browsing-behavior  --approach sequence --strategy nw --scope local --scoring_matrix 1 -1 -1 -1
cargo build --release && ./target/release/tracking-by-browsing-behavior  --approach sequence --strategy sw --scope local --scoring_matrix 1 -1 -1 -1
cargo build --release && ./target/release/tracking-by-browsing-behavior  --approach sequence --strategy nw --scope global --scoring_matrix 1 -1 -1 -1
cargo build --release && ./target/release/tracking-by-browsing-behavior  --approach sequence --strategy sw --scope global --scoring_matrix 1 -1 -1 -1

cargo build --release && ./target/release/tracking-by-browsing-behavior  --approach sequence --strategy nw --scope local --scoring_matrix 1 -2 -1 -1
cargo build --release && ./target/release/tracking-by-browsing-behavior  --approach sequence --strategy sw --scope local --scoring_matrix 1 -2 -1 -1
cargo build --release && ./target/release/tracking-by-browsing-behavior  --approach sequence --strategy nw --scope global --scoring_matrix 1 -2 -1 -1
cargo build --release && ./target/release/tracking-by-browsing-behavior  --approach sequence --strategy sw --scope global --scoring_matrix 1 -2 -1 -1

cargo build --release && ./target/release/tracking-by-browsing-behavior  --approach sequence --strategy nw --scope local --scoring_matrix 1 -1 -2 -2
cargo build --release && ./target/release/tracking-by-browsing-behavior  --approach sequence --strategy sw --scope local --scoring_matrix 1 -1 -2 -2
cargo build --release && ./target/release/tracking-by-browsing-behavior  --approach sequence --strategy nw --scope global --scoring_matrix 1 -1 -2 -2
cargo build --release && ./target/release/tracking-by-browsing-behavior  --approach sequence --strategy sw --scope global --scoring_matrix 1 -1 -2 -2

cargo build --release && ./target/release/tracking-by-browsing-behavior  --approach sequence --strategy nw --scope local --scoring_matrix 1 -2 -2 -2
cargo build --release && ./target/release/tracking-by-browsing-behavior  --approach sequence --strategy sw --scope local --scoring_matrix 1 -2 -2 -2
cargo build --release && ./target/release/tracking-by-browsing-behavior  --approach sequence --strategy nw --scope global --scoring_matrix 1 -2 -2 -2
cargo build --release && ./target/release/tracking-by-browsing-behavior  --approach sequence --strategy sw --scope global --scoring_matrix 1 -2 -2 -2

cargo build --release && ./target/release/tracking-by-browsing-behavior  --approach sequence --strategy nw --scope local --scoring_matrix 2 -1 -1 -1
cargo build --release && ./target/release/tracking-by-browsing-behavior  --approach sequence --strategy sw --scope local --scoring_matrix 2 -1 -1 -1
cargo build --release && ./target/release/tracking-by-browsing-behavior  --approach sequence --strategy nw --scope global --scoring_matrix 2 -1 -1 -1
cargo build --release && ./target/release/tracking-by-browsing-behavior  --approach sequence --strategy sw --scope global --scoring_matrix 2 -1 -1 -1
