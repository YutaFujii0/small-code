#!bin/bash
echo '----------------------------------';
echo 'Iterate 10 times with python code';
cd python;

hyperfine --warmup 5 "python3 search.py startup";
cd ../;

echo '----------------------------------';
echo 'Iterate 10 times with Rust code(debug mode)';
cd rust-normal;

cargo build $1>/dev/null;
hyperfine --warmup 5 "target/debug/search startup";
cd ../;


echo '----------------------------------';
echo 'Iterate 10 times with Rust code(release mode)';
cd rust-normal;

cargo build --release $1>/dev/null;
hyperfine --warmup 5 "target/release/search startup";
cd ../;


echo '----------------------------------';
echo 'Iterate 10 times with inverted-index Rust code(debug mode)';
cd rust-inverted-index;

cargo build --release $1>/dev/null;
hyperfine --warmup 5 "target/debug/inverted_index startup";
cd ../;


echo '----------------------------------';
echo 'Iterate 10 times with inverted-index Rust code(release mode)';
cd rust-inverted-index;

cargo build --release $1>/dev/null;
hyperfine --warmup 5 "target/release/inverted_index startup";
cd ../;
