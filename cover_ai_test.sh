#!/bin/sh

# cargo install cargo-tarpaulin
cover-agent \
  --source-file-path "src/utils/validator.rs" \
  --test-file-path "src/utils/validator.rs" \
  --test-command "cargo tarpaulin --out xml" \
  --desired-coverage 70 \
  --max-iterations 10 \
  --code-coverage-report-path "cobertura.xml" \
  --coverage-type "cobertura"
