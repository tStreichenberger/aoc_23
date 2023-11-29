# sets up a day
# 1. download input
# 2. create simple day rust file
# 3. add mod to day.rs



CURRENT_DAY=$(date +%d)

INPUT_FILE_PATH=inputs/day${CURRENT_DAY}.txt


if [ ! -f $INPUT_FILE_PATH ]; then
    # you can download this cli tool with `cargo install aoc-cli`
    aoc download --day $CURRENT_DAY --input-only --input-file ${INPUT_FILE_PATH}
else
    echo "Day ${CURRENT_DAY} already downlaoded. Skipping download"
fi


DAY_MOD_PATH="src/days.rs"
CURRENT_DAY_MOD_PATH="src/days/day${CURRENT_DAY}.rs"

if [ -f $CURRENT_DAY_MOD_PATH ]; then
    echo "$CURRENT_DAY_MOD_PATH already exists. Skipping generation"
    exit 0
fi


# add mod to days.rs
echo -e "pub mod day${CURRENT_DAY};\n$(cat $DAY_MOD_PATH)" > $DAY_MOD_PATH



# create template .rs file
cat <<EOT >> $CURRENT_DAY_MOD_PATH
use crate::prelude::*;

pub struct Day${CURRENT_DAY};
impl Day for Day${CURRENT_DAY} {}

EOT