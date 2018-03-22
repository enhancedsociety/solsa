#! /bin/bash

set -e

## use yq for .travis.yml parsing https://github.com/mikefarah/yq

## PREPARE ENVIRONMENT ##
echo "[$(date +%H:%M:%S)] Read build configuration (.travis.yml)"

ENV=""
for var in $(./yq read .travis.yml env)
do
    if [[ $var == *: ]]; then
        ENV_K="${var%:}"
    else
        ENV_FLAGS+=" -e ${ENV_K}=${var}"
    fi
done

## CONFIGURE STUFF FOR CONTAINER

JOBID=123
CURR_DIR=$(basename pwd)
CONTAINER_CODE_DIR=/root/${CURR_DIR}
CONTAINER_WORK_DIR=/tmp/${CURR_DIR}
CONTAINER_NAME=fakeci-${JOBID}
if [[ $1 == "-d" ]]; then
    CONTAINER_IMAGE=docker:dind
else
    CONTAINER_IMAGE=ubuntu:xenial
fi

## https://www.youtube.com/watch?v=ifwc5xgI3QM !

## XXX docker pull ${CONTAINER_IMAGE}
echo "[$(date +%H:%M:%S)] Preparing build machine"


if [[ ${CONTAINER_IMAGE} = ubuntu:* ]]; then
    docker run -d --name ${CONTAINER_NAME} ${ENV_FLAGS} -v "$(pwd)":"${CONTAINER_CODE_DIR}":ro ${CONTAINER_IMAGE} /sbin/init
    docker exec -t ${CONTAINER_NAME} bash -c "apt update" #> /dev/null
    docker exec -t ${CONTAINER_NAME} bash -c "apt install -y software-properties-common" #> /dev/null
elif [[ ${CONTAINER_IMAGE} = docker:*dind* ]]; then
    docker run --privileged -d --name ${CONTAINER_NAME} ${ENV_FLAGS} -v "$(pwd)":"${CONTAINER_CODE_DIR}":ro ${CONTAINER_IMAGE}
fi

test "$(docker inspect -f '{{.State.Running}}' ${CONTAINER_NAME})" == 'true'


trap cleanup EXIT

cleanup() {
  docker rm -f ${CONTAINER_NAME}
}

# Copy the code in case so we can build in source
docker exec -t ${CONTAINER_NAME} cp -ar "${CONTAINER_CODE_DIR}" "${CONTAINER_WORK_DIR}"

## BUILD SCRIPTS ##
echo "[$(date +%H:%M:%S)] Running build"

STAGES="before_install install before_script script"
BUILD_GOING=true
for stage in ${STAGES}
do
    echo "STAGE: ${stage}"
    IFS=$'\n'
    for var in $(./yq read .travis.yml "${stage}")
    do
        SCRIPT="${var#- }"
        if [[ ${BUILD_GOING} == true && "${SCRIPT}" != null ]]; then
            echo "> ${SCRIPT}"
            docker exec -t ${CONTAINER_NAME} sh -c "cd ${CONTAINER_WORK_DIR} && ${SCRIPT}"
            if (( $? != 0 )) ; then
                BUILD_GOING=false
            fi
        fi
        echo ""
    done
    unset IFS
done
## CLEANUP

if ${BUILD_GOING}; then
    echo "[$(date +%H:%M:%S)] Build success"
else
    echo "[$(date +%H:%M:%S)] Build failed"
fi
