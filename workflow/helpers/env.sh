#!/bin/bash

#echo "${GITHUB_JSON}"

function var_from_json() {
	VARNAME=$1
	JSON=$2
	PATH=$3

	echo "VARNAME: >${VARNAME}<"
	echo "JSON   : >${JSON}<"
	echo "PATH   : >${PATH}<"

	TEMP=$(echo "${JSON}" | /usr/bin/jq -r "${PATH}")

	echo "${VARNAME}=${TEMP}" >> $GITHUB_ENV

}

jq --version
/usr/bin/jq --version

PROJECT=$(echo "${GITHUB_JSON}" | jq -r .event.inputs.project)
echo "PROJECT=${PROJECT}"

var_from_json PROJECT "${GITHUB_JSON}" .event.inputs.project
var_from_json TEMP "${RUNNER_JSON}" .runner.temp

#PROJECT=$(echo "${GITHUB_JSON}" | jq -r .event.inputs.project)
#echo "PROJECT=${PROJECT}" >> $GITHUB_ENV


#PROJECT=${{ github.event.inputs.project }}
#echo "PROJECT=${PROJECT}" >> $GITHUB_ENV

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

