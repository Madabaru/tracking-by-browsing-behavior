
FILE=tmp/output
if [ -f "$FILE" ]; then
    rm tmp/output
fi

cargo build --release && ./target/release/tracking-by-browsing-behavior  --metric euclidean 
python evaluate.py

FILE=tmp/output
if [ -f "$FILE" ]; then
    rm tmp/output
fi

cargo build --release && ./target/release/tracking-by-browsing-behavior  --metric manhatten
python evaluate.py

FILE=tmp/output
if [ -f "$FILE" ]; then
    rm tmp/output
fi

cargo build --release && ./target/release/tracking-by-browsing-behavior  --metric cosine
python evaluate.py

FILE=tmp/output
if [ -f "$FILE" ]; then
    rm tmp/output
fi

cargo build --release && ./target/release/tracking-by-browsing-behavior  --metric jaccard
python evaluate.py

FILE=tmp/output
if [ -f "$FILE" ]; then
    rm tmp/output
fi

cargo build --release && ./target/release/tracking-by-browsing-behavior  --metric bhattacharyya
python evaluate.py

FILE=tmp/output
if [ -f "$FILE" ]; then
    rm tmp/output
fi

cargo build --release && ./target/release/tracking-by-browsing-behavior  --metric kullbrack_leibler
python evaluate.py

FILE=tmp/output
if [ -f "$FILE" ]; then
    rm tmp/output
fi

cargo build --release && ./target/release/tracking-by-browsing-behavior  --metric total_variation
python evaluate.py

FILE=tmp/output
if [ -f "$FILE" ]; then
    rm tmp/output
fi

cargo build --release && ./target/release/tracking-by-browsing-behavior  --metric jeffries_matusita
python evaluate.py

FILE=tmp/output
if [ -f "$FILE" ]; then
    rm tmp/output
fi

cargo build --release && ./target/release/tracking-by-browsing-behavior  --metric chi_quared
python evaluate.py