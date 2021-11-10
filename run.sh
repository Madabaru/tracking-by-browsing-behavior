
FILE=tmp/output
if [ -f "$FILE" ]; then
    rm tmp/output
fi

cargo build --release && ./target/release/tracking-by-browsing-behavior  --delay_limit 1800 --fields website category 

python evaluate.py

