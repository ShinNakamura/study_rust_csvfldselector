#! /bin/bash
unalias -a

test -d ./sample || mkdir ./sample
<./tests/members.csv cargo run -- id name >./sample/expect-id-name.csv
diff ./tests/expect-id-name.csv ./sample/expect-id-name.csv
