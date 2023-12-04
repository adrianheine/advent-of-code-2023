#!/bin/sh
set -e

run_test() {
  DAY=$1
  PART=$2
  TEST=${DAY}_part${PART}
  set +e
  RESULT=$(cargo -q run --bin $TEST < $i 2>/dev/null)
  test $? != 0 && return 1
  set -e
  if [ -f results/$TEST ]
  then
      EXPECTED=$(cat results/$TEST)
      echo -n "$TEST: $EXPECTED = $RESULT"
      test "$EXPECTED" = "$RESULT" && echo " OK" || echo " FAIL"
  else
      printf "%s: %s correct? " "$TEST" "$RESULT"
      read -r REPLY
      test "$REPLY" = "y" -o "$REPLY" = "" && echo "  (ok, writing to results file)" && mkdir -p results && echo "$RESULT" > results/$TEST
  fi
}

cargo test

for i in $(ls input/* -r)
do
  DAY=$(basename $i)
  run_test $DAY 1
  run_test $DAY 2
done
