#/bin/bash

THIS_ABS_DIR=$(cd $(dirname $0) && pwd)

SQL_COMPILER="${THIS_ABS_DIR}/../../sql-to-dbsp-compiler/SQL-compiler/sql-to-dbsp"
# ${SQL_COMPILER} --help
# exit 0

# Remove the allocator code from the output
# Note: This removes only the exact lines, so if the format changes, this will not work
# Usage: remove_allocator <file>
remove_allocator() {
    local FILE="${1}"

    sed -i -e '/#\[cfg(not(target_env = "msvc"))\]$/d' \
        -e '/\[global_allocator\]$/d' \
        -e '/static ALLOC: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;/d' \
        -e '/#\[allow(non_upper_case_globals)\]/d' \
        -e '/#\[export_name = "malloc_conf"\]/d' \
        -e '/pub static malloc_conf: &\[u8\] = b"prof:true,prof_active:true,lg_prof_sample:19\\0";/d' "${FILE}"
}

# Usage: compile <views file> <output file> [do_incremental] [draw_graph]
compile() {
    local SCHEMA="${THIS_ABS_DIR}/sql/schema.sql"
    local VIEWS_FILE="${1}"
    local OUTPUT="${2}"
    local DO_INCREMENTAL="-i"

    local OUTPUT_DIR=$(dirname "${OUTPUT}")
    local BASE_FILE=$(basename "${OUTPUT}")

    # Check if parameter is set and 1
    if [ -z "${3}" ] || [ "${3}" == "0" ] ; then
        local DO_INCREMENTAL=""
    fi

    # [BIG_QUERY, ORACLE, MYSQL, MYSQL_ANSI, SQL_SERVER, JAVA]
    # big_query oracle mysql
    # Concatenate the schema and payment files on the fly and hand to the compiler
    args="--alltables --handles ${DO_INCREMENTAL}"
    ${SQL_COMPILER} <(cat "${SCHEMA}" "${VIEWS_FILE}") ${args} -o ${OUTPUT}
    # Remove the allocator code from the output
    remove_allocator "${OUTPUT}"

    # Draw the graph if the parameter is set and 1
    if [ -z "${4}" ] || [ "${4}" == "1" ] ; then
        mkdir -p "${OUTPUT_DIR}/../graphs/"
        ${SQL_COMPILER} <(cat "${SCHEMA}" "${VIEWS_FILE}") ${args} -png -o "${OUTPUT_DIR}/../graphs/${BASE_FILE}.png"
    fi

    # Create a brief description of the returned handles
    ## Input handles: CREATE TABLE <name> ...
    ## Adding "in_" prefix to input handles
    input_handles=$(cat "${SCHEMA}" "${VIEWS_FILE}" | grep -oP "(?<!(--))(?<=CREATE TABLE )\w+" | sed 's/^/in_/')
    ## Output handles: CREATE VIEW <name> ...
    output_handles=$(cat "${SCHEMA}" "${VIEWS_FILE}" | grep -oP "(?<!(--))(?<=CREATE VIEW )\w+" | sed 's/^/out_/')

    ## Format as rust tuple with named elements, with prefix in_ and out_
    mkdir -p "${OUTPUT_DIR}/../handles/"
    rust_code="(`echo ${input_handles} | sed 's/ /, /g'`, `echo ${output_handles} | sed 's/ /, /g'`)"
    echo "${rust_code}" > "${OUTPUT_DIR}/../handles/${BASE_FILE}.handles.txt"
}

# compile "${THIS_ABS_DIR}/sql/payment.sql" "${THIS_ABS_DIR}/src/payment_sql.rs" 0 1
# compile "${THIS_ABS_DIR}/sql/payment.sql" "${THIS_ABS_DIR}/src/payment_sql_incremental.rs" 1 1
compile "${THIS_ABS_DIR}/sql/byname.sql" "${THIS_ABS_DIR}/src/byname_sql.rs" 0 1
compile "${THIS_ABS_DIR}/sql/byname.sql" "${THIS_ABS_DIR}/src/byname_sql_incremental.rs" 1 1
#compile "${THIS_ABS_DIR}/sql/warehouse_ytd.sql" "${THIS_ABS_DIR}/src/warehouse_ytd_sql.rs" 0 1
#compile "${THIS_ABS_DIR}/sql/warehouse_ytd.sql" "${THIS_ABS_DIR}/src/warehouse_ytd_sql_incremental.rs" 1 1
