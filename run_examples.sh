echo "running examples"
for example in $(ls examples/examples/*.rs); do
    example_name=$(basename $example .rs)
    cargo run -p uptest-examples --example $example_name
done
