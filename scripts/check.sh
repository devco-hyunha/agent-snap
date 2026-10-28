#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
mkdir -p logs
LOG="logs/check-$(date +%Y%m%d-%H%M%S).log"
echo "로그: $LOG"
: > "$LOG"

echo "[1/4] cargo fmt --check" | tee -a "$LOG"
cargo fmt --all -- --check 2>&1 | tee -a "$LOG"

echo "[2/4] cargo clippy" | tee -a "$LOG"
cargo clippy --all-targets -- -D warnings 2>&1 | tee -a "$LOG"

echo "[3/4] cargo test" | tee -a "$LOG"
cargo test --all -- --nocapture 2>&1 | tee -a "$LOG"

echo "[4/4] 줄 수 검사 (500줄 초과 금지, 테스트 모듈 제외)" | tee -a "$LOG"
fail=0
while IFS= read -r f; do
  n=$(awk '/^[[:space:]]*#\[cfg\(test\)\]/ { print NR - 1; found = 1; exit } END { if (!found) print NR }' "$f")
  if [ "$n" -gt 500 ]; then
    echo "초과: $f ($n줄)" | tee -a "$LOG"
    fail=1
  fi
done < <(find crates -name '*.rs' -not -path '*/tests/*' -not -path '*/target/*')
if [ "$fail" -ne 0 ]; then
  echo "줄 수 검사 실패" | tee -a "$LOG"
  exit 1
fi

echo "검사 통과" | tee -a "$LOG"
echo "로그: $LOG"
