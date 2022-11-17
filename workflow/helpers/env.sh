#!/bin/bash

#echo "${GITHUB_JSON}"
echo "PATH (bash): ${PATH}"

function from_json() {
	json=$1
	varpath=$2

	temp=$(echo "${json}" | jq -r "${varpath}")

	echo "${temp}"
}

function var_from_json() {
	varname=$1
	json=$2
	varpath=$3

	#echo "varname: >${varname}<"
	#echo "json   : >${json}<"
	#echo "varpath   : >${varpath}<"

#	temp=$(echo "${JSON}" | /usr/bin/jq -r "${PATH}")
	temp=$(from_json "${json}" ${varpath})

	echo "${varname}=${temp}" >> $GITHUB_ENV

}

# PROJECT
PROJECT=$(from_json "${GITHUB_JSON}" .event.inputs.project)
echo "PROJECT=${PROJECT}" >> $GITHUB_ENV
#var_from_json PROJECT "${GITHUB_JSON}" .event.inputs.project

# DATE
DATE=$(from_json "${ENV_JSON}" .DATE)

# VERSION
VERSION=$(from_json "${ENV_JSON}" .VERSION)

# TEMP_FOLDER
TEMP_FOLDER=$(from_json "${RUNNER_JSON}" .temp)
echo "TEMP_FOLDER=${TEMP_FOLDER}" >> $GITHUB_ENV

# LATEST_FOLDER
LATEST_FOLDER="${TEMP_FOLDER}/latest/"
mkdir -p ${LATEST_FOLDER}
echo "LATEST_FOLDER=${LATEST_FOLDER}" >> $GITHUB_ENV

# S3_ARCHIVE_BUCKET
S3_ARCHIVE_BUCKET="artifacts.${PROJECT}.omnimad.net"
echo "S3_ARCHIVE_BUCKET=${S3_ARCHIVE_BUCKET}" >> $GITHUB_ENV

# S3_ARCHIVE_FOLDER
S3_ARCHIVE_FOLDER=${DATE}/${VERSION}
echo "S3_ARCHIVE_FOLDER=${S3_ARCHIVE_FOLDER}" >> $GITHUB_ENV

# USE_LATEST_DATA
USE_LATEST_DATA=$(from_json "${GITHUB_JSON}" .event.inputs.use_latest_data)

# USE_LATEST_APP
USE_LATEST_APP=$(from_json "${GITHUB_JSON}" .event.inputs.use_latest_app)

if [[ "x${USE_LATEST_DATA}" == "xtrue" || "x${USE_LATEST_APP}" == "xtrue" ]]
then
	src="s3://${S3_ARCHIVE_BUCKET}/latest/"
	dest="${LATEST_FOLDER}"
	echo "Syncing latest from ${src} to ${dest}"
	aws s3 sync ${src} ${dest}
	ls -lsa ${dest}
else
	echo "Not syncing latest"
fi

DATA_DATE=${DATE}
DATA_VERSION=${VERSION}

if [[ "x${USE_LATEST_DATA}" == "xtrue" ]]
then
	echo "Using latest data"
	DATA_DATE=$(cat ${LATEST_FOLDER}/latest_data_date.txt)
	DATA_VERSION=$(cat ${LATEST_FOLDER}/latest_data_version.txt)
else
	echo "Not using latest data"
#	DATA_DATE=${DATE}
#	DATA_VERSION=${VERSION}
fi

echo "DATA_DATE=${DATA_DATE}"
echo "DATA_VERSION=${DATA_VERSION}"

echo "DATA_DATE=${DATA_DATE}" >> $GITHUB_ENV
echo "DATA_VERSION=${DATA_VERSION}" >> $GITHUB_ENV


APP_DATE=${DATE}
APP_VERSION=${VERSION}

if [[ "x${USE_LATEST_APP}" == "xtrue" ]]
then
	echo "Using latest app"
	APP_DATE=$(cat ${LATEST_FOLDER}/latest_app_apple-darwin_date.txt)
	APP_VERSION=$(cat ${LATEST_FOLDER}/latest_app_apple-darwin_version.txt)
else
	echo "Not using latest app"
#	DATA_DATE=${DATE}
#	DATA_VERSION=${VERSION}
fi

echo "APP_DATE=${DATA_DATE}"
echo "APP_VERSION=${DATA_VERSION}"

echo "APP_DATE=${APP_DATE}" >> $GITHUB_ENV
echo "APP_VERSION=${APP_VERSION}" >> $GITHUB_ENV

# S3_DATA_ARCHIVE_FOLDER
S3_DATA_ARCHIVE_FOLDER="${DATA_DATE}/${DATA_VERSION}"
echo "S3_DATA_ARCHIVE_FOLDER=${S3_DATA_ARCHIVE_FOLDER}" >> $GITHUB_ENV

# S3_APP_ARCHIVE_FOLDER
S3_APP_ARCHIVE_FOLDER="${APP_DATE}/${APP_VERSION}"
echo "S3_APP_ARCHIVE_FOLDER=${S3_APP_ARCHIVE_FOLDER}" >> $GITHUB_ENV

# TEMP
var_from_json TEMP "${RUNNER_JSON}" .temp


# PARTS_FOLDER
PARTS_FOLDER="${TEMP_FOLDER}/parts_folder/"
mkdir -p ${PARTS_FOLDER}
echo "PARTS_FOLDER=${PARTS_FOLDER}" >> $GITHUB_ENV

# DATA_FOLDER
DATA_FOLDER="data"
echo "DATA_FOLDER=${DATA_FOLDER}" >> $GITHUB_ENV

# OMAR_FOLDER
OMAR_FOLDER="${PARTS_FOLDER}/omar/"
mkdir -p ${OMAR_FOLDER}
echo "OMAR_FOLDER=${OMAR_FOLDER}" >> $GITHUB_ENV

# ARCHIVE_FOLDER
ARCHIVE_FOLDER="${PARTS_FOLDER}"
mkdir -p ${ARCHIVE_FOLDER}
echo "ARCHIVE_FOLDER=${ARCHIVE_FOLDER}" >> $GITHUB_ENV

# OMAR_ARCHIVE_PREFIX
OMAR_ARCHIVE_PREFIX="${PROJECT}-"
echo "OMAR_ARCHIVE_PREFIX=${OMAR_ARCHIVE_PREFIX}" >> $GITHUB_ENV

# OMAR_ARCHIVE_SUFFIX
OMAR_ARCHIVE_SUFFIX="-${DATA_VERSION}.tgz"
echo "OMAR_ARCHIVE_SUFFIX=${OMAR_ARCHIVE_SUFFIX}" >> $GITHUB_ENV

# DATA_ARCHIVES
DATA_ARCHIVES=("base") # ("data" "premium" "dlc1" "etc")
echo "DATA_ARCHIVES=${DATA_ARCHIVES}" >> $GITHUB_ENV


# S3_CACHE_TARGET
S3_CACHE_TARGET="${S3_ARCHIVE_BUCKET}/cache/target"
echo "S3_CACHE_TARGET=${S3_CACHE_TARGET}" >> $GITHUB_ENV

# TARGET_FOLDER
TARGET_FOLDER="${PROJECT}/target"
echo "TARGET_FOLDER=${TARGET_FOLDER}" >> $GITHUB_ENV


# APP_VERSION
# :TODO: remove hardcoded rar-rs
APP_VERSION=$(grep version rar-rs/Cargo.toml|cut -d"\"" -f2|head -n 1)
echo "APP_VERSION=${APP_VERSION}" >> $GITHUB_ENV

# BUILD_NUMBER
BUILD_NUMBER=$(cat build_number.txt)
echo "BUILD_NUMBER=${BUILD_NUMBER}" >> $GITHUB_ENV

# PACKAGE_FOLDER
PACKAGE_FOLDER=${TEMP_FOLDER}/package_folder/
mkdir -p ${PACKAGE_FOLDER}
echo "PACKAGE_FOLDER=${PACKAGE_FOLDER}" >> $GITHUB_ENV

# APP_NAME
APP_NAME="rar-rs"
echo "APP_NAME=${APP_NAME}" >> $GITHUB_ENV

# MACOS_APP_ARCHIVE_FOLDER
MACOS_APP_ARCHIVE_FOLDER="${TEMP_FOLDER}"
echo "MACOS_APP_ARCHIVE_FOLDER=${MACOS_APP_ARCHIVE_FOLDER}" >> $GITHUB_ENV

# MACOS_APP_ARCHIVE
MACOS_APP_ARCHIVE="${PROJECT}-apple-darwin-FAT-apple-darwin-${VERSION}.app.tgz"
echo "MACOS_APP_ARCHIVE=${MACOS_APP_ARCHIVE}" >> $GITHUB_ENV

# MACOS_APP_ARCHIVE_PATH
MACOS_APP_ARCHIVE_PATH="${MACOS_APP_ARCHIVE_FOLDER}/${MACOS_APP_ARCHIVE}"
echo "MACOS_APP_ARCHIVE_PATH=${MACOS_APP_ARCHIVE_PATH}" >> $GITHUB_ENV

exit 0

### itch.io
#          temp=${{ runner.temp }}
#          echo "TEMP=${temp}" >> $GITHUB_ENV
#          parts_folder=${temp}/parts_folder/
#          mkdir -p ${parts_folder}
#          echo "PARTS_FOLDER=${parts_folder}" >> $GITHUB_ENV
#          package_folder=${temp}/package_folder/
#          mkdir -p ${package_folder}
#          echo "PACKAGE_FOLDER=${package_folder}" >> $GITHUB_ENV
#          runtime_tgz=fiiish-rs-runtime-${{ env.VERSION }}-${{ matrix.target }}.tgz
#          fiiish_data_tgz=fiiish-rs-fiiish-data-${{ env.VERSION }}.tgz
#          dummy_data_tgz=fiiish-rs-dummy-data-${{ env.VERSION }}.tgz
#          app_tgz=fiiish-rs-${{ matrix.platform }}-${{ matrix.target }}-${{ matrix.package_type }}.tgz
#          s3_archive_folder=${{ env.DATE }}/${{ env.VERSION }}
#          echo "RUNTIME_TGZ=${runtime_tgz}" >> $GITHUB_ENV
#          echo "FIIISH_DATA_TGZ=${fiiish_data_tgz}" >> $GITHUB_ENV
#          echo "DUMMY_DATA_TGZ=${dummy_data_tgz}" >> $GITHUB_ENV
#          echo "APP_TGZ=${app_tgz}" >> $GITHUB_ENV
#          echo "S3_ARCHIVE_FOLDER=${s3_archive_folder}" >> $GITHUB_ENV
#          # build number will be taken from the parts
#          # build_number=$(cat build_number.txt)
#          # echo "BUILD_NUMBER=${build_number}" >> $GITHUB_ENV
#          echo "ARCHIVE=${temp}/${app_tgz}" >> $GITHUB_ENV

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

