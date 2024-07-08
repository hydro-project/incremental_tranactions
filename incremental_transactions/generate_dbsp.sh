#/bin/bash

# TODO: Maybe convert this into a build.rs file

THIS_ABS_DIR=$(cd $(dirname $0) && pwd)

SQL_COMPILER="${THIS_ABS_DIR}/../../sql-to-dbsp-compiler/SQL-compiler/sql-to-dbsp"

# Usage: compile <views file> <output file> [do_incremental] [draw_graph]
compile() {
    local SCHEMA="${THIS_ABS_DIR}/sql/schema.sql"
    local VIEWS_FILE="${1}"
    local OUTPUT="${2}"
    local DO_INCREMENTAL="-i"

    # Check if parameter is set and 1
    if [ -z "${3}" ] || [ "${3}" == "0" ] ; then
        local DO_INCREMENTAL=""
    fi

    # Concatenate the schema and payment files on the fly and hand to the compiler
    args="--alltables --handles ${DO_INCREMENTAL}"
    ${SQL_COMPILER} <(cat "${SCHEMA}" "${VIEWS_FILE}") ${args} -o ${OUTPUT}
    if [ -z "${4}" ] || [ "${4}" == "1" ] ; then
        ${SQL_COMPILER} <(cat "${SCHEMA}" "${VIEWS_FILE}") ${args} -png -o ${OUTPUT}.png
    fi

    # Create a brief description of the returned handles
    ## Input handles: CREATE TABLE <name> ...
    ## Adding "in_" prefix to input handles
    input_handles=$(cat "${SCHEMA}" "${VIEWS_FILE}" | grep -oP "(?<!(--))(?<=CREATE TABLE )\w+" | sed 's/^/in_/')
    ## Output handles: CREATE VIEW <name> ...
    output_handles=$(cat "${SCHEMA}" "${VIEWS_FILE}" | grep -oP "(?<!(--))(?<=CREATE VIEW )\w+" | sed 's/^/out_/')

    ## Format as rust tuple with named elements, with prefix in_ and out_
    rust_code="(`echo ${input_handles} | sed 's/ /, /g'`, `echo ${output_handles} | sed 's/ /, /g'`)"
    echo "${rust_code}" > "${OUTPUT}.handles.txt"
}

# XXX:
# Specify your SQL file and rust output file here,
# whether the circuit should be incremental and you want to draw the graph
compile "${THIS_ABS_DIR}/sql/... .sql" "${THIS_ABS_DIR}/src/... .rs" [incremental: 0/1] [draw_graph: 0/1]