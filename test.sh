#!/bin/sh
set -e

RED='\033[0;31m'
NC='\033[0m'

run_test() {
  DAY=$1
  PART=$2
  TEST=${DAY}_part${PART}
  set +e
  test -f src/bin/$TEST.rs || return 1
  RESULT=$(time -f "%U %M" cargo -q run --release --bin $TEST < $i)
  set -e
  if [ -f results/$TEST ]
  then
      EXPECTED=$(cat results/$TEST)
      echo -n "$TEST: $EXPECTED = $RESULT"
      test "$EXPECTED" = "$RESULT" && echo " OK" || printf " ${RED}FAIL${NC}\n"
  else
      printf "%s: %s correct? " "$TEST" "$RESULT"
      read -r REPLY
      test "$REPLY" = "y" -o "$REPLY" = "" && echo "  (ok, writing to results file)" && mkdir -p results && echo "$RESULT" > results/$TEST
  fi
}

cargo test --release

for i in $(ls input/* -r)
do
  DAY=$(basename $i)
  run_test $DAY 1
  run_test $DAY 2
done
