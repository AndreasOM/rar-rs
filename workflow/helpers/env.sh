#!/bin/bash

#echo "${GITHUB_JSON}"
echo "PATH (bash): ${PATH}"

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
/usr/bin/mkdir -p ${PARTS_FOLDER}
echo "PARTS_FOLDER=${PARTS_FOLDER}" >> $GITHUB_ENV

# DATA_FOLDER
DATA_FOLDER="data"
echo "DATA_FOLDER=${DATA_FOLDER}" >> $GITHUB_ENV

# OMAR_FOLDER
OMAR_FOLDER="${PARTS_FOLDER}/omar/"
/usr/bin/mkdir -p ${OMAR_FOLDER}
echo "OMAR_FOLDER=${OMAR_FOLDER}" >> $GITHUB_ENV

# ARCHIVE_FOLDER
ARCHIVE_FOLDER="${PARTS_FOLDER}"
/usr/bin/mkdir -p ${ARCHIVE_FOLDER}
echo "ARCHIVE_FOLDER=${ARCHIVE_FOLDER}" >> $GITHUB_ENV

# OMAR_ARCHIVE_PREFIX
OMAR_ARCHIVE_PREFIX="${PROJECT}-"
echo "OMAR_ARCHIVE_PREFIX=${OMAR_ARCHIVE_PREFIX}" >> $GITHUB_ENV

# OMAR_ARCHIVE_SUFFIX
OMAR_ARCHIVE_SUFFIX="-${VERSION}.tgz"
echo "OMAR_ARCHIVE_SUFFIX=${OMAR_ARCHIVE_SUFFIX}" >> $GITHUB_ENV

# DATA_ARCHIVES
DATA_ARCHIVES=("base") # ("data" "premium" "dlc1" "etc")
echo "DATA_ARCHIVES=${DATA_ARCHIVES}" >> $GITHUB_ENV

# S3_ARCHIVE_BUCKET
S3_ARCHIVE_BUCKET="artifacts.${PROJECT}.omnimad.net"
echo "S3_ARCHIVE_BUCKET=${S3_ARCHIVE_BUCKET}" >> $GITHUB_ENV

# S3_ARCHIVE_FOLDER
S3_ARCHIVE_FOLDER=${DATE}/${VERSION}
echo "S3_ARCHIVE_FOLDER=${S3_ARCHIVE_FOLDER}" >> $GITHUB_ENV


# LATEST_FOLDER
LATEST_FOLDER="${TEMP_FOLDER}/latest/"
/usr/bin/mkdir -p ${LATEST_FOLDER}
echo "LATEST_FOLDER=${LATEST_FOLDER}" >> $GITHUB_ENV


# APP_VERSION
# :TODO: remove hardcoded rar-rs
APP_VERSION=$(grep version rar-rs/Cargo.toml|cut -d"\"" -f2|head -n 1)
echo "APP_VERSION=${APP_VERSION}" >> $GITHUB_ENV

# BUILD_NUMBER
BUILD_NUMBER=$(cat build_number.txt)
echo "BUILD_NUMBER=${BUILD_NUMBER}" >> $GITHUB_ENV

# PACKAGE_FOLDER
PACKAGE_FOLDER=${TEMP_FOLDER}/package_folder/
/usr/bin/mkdir -p ${PACKAGE_FOLDER}
echo "PACKAGE_FOLDER=${PACKAGE_FOLDER}" >> $GITHUB_ENV

# APP_NAME
APP_NAME="rar-rs"
echo "APP_NAME=${APP_NAME}" >> $GITHUB_ENV

# MACOS_APP_ARCHIVE
MACOS_APP_ARCHIVE="${PROJECT}-apple-darwin-FAT-apple-darwin-${VERSION}.app.tgz"
echo "MACOS_APP_ARCHIVE=${MACOS_APP_ARCHIVE}" >> $GITHUB_ENV

exit 0

### .app
###          PROJECT=${{ github.event.inputs.project }}
###          echo "PROJECT=${PROJECT}" >> $GITHUB_ENV
###          echo "Preparing variables"
###          temp=${{ runner.temp }}
###          echo "TEMP=${temp}" >> $GITHUB_ENV
###          s3_archive_bucket="artifacts.${PROJECT}.omnimad.net"
###          s3_archive_folder=${{ env.DATE }}/${{ env.VERSION }}
###          echo "S3_ARCHIVE_FOLDER=${s3_archive_folder}" >> $GITHUB_ENV
###          echo "S3_ARCHIVE_BUCKET=${s3_archive_bucket}" >> $GITHUB_ENV
###          data_tgz=${PROJECT}-rar-data-${{ env.VERSION }}.tgz
###          echo "DATA_TGZ=${data_tgz}" >> $GITHUB_ENV          
###          parts_folder=${temp}/parts_folder
###          mkdir -p ${parts_folder}
###          echo "PARTS_FOLDER=${parts_folder}" >> $GITHUB_ENV
###          package_folder=${temp}/package_folder/
###          mkdir -p ${package_folder}
###          echo "PACKAGE_FOLDER=${package_folder}" >> $GITHUB_ENV
###          echo "APP_NAME=rar-rs" >> $GITHUB_ENV
###          build_number=$(cat build_number.txt)
###          echo "BUILD_NUMBER=${build_number}" >> $GITHUB_ENV
###          app_version=$(grep version rar-rs/Cargo.toml|cut -d"\"" -f2|head -n 1)
###          echo "APP_VERSION=${app_version}" >> $GITHUB_ENV
###          app_tgz=${PROJECT}-apple-darwin-FAT-apple-darwin-.app.tgz
###          echo "ARCHIVE=${temp}/${app_tgz}" >> $GITHUB_ENV


### data
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

