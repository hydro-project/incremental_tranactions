#/bin/bash

THIS_ABS_DIR=$(cd $(dirname $0) && pwd)

SQL_COMPILER="${THIS_ABS_DIR}/../../sql-to-dbsp-compiler/SQL-compiler/sql-to-dbsp"

# TODO: Maybe convert this into a build.rs file
# People example from Feldera
${SQL_COMPILER} "${THIS_ABS_DIR}/sql/people_example.sql" --handles -o "${THIS_ABS_DIR}/src/people_example_sql.rs"
${SQL_COMPILER} "${THIS_ABS_DIR}/sql/people_example.sql" -i --handles -o "${THIS_ABS_DIR}/src/people_example_sql_incremental.rs"