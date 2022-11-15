#!/bin/bash

#echo "${GITHUB_JSON}"

function from_json() {
	JSON=$1
	PATH=$2

	TEMP=$(echo "${JSON}" | /usr/bin/jq -r "${PATH}")

	echo "${TEMP}"
}

function var_from_json() {
	VARNAME=$1
	JSON=$2
	PATH=$3

	echo "VARNAME: >${VARNAME}<"
	echo "JSON   : >${JSON}<"
	echo "PATH   : >${PATH}<"

#	TEMP=$(echo "${JSON}" | /usr/bin/jq -r "${PATH}")
	TEMP=$(from_json "${JSON}" ${PATH})

	echo "${VARNAME}=${TEMP}" >> $GITHUB_ENV

}

# PROJECT
PROJECT=$(from_json "${GITHUB_JSON}" .event.inputs.project)
echo "PROJECT=${PROJECT}" >> $GITHUB_ENV
#var_from_json PROJECT "${GITHUB_JSON}" .event.inputs.project

# DATE
DATE=$(from_json "${ENV_JSON}" .DATE)

# VERSION
VERSION=$(from_json "${ENV_JSON}" .VERSION)

var_from_json TEMP "${RUNNER_JSON}" .temp

# PARTS_FOLDER
TEMP_FOLDER=$(from_json "${RUNNER_JSON}" .temp)
PARTS_FOLDER="${TEMP_FOLDER}/parts_folder/"
mkdir -p ${PARTS_FOLDER}
echo "PARTS_FOLDER=${PARTS_FOLDER}" >> $GITHUB_ENV

# DATA_ARCHIVE
DATA_ARCHIVE=${PROJECT}-data-${VERSION}
echo "DATA_ARCHIVE=${DATA_ARCHIVE}" >> $GITHUB_ENV

# S3_ARCHIVE_BUCKET
S3_ARCHIVE_BUCKET="artifacts.${PROJECT}.omnimad.net"
echo "S3_ARCHIVE_BUCKET=${S3_ARCHIVE_BUCKET}" >> $GITHUB_ENV

# S3_ARCHIVE_FOLDER
S3_ARCHIVE_FOLDER=${DATE}/${VERSION}
echo "S3_ARCHIVE_FOLDER=${S3_ARCHIVE_FOLDER}" >> $GITHUB_ENV


exit 0

#          PROJECT=${{ github.event.inputs.project }}
#          echo "PROJECT=${PROJECT}" >> $GITHUB_ENV
#          temp=${{ runner.temp }}
#          echo "TEMP=${temp}" >> $GITHUB_ENV
#          parts_folder=${{ runner.temp }}/parts_folder/
#          mkdir -p ${parts_folder}
#          echo "PARTS_FOLDER=${parts_folder}" >> $GITHUB_ENV
#          package_folder=${{ runner.temp }}/package_folder/
#          mkdir -p ${package_folder}
#          echo "PACKAGE_FOLDER=${package_folder}" >> $GITHUB_ENV
#          ######runtime_tgz=rar-rs-runtime-${{ env.VERSION }}-${{ matrix.target }}.tgz
#          data_tgz=${PROJECT}-rar-data-${{ env.VERSION }}.tgz
#          s3_archive_bucket="artifacts.${PROJECT}.omnimad.net"
#          s3_archive_folder=${{ env.DATE }}/${{ env.VERSION }}
#          echo "RUNTIME_TGZ=${runtime_tgz}" >> $GITHUB_ENV
#          echo "DATA_TGZ=${data_tgz}" >> $GITHUB_ENV
#          echo "S3_ARCHIVE_FOLDER=${s3_archive_folder}" >> $GITHUB_ENV
#          echo "S3_ARCHIVE_BUCKET=${s3_archive_bucket}" >> $GITHUB_ENV
#          build_number=$(cat build_number.txt)
#          echo "BUILD_NUMBER=${build_number}" >> $GITHUB_ENV
#          echo "ARCHIVE=${{ runner.temp }}/${data_tgz}" >> $GITHUB_ENV

