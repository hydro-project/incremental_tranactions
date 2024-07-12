#/bin/bash

THIS_ABS_DIR=$(cd $(dirname $0) && pwd)

SQL_COMPILER="${THIS_ABS_DIR}/../../sql-to-dbsp-compiler/SQL-compiler/sql-to-dbsp"

compile() {
    local VIEWS_FILE="${1}"
    local OUTPUT="${2}"
    local DO_INCREMENTAL="-i"

    # Check if parameter is set and 1
    if [ -z "${3}" ] || [ "${3}" == "0" ] ; then
        local DO_INCREMENTAL=""
    fi

    args="--alltables --handles ${DO_INCREMENTAL}"
    ${SQL_COMPILER} "${VIEWS_FILE}" ${args} -o ${OUTPUT}
    # TODO: Remove the allocator code from the output

    # Draw the graph if the parameter is set and 1
    if [ -z "${4}" ] || [ "${4}" == "1" ] ; then
        ${SQL_COMPILER} "${VIEWS_FILE}" ${args} -png -o ${OUTPUT}.png
    fi

    # Create a brief description of the returned handles
    ## Input handles: CREATE TABLE <name> ...
    ## Adding "in_" prefix to input handles
    input_handles=$(cat ${VIEWS_FILE} | grep -oP "(?<!(--))(?<=CREATE TABLE )\w+" | sed 's/^/in_/')
    ## Output handles: CREATE VIEW <name> ...
    output_handles=$(cat ${VIEWS_FILE} | grep -oP "(?<!(--))(?<=CREATE VIEW )\w+" | sed 's/^/out_/')

    ## Format as rust tuple with named elements, with prefix in_ and out_
    rust_code="(`echo ${input_handles} | sed 's/ /, /g'`, `echo ${output_handles} | sed 's/ /, /g'`)"
    echo "${rust_code}" > "${OUTPUT}.handles.txt"
}

# TODO: Maybe convert this into a build.rs file
# People example from Feldera
compile "${THIS_ABS_DIR}/sql/people_example.sql" "${THIS_ABS_DIR}/src/people_example_sql.rs" 0 1
compile "${THIS_ABS_DIR}/sql/people_example.sql" "${THIS_ABS_DIR}/src/people_example_sql_incremental.rs" 1 1